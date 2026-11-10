package com.oppzippy.openscq30.features.tile

import android.app.PendingIntent
import android.content.ComponentName
import android.content.Intent
import android.content.ServiceConnection
import android.graphics.drawable.Icon
import android.os.Build
import android.os.IBinder
import android.service.quicksettings.Tile
import android.service.quicksettings.TileService
import android.util.Log
import androidx.annotation.DrawableRes
import com.oppzippy.openscq30.MainActivity
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.features.soundcoredevice.service.ConnectionStatus
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceService
import com.oppzippy.openscq30.lib.bindings.OpenScq30Device
import com.oppzippy.openscq30.lib.bindings.OpenScq30Exception
import com.oppzippy.openscq30.lib.bindings.SettingIdValuePair
import com.oppzippy.openscq30.lib.bindings.translateDeviceModel
import com.oppzippy.openscq30.lib.bindings.translateValue
import com.oppzippy.openscq30.lib.wrapper.Setting
import com.oppzippy.openscq30.lib.wrapper.toValue
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.Job
import kotlinx.coroutines.MainScope
import kotlinx.coroutines.cancel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.combine
import kotlinx.coroutines.flow.filterNotNull
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.flow.flatMapLatest
import kotlinx.coroutines.flow.flowOf
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.launch
import kotlinx.coroutines.withTimeoutOrNull

/**
 * Quick Settings tile showing the current sound mode. Tapping it cycles through the ambient sound modes (respecting the
 * device's own sound mode cycle configuration when it has one), so switching between e.g. noise canceling and
 * transparency doesn't require opening the app. Long pressing the tile opens the app.
 */
@OptIn(ExperimentalCoroutinesApi::class)
class SoundModeTileService : TileService() {
    companion object {
        private const val TAG = "SoundModeTileService"
        private const val AMBIENT_SOUND_MODE = "ambientSoundMode"
        private const val SERVICE_CONNECTION_TIMEOUT_MS = 2000L

        /** Setting ids of the device's own "which modes are included when cycling" configuration */
        private val cycleSettingIds = mapOf(
            "NoiseCanceling" to "noiseCancelingModeInCycle",
            "Transparency" to "transparencyModeInCycle",
            "Normal" to "normalModeInCycle",
        )
    }

    private val scope = MainScope()
    private var listeningJob: Job? = null
    private var isBound = false
    private val deviceService = MutableStateFlow<DeviceService?>(null)

    private val serviceConnection = object : ServiceConnection {
        override fun onServiceConnected(name: ComponentName?, service: IBinder?) {
            deviceService.value = (service as? DeviceService.MyBinder)?.getService()
        }

        override fun onServiceDisconnected(name: ComponentName?) {
            deviceService.value = null
        }
    }

    override fun onCreate() {
        super.onCreate()
        // Stay bound for as long as the tile service exists rather than only while the tile is listening, since clicks
        // can be delivered while it isn't. Flags of 0 means we don't start the service. If it isn't running, we'll be
        // connected once it starts.
        isBound = bindService(Intent(this, DeviceService::class.java), serviceConnection, 0)
    }

    override fun onDestroy() {
        scope.cancel()
        if (isBound) {
            unbindService(serviceConnection)
            isBound = false
        }
        deviceService.value = null
        super.onDestroy()
    }

    override fun onStartListening() {
        super.onStartListening()
        listeningJob?.cancel()
        listeningJob = scope.launch {
            combine(DeviceService.isRunning, deviceService) { isRunning, service -> if (isRunning) service else null }
                .flatMapLatest { service -> service?.connectionStatusFlow ?: flowOf(ConnectionStatus.Disconnected) }
                .flatMapLatest { status ->
                    when (status) {
                        is ConnectionStatus.Connected -> {
                            val device = status.deviceManager.device
                            status.deviceManager.watchForChangeNotification.map { tileStateOf(device) }
                        }

                        is ConnectionStatus.Connecting -> flowOf(TileState.Connecting)

                        else -> flowOf(TileState.Disconnected)
                    }
                }
                .collect { render(it) }
        }
    }

    override fun onStopListening() {
        listeningJob?.cancel()
        listeningJob = null
        super.onStopListening()
    }

    override fun onClick() {
        super.onClick()
        val service = deviceService.value
        when {
            service != null -> cycleSoundMode(service)

            // Binding completes asynchronously, so a click can arrive just before the connection is established
            DeviceService.isRunning.value -> scope.launch {
                val connectedService = withTimeoutOrNull(SERVICE_CONNECTION_TIMEOUT_MS) {
                    deviceService.filterNotNull().first()
                }
                if (connectedService != null) cycleSoundMode(connectedService) else openApp()
            }

            else -> openApp()
        }
    }

