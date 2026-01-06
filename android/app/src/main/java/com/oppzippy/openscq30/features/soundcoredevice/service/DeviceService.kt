package com.oppzippy.openscq30.features.soundcoredevice.service

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.os.Binder
import android.os.IBinder
import android.util.Log
import android.widget.Toast
import androidx.core.content.ContextCompat
import androidx.glance.appwidget.GlanceAppWidgetManager
import androidx.glance.appwidget.state.updateAppWidgetState
import androidx.lifecycle.LifecycleService
import androidx.lifecycle.lifecycleScope
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.features.soundcoredevice.connectionBackends
import com.oppzippy.openscq30.features.soundcoredevice.service.SoundcoreDeviceNotification.ACTION_DISCONNECT
import com.oppzippy.openscq30.features.soundcoredevice.service.SoundcoreDeviceNotification.ACTION_QUICK_PRESET
import com.oppzippy.openscq30.features.soundcoredevice.service.SoundcoreDeviceNotification.ACTION_SEND_NOTIFICATION
import com.oppzippy.openscq30.features.soundcoredevice.service.SoundcoreDeviceNotification.ACTION_SET_ANC_MODE
import com.oppzippy.openscq30.features.soundcoredevice.service.SoundcoreDeviceNotification.INTENT_EXTRA_ANC_MODE
import com.oppzippy.openscq30.features.soundcoredevice.service.SoundcoreDeviceNotification.INTENT_EXTRA_PRESET_ID
import com.oppzippy.openscq30.features.soundcoredevice.service.SoundcoreDeviceNotification.NOTIFICATION_CHANNEL_ID
import com.oppzippy.openscq30.features.soundcoredevice.service.SoundcoreDeviceNotification.NOTIFICATION_ID
import com.oppzippy.openscq30.features.statusnotification.storage.FeaturedSettingSlotDao
import com.oppzippy.openscq30.features.statusnotification.storage.QuickPresetSlotDao
import com.oppzippy.openscq30.lib.bindings.OpenScq30Exception
import com.oppzippy.openscq30.lib.bindings.OpenScq30Session
import com.oppzippy.openscq30.lib.bindings.SettingIdValuePair
import com.oppzippy.openscq30.lib.wrapper.Setting
import com.oppzippy.openscq30.lib.wrapper.toValue
import com.oppzippy.openscq30.widget.modes.ModesWidget
import dagger.hilt.android.AndroidEntryPoint
import javax.inject.Inject
import kotlin.time.Duration.Companion.milliseconds
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.FlowPreview
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.collectLatest
import kotlinx.coroutines.flow.debounce
import kotlinx.coroutines.flow.emptyFlow
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.flow.flatMapLatest
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.launch

@OptIn(FlowPreview::class, ExperimentalCoroutinesApi::class)
@AndroidEntryPoint
class DeviceService : LifecycleService() {
    companion object {
        private const val TAG = "DeviceService"

        /** Intent extra for setting mac address when launching service */
        const val MAC_ADDRESS = "com.oppzippy.openscq30.macAddress"

        fun doesNotificationExist(context: Context): Boolean {
            val notificationManager =
                context.getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
            val doesNotificationExist = notificationManager.activeNotifications.any {
                (it.notification.channelId == NOTIFICATION_CHANNEL_ID) && (it.id == NOTIFICATION_ID)
            }
            return doesNotificationExist
        }
    }

    @Inject
    lateinit var session: OpenScq30Session

    @Inject
    lateinit var notificationBuilder: NotificationBuilder

    @Inject
    lateinit var quickPresetSlotDao: QuickPresetSlotDao

    @Inject
    lateinit var featuredSettingSlotDao: FeaturedSettingSlotDao

    val connectionStatusFlow: MutableStateFlow<ConnectionStatus> =
        MutableStateFlow(ConnectionStatus.AwaitingConnection)

    private var quickPresetNames = connectionStatusFlow.flatMapLatest { connectionStatus ->
        if (connectionStatus is ConnectionStatus.Connected) {
            val device = connectionStatus.deviceManager.device
            quickPresetSlotDao.allNames(device.model())
        } else {
            emptyFlow()
        }
    }.stateIn(lifecycleScope, SharingStarted.Lazily, emptyList())
    private var featuredSettingIds = connectionStatusFlow.flatMapLatest { connectionStatus ->
        if (connectionStatus is ConnectionStatus.Connected) {
            val device = connectionStatus.deviceManager.device
            featuredSettingSlotDao.allSettingIds(device.model())
        } else {
            emptyFlow()
        }
    }.stateIn(lifecycleScope, SharingStarted.Lazily, emptyList())

