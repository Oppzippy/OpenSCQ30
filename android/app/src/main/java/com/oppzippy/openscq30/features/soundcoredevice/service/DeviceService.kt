package com.oppzippy.openscq30.features.soundcoredevice.service

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.appwidget.AppWidgetManager
import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.os.Binder
import android.os.IBinder
import android.util.Log
import android.widget.Toast
import androidx.core.content.ContextCompat
import androidx.core.content.IntentCompat
import androidx.glance.appwidget.GlanceAppWidgetManager
import androidx.lifecycle.LifecycleService
import androidx.lifecycle.lifecycleScope
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.features.soundcoredevice.connectionBackends
import com.oppzippy.openscq30.features.statusnotification.storage.FeaturedSettingSlotDao
import com.oppzippy.openscq30.features.statusnotification.storage.QuickPresetSlotDao
import com.oppzippy.openscq30.lib.bindings.OpenScq30Device
import com.oppzippy.openscq30.lib.bindings.OpenScq30Exception
import com.oppzippy.openscq30.lib.bindings.OpenScq30Session
import com.oppzippy.openscq30.lib.bindings.SettingIdValuePair
import com.oppzippy.openscq30.lib.wrapper.Value
import com.oppzippy.openscq30.widget.SettingWidget
import dagger.hilt.android.AndroidEntryPoint
import javax.inject.Inject
import kotlin.time.Duration.Companion.milliseconds
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.FlowPreview
import kotlinx.coroutines.MainScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.collectLatest
import kotlinx.coroutines.flow.debounce
import kotlinx.coroutines.flow.emptyFlow
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.flow.flatMapLatest
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.launch