    private fun cycleSoundMode(service: DeviceService) {
        when (val status = service.connectionStatusFlow.value) {
            is ConnectionStatus.Connected -> {
                val device = status.deviceManager.device
                val nextMode = try {
                    nextAmbientSoundMode(device)
                } catch (ex: IllegalStateException) {
                    Log.w(TAG, "device was closed", ex)
                    null
                }
                if (nextMode == null) {
                    openApp()
                    return
                }
                scope.launch {
                    try {
                        device.setSettingValues(listOf(SettingIdValuePair(AMBIENT_SOUND_MODE, nextMode.toValue())))
                    } catch (ex: OpenScq30Exception) {
                        Log.e(TAG, "error changing ambient sound mode", ex)
                    } catch (ex: IllegalStateException) {
                        Log.w(TAG, "device was closed, not changing ambient sound mode", ex)
                    }
                }
            }

            // Already busy connecting, nothing useful to do
            is ConnectionStatus.Connecting -> {}

            else -> openApp()
        }
    }

    private fun nextAmbientSoundMode(device: OpenScq30Device): String? {
        val ambientSoundMode = device.setting(AMBIENT_SOUND_MODE) as? Setting.SelectSetting ?: return null
        val options = ambientSoundMode.setting.options
        val optionsInCycle = options
            .filter { option ->
                val inCycle = cycleSettingIds[option]?.let { device.setting(it) } as? Setting.ToggleSetting
                inCycle?.value ?: true
            }
            .takeIf { it.size >= 2 }
            ?: options
        if (optionsInCycle.isEmpty()) return null
        // indexOf returning -1 (current mode excluded from the cycle) conveniently wraps to the first mode
        return optionsInCycle[(optionsInCycle.indexOf(ambientSoundMode.value) + 1) % optionsInCycle.size]
    }

    private fun tileStateOf(device: OpenScq30Device): TileState = try {
        val modelName = translateDeviceModel(device.model())
        val ambientSoundMode = device.setting(AMBIENT_SOUND_MODE) as? Setting.SelectSetting
        if (ambientSoundMode == null) {
            TileState.Connected(label = modelName, subtitle = null, icon = R.drawable.headphones_24px)
        } else {
            val subtitle = when (ambientSoundMode.value) {
                "NoiseCanceling" -> noiseCancelingDetail(device)
                "Transparency" -> device.setting("transparencyMode")?.let { translateValue(it, it.toValue()) }
                else -> null
            } ?: modelName
            TileState.Connected(
                label = translateValue(ambientSoundMode, ambientSoundMode.toValue()),
                subtitle = subtitle,
                icon = when (ambientSoundMode.value) {
                    "NoiseCanceling" -> R.drawable.noise_control_off_24px
                    "Transparency" -> R.drawable.hearing_24px
                    else -> R.drawable.headphones_24px
                },
            )
        }
    } catch (ex: IllegalStateException) {
        Log.w(TAG, "device was closed", ex)
        TileState.Disconnected
    }

    private fun noiseCancelingDetail(device: OpenScq30Device): String? {
        val mode = device.setting("noiseCancelingMode") as? Setting.SelectSetting
        val manual = device.setting("manualNoiseCanceling")
        val transportation = device.setting("transportationMode")
        return when {
            (mode == null || mode.value == "Manual") && manual is Setting.I32RangeSetting ->
                getString(R.string.level_x, manual.value)

            mode?.value == "Transportation" && transportation != null ->
                translateValue(transportation, transportation.toValue())

            mode != null -> translateValue(mode, mode.toValue())

            else -> null
        }
    }

    private fun render(state: TileState) {
        val tile = qsTile ?: return
        val (label, subtitle) = when (state) {
            is TileState.Connected -> {
                tile.state = Tile.STATE_ACTIVE
                tile.icon = Icon.createWithResource(this, state.icon)
                Pair(state.label, state.subtitle)
            }

            TileState.Connecting -> {
                tile.state = Tile.STATE_INACTIVE
                tile.icon = Icon.createWithResource(this, R.drawable.noise_control_off_24px)
                Pair(getString(R.string.sound_mode), getString(R.string.connecting))
            }

            TileState.Disconnected -> {
                tile.state = Tile.STATE_INACTIVE
                tile.icon = Icon.createWithResource(this, R.drawable.noise_control_off_24px)
                Pair(getString(R.string.sound_mode), getString(R.string.disconnected))
            }
        }
        tile.label = label
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
            tile.subtitle = subtitle
        }
        // Small (icon only) tiles have no visible text, so make sure screen readers get the details too
        tile.contentDescription = listOfNotNull(label, subtitle).joinToString(", ")
        tile.updateTile()
    }

    private fun openApp() {
        val intent = Intent(this, MainActivity::class.java).addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.UPSIDE_DOWN_CAKE) {
            startActivityAndCollapse(
                PendingIntent.getActivity(this, 0, intent, PendingIntent.FLAG_IMMUTABLE),
            )
        } else {
            @Suppress("DEPRECATION")
            startActivityAndCollapse(intent)
        }
    }

    private sealed class TileState {
        data object Disconnected : TileState()
        data object Connecting : TileState()
        data class Connected(val label: String, val subtitle: String?, @param:DrawableRes val icon: Int) : TileState()
    }
}