    private val broadcastReceiver = object : BroadcastReceiver() {
        override fun onReceive(context: Context?, intent: Intent?) {
            when (intent?.action) {
                ACTION_DISCONNECT -> stopSelf()

                ACTION_QUICK_PRESET -> {
                    val presetIndex = intent.getIntExtra(INTENT_EXTRA_PRESET_ID, 0)
                    lifecycleScope.launch {
                        connectionStatusFlow.value.let {
                            if (it is ConnectionStatus.Connected) {
                                val device = it.deviceManager.device
                                session.quickPresetHandler().use { quickPresetHandler ->
                                    val quickPresets = quickPresetHandler.quickPresets(device)
                                    quickPresets.getOrNull(presetIndex)?.let { preset ->
                                        try {
                                            quickPresetHandler.activate(device, preset.name)
                                        } catch (ex: OpenScq30Exception) {
                                            Log.e(TAG, "error activating quick preset ${preset.name}", ex)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                ACTION_SEND_NOTIFICATION -> {
                    sendNotification()
                }
            }
        }
    }

    override fun onCreate() {
        super.onCreate()

        lifecycleScope.launch { quickPresetNames.collectLatest { sendNotification() } }
        lifecycleScope.launch { featuredSettingIds.collectLatest { sendNotification() } }

        // when we are connected to a device that becomes disconnected, update our connection status to disconnected
        lifecycleScope.launch {
            connectionStatusFlow.collectLatest { connectionStatus ->
                if (connectionStatus is ConnectionStatus.Connected) {
                    connectionStatus.deviceManager.connectionStatusFlow.collectLatest { deviceConnectionStatus ->
                        if (deviceConnectionStatus ==
                            com.oppzippy.openscq30.lib.bindings.ConnectionStatus.Disconnected
                        ) {
                            Log.d(TAG, "device disconnected")
                            connectionStatusFlow.value = ConnectionStatus.Disconnected
                        }
                    }
                }
            }
        }
        lifecycleScope.launch {
            connectionStatusFlow.first { it == ConnectionStatus.Disconnected }
            stopSelf()
        }
        lifecycleScope.launch {
            connectionStatusFlow.collectLatest {
                if (it is ConnectionStatus.Connected) {
                    it.deviceManager.watchForChangeNotification.debounce(500.milliseconds).collectLatest {
                        sendNotification()
                    }
                }
            }
        }
        lifecycleScope.launch {
            connectionStatusFlow.collectLatest { connectionStatus ->
                if (connectionStatus is ConnectionStatus.Connected) {
                    val device = connectionStatus.deviceManager.device
                    updateWidgetState(device)
                    connectionStatus.deviceManager.watchForChangeNotification.collectLatest {
                        updateWidgetState(device)
                    }
                } else {
                    updateWidgetState(null)
                }
            }
        }

        val filter = IntentFilter(ACTION_DISCONNECT).apply {
            addAction(ACTION_QUICK_PRESET)
            addAction(ACTION_SEND_NOTIFICATION)
        }
        ContextCompat.registerReceiver(
            this,
            broadcastReceiver,
            filter,
            ContextCompat.RECEIVER_NOT_EXPORTED,
        )

        createNotificationChannel()
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        super.onStartCommand(intent, flags, startId)

        val notification = buildNotification()
        startForeground(NOTIFICATION_ID, notification)

        if (intent?.action == ACTION_SET_ANC_MODE) {
            val mode = intent.getStringExtra(INTENT_EXTRA_ANC_MODE)
            if (mode != null) {
                lifecycleScope.launch {
                    connectionStatusFlow.value.let {
                        if (it is ConnectionStatus.Connected) {
                            try {
                                it.deviceManager.device.setSettingValues(
                                    listOf(SettingIdValuePair("ambientSoundMode", mode.toValue()))
                                )
                            } catch (ex: Exception) {
                                Log.e(TAG, "error setting anc mode", ex)
                            }
                        }
                    }
                }
            }
        }

        intent?.getStringExtra(MAC_ADDRESS)?.let { macAddress ->
            lifecycleScope.launch {
                try {
                    val device = session.connectWithBackends(
                        connectionBackends(applicationContext, lifecycleScope),
                        macAddress,
                    )
                    connectionStatusFlow.value = ConnectionStatus.Connected(DeviceConnectionManager(device))
                } catch (ex: OpenScq30Exception) {
                    Log.w(TAG, "error connecting to device", ex)
                    Toast.makeText(applicationContext, R.string.error_connecting, Toast.LENGTH_SHORT).show()
                    connectionStatusFlow.value = ConnectionStatus.Disconnected
                }
            }
            connectionStatusFlow.compareAndSet(
                ConnectionStatus.AwaitingConnection,
                ConnectionStatus.Connecting(macAddress),
            )
        }

        return START_REDELIVER_INTENT
    }

    override fun onDestroy() {
        super.onDestroy()
        unregisterReceiver(broadcastReceiver)
        cancelNotification()
        connectionStatusFlow.value.let {
            if (it is ConnectionStatus.Connected) {
                it.deviceManager.close()
            }
        }
    }

    private fun createNotificationChannel() {
        val channel = NotificationChannel(
            NOTIFICATION_CHANNEL_ID,
            "Device Connection Service",
            NotificationManager.IMPORTANCE_LOW,
        )
        channel.enableVibration(false)
        channel.enableLights(false)
        channel.setSound(null, null)
        val manager = getSystemService(NotificationManager::class.java)
        manager.createNotificationChannel(channel)
    }

    private fun sendNotification() {
        val notificationManager =
            getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
        notificationManager.notify(NOTIFICATION_ID, buildNotification())
    }

    private fun cancelNotification() {
        val notificationManager =
            getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
        notificationManager.cancel(NOTIFICATION_ID)
    }

    private fun buildNotification(): Notification = notificationBuilder(
        status = connectionStatusFlow.value,
        quickPresetNames = quickPresetNames.value,
        featuredSettingIds = featuredSettingIds.value,
    )

    private suspend fun updateWidgetState(device: com.oppzippy.openscq30.lib.bindings.OpenScq30Device?) {
        val context = applicationContext
        val manager = GlanceAppWidgetManager(context)
        val widget = ModesWidget()
        val glanceIds = manager.getGlanceIds(widget.javaClass)

        // Fetch last connected device if current device is null
        val lastConnectedDevice = if (device == null) {
            try {
                session.lastConnectedDevice()
            } catch (e: Exception) {
                Log.e(TAG, "Failed to fetch last connected device", e)
                null
            }
        } else {
            null
        }

        glanceIds.forEach { glanceId ->
            updateAppWidgetState(context, glanceId) { prefs ->
                if (device != null) {
                    prefs[ModesWidget.IS_CONNECTED_KEY] = true
                    val setting = device.setting("ambientSoundMode")
                    if (setting is Setting.SelectSetting) {
                        prefs[ModesWidget.IS_SUPPORTED_KEY] = true
                        prefs[ModesWidget.CURRENT_MODE_KEY] = setting.value
                        // TODO: Get the actual device name or alias. For now, using the model name.
                        // Ideally, we should fetch the alias if available.
                        prefs[ModesWidget.DEVICE_NAME_KEY] = device.model()
                    } else {
                        prefs[ModesWidget.IS_SUPPORTED_KEY] = false
                    }
                    // Clear last device info when connected
                    prefs.remove(ModesWidget.LAST_DEVICE_MAC_KEY)
                } else {
                    prefs[ModesWidget.IS_CONNECTED_KEY] = false
                    if (lastConnectedDevice != null) {
                        prefs[ModesWidget.DEVICE_NAME_KEY] = lastConnectedDevice.model
                        prefs[ModesWidget.LAST_DEVICE_MAC_KEY] = lastConnectedDevice.macAddress
                    } else {
                        prefs.remove(ModesWidget.DEVICE_NAME_KEY)
                        prefs.remove(ModesWidget.LAST_DEVICE_MAC_KEY)
                    }
                }
            }
            widget.update(context, glanceId)
        }
    }

    private val binder = MyBinder()

    override fun onBind(intent: Intent): IBinder {
        super.onBind(intent)
        return binder
    }

    inner class MyBinder : Binder() {
        fun getService(): DeviceService = this@DeviceService
    }
}