@OptIn(FlowPreview::class, ExperimentalCoroutinesApi::class)
@AndroidEntryPoint
class DeviceService : LifecycleService() {
    companion object {
        private const val TAG = "DeviceService"

        /** Intent extra for setting mac address when launching service */
        const val INTENT_EXTRA_MAC_ADDRESS = "com.oppzippy.openscq30.macAddress"

        const val NOTIFICATION_CHANNEL_ID = "com.oppzippy.openscq30.notification.DeviceServiceChannel"
        const val NOTIFICATION_ID = 1

        const val ACTION_QUICK_PRESET = "com.oppzippy.openscq30.broadcast.QuickPreset"
        const val ACTION_DISCONNECT = "com.oppzippy.openscq30.broadcast.Disconnect"
        const val ACTION_SEND_NOTIFICATION = "com.oppzippy.openscq30.broadcast.SendNotification"
        const val ACTION_SET_SETTING_VALUE = "com.oppzippy.openscq30.broadcast.SetSettingValue"
        const val ACTION_UPDATE_WIDGET = "com.oppzippy.openscq30.broadcast.UPDATE_WIDGET"

        private val ACTIONS = listOf(
            ACTION_DISCONNECT,
            ACTION_QUICK_PRESET,
            ACTION_SEND_NOTIFICATION,
            ACTION_SET_SETTING_VALUE,
            ACTION_UPDATE_WIDGET,
        )

        const val INTENT_EXTRA_PRESET_SLOT_INDEX = "com.oppzippy.openscq30.presetNumber"
        const val INTENT_EXTRA_SETTING_ID = "com.oppzippy.openscq30.settingId"
        const val INTENT_EXTRA_SETTING_VALUE = "com.oppzippy.openscq30.settingValue"

        private val _isRunning = MutableStateFlow(false)
        val isRunning = _isRunning.asStateFlow()
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
            quickPresetSlotDao.allNames(device.model()).map { slotNames ->
                // A slot can outlive the preset it points to, so don't offer buttons for presets that were deleted
                existingQuickPresetNames(device)?.let { retainExistingQuickPresets(slotNames, it) } ?: slotNames
            }
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
                ACTION_DISCONNECT -> {
                    Log.i(TAG, "got disconnect request, initiating stop service")
                    stopSelf()
                }

                ACTION_QUICK_PRESET -> {
                    val presetIndex = intent.getIntExtra(INTENT_EXTRA_PRESET_SLOT_INDEX, Int.MIN_VALUE)
                    if (presetIndex == Int.MIN_VALUE) {
                        Log.e(TAG, "got ACTION_QUICK_PRESET, but INTENT_EXTRA_PRESET_SLOT_INDEX is not specified")
                        return
                    }
                    val presetName = quickPresetNames.value.getOrNull(presetIndex)
                    if (presetName == null) {
                        Log.w(TAG, "got ACTION_QUICK_PRESET for slot $presetIndex, but it has no quick preset")
                        showQuickPresetError()
                        return
                    }
                    lifecycleScope.launch {
                        connectionStatusFlow.value.let {
                            if (it is ConnectionStatus.Connected) {
                                val device = it.deviceManager.device
                                session.quickPresetHandler().use { quickPresetHandler ->
                                    try {
                                        val presetExists =
                                            quickPresetHandler.quickPresets(device).any { it.name == presetName }
                                        if (presetExists) {
                                            quickPresetHandler.activate(device, presetName)
                                        } else {
                                            Log.w(TAG, "quick preset $presetName no longer exists, removing its slot")
                                            quickPresetSlotDao.deleteByName(device.model(), presetName)
                                            showQuickPresetError()
                                        }
                                    } catch (ex: OpenScq30Exception) {
                                        Log.e(TAG, "error activating quick preset $presetName", ex)
                                        showQuickPresetError()
                                    }
                                }
                            }
                        }
                    }
                }

                ACTION_SEND_NOTIFICATION -> {
                    sendNotification()
                }

                ACTION_SET_SETTING_VALUE -> {
                    lifecycleScope.launch {
                        val settingId = intent.getStringExtra(INTENT_EXTRA_SETTING_ID)
                        val value =
                            IntentCompat.getParcelableExtra(intent, INTENT_EXTRA_SETTING_VALUE, Value::class.java)
                        if (settingId != null && value != null) {
                            connectionStatusFlow.value.let {
                                if (it is ConnectionStatus.Connected) {
                                    val device = it.deviceManager.device
                                    try {
                                        device.setSettingValues(listOf(SettingIdValuePair(settingId, value)))
                                    } catch (ex: IllegalStateException) {
                                        Log.w(TAG, "device was closed, not setting values", ex)
                                    }
                                }
                            }
                        } else {
                            Log.e(TAG, "$ACTION_SET_SETTING_VALUE requires settingId and value to be set")
                        }
                    }
                }

                ACTION_UPDATE_WIDGET -> {
                    val appWidgetId =
                        intent.getIntExtra(AppWidgetManager.EXTRA_APPWIDGET_ID, AppWidgetManager.INVALID_APPWIDGET_ID)
                    if (appWidgetId != AppWidgetManager.INVALID_APPWIDGET_ID) {
                        val manager = GlanceAppWidgetManager(applicationContext)
                        val glanceId = manager.getGlanceIdBy(appWidgetId)
                        lifecycleScope.launch {
                            SettingWidget().updateConnectionStatus(
                                applicationContext,
                                session,
                                connectionStatusFlow.value,
                                glanceId,
                            )
                        }
                    } else {
                        Log.e(TAG, "tried to update widget without specifying EXTRA_APPWIDGET_ID")
                    }
                }
            }
        }
    }

    override fun onCreate() {
        super.onCreate()
        _isRunning.value = true

        lifecycleScope.launch { quickPresetNames.collectLatest { sendNotification() } }
        lifecycleScope.launch { featuredSettingIds.collectLatest { sendNotification() } }

        ContextCompat.registerReceiver(
            this,
            broadcastReceiver,
            IntentFilter().apply { ACTIONS.forEach { addAction(it) } },
            ContextCompat.RECEIVER_NOT_EXPORTED,
        )

        createNotificationChannel()
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        super.onStartCommand(intent, flags, startId)

        val notification = buildNotification()
        startForeground(NOTIFICATION_ID, notification)

        intent?.getStringExtra(INTENT_EXTRA_MAC_ADDRESS)?.let { macAddress ->
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
            val widget = SettingWidget()
            connectionStatusFlow.collectLatest { connectionStatus ->
                widget.updateAllConnectionStatus(applicationContext, session, connectionStatus)
                if (connectionStatus is ConnectionStatus.Connected) {
                    connectionStatus.deviceManager.watchForChangeNotification.collectLatest {
                        widget.updateAllConnectionStatus(applicationContext, session, connectionStatus)
                    }
                }
            }
        }

        return START_REDELIVER_INTENT
    }

    override fun onDestroy() {
        super.onDestroy()
        Log.i(TAG, "stopping service")
        unregisterReceiver(broadcastReceiver)
        cancelNotification()
        connectionStatusFlow.value.let {
            if (it is ConnectionStatus.Connected) {
                connectionStatusFlow.value = ConnectionStatus.Disconnected
                it.deviceManager.close()
            }
        }
        MainScope().launch {
            SettingWidget().updateAllConnectionStatus(applicationContext, session, ConnectionStatus.Disconnected)
        }
        _isRunning.value = false
    }

    /** Returns the names of the quick presets that exist for [device], or null if they could not be fetched. */
    private suspend fun existingQuickPresetNames(device: OpenScq30Device): Set<String>? = try {
        session.quickPresetHandler().use { quickPresetHandler ->
            quickPresetHandler.quickPresets(device).map { it.name }.toSet()
        }
    } catch (ex: OpenScq30Exception) {
        Log.w(TAG, "error fetching quick presets, not checking whether slots refer to existing presets", ex)
        null
    } catch (ex: IllegalStateException) {
        Log.w(TAG, "device was closed, can't fetch quick presets", ex)
        null
    }

    private fun showQuickPresetError() {
        Toast.makeText(applicationContext, R.string.error_activating_quick_preset, Toast.LENGTH_SHORT).show()
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
            getSystemService(NOTIFICATION_SERVICE) as NotificationManager
        notificationManager.notify(NOTIFICATION_ID, buildNotification())
    }

    private fun cancelNotification() {
        val notificationManager =
            getSystemService(NOTIFICATION_SERVICE) as NotificationManager
        notificationManager.cancel(NOTIFICATION_ID)
    }

    private fun buildNotification(): Notification = notificationBuilder(
        status = connectionStatusFlow.value,
        quickPresetNames = quickPresetNames.value,
        featuredSettingIds = featuredSettingIds.value,
    )

    private val binder = MyBinder()

    override fun onBind(intent: Intent): IBinder {
        super.onBind(intent)
        return binder
    }

    inner class MyBinder : Binder() {
        fun getService(): DeviceService = this@DeviceService
    }
}
