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
import androidx.core.content.ContextCompat
import androidx.lifecycle.LifecycleService
import androidx.lifecycle.lifecycleScope
import com.oppzippy.openscq30.features.quickpresets.storage.QuickPresetRepository
import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDeviceConnector
import com.oppzippy.openscq30.features.soundcoredevice.service.SoundcoreDeviceNotification.ACTION_DISCONNECT
import com.oppzippy.openscq30.features.soundcoredevice.service.SoundcoreDeviceNotification.ACTION_QUICK_PRESET
import com.oppzippy.openscq30.features.soundcoredevice.service.SoundcoreDeviceNotification.ACTION_SEND_NOTIFICATION
import com.oppzippy.openscq30.features.soundcoredevice.service.SoundcoreDeviceNotification.INTENT_EXTRA_PRESET_ID
import com.oppzippy.openscq30.features.soundcoredevice.service.SoundcoreDeviceNotification.NOTIFICATION_CHANNEL_ID
import com.oppzippy.openscq30.features.soundcoredevice.service.SoundcoreDeviceNotification.NOTIFICATION_ID
import com.oppzippy.openscq30.features.soundcoredevice.usecases.ActivateQuickPresetUseCase
import dagger.hilt.android.AndroidEntryPoint
import javax.inject.Inject
import kotlin.time.Duration.Companion.milliseconds
import kotlin.time.Duration.Companion.seconds
import kotlinx.coroutines.FlowPreview
import kotlinx.coroutines.MainScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.collectLatest
import kotlinx.coroutines.flow.debounce
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.launch
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock

@OptIn(FlowPreview::class)
@AndroidEntryPoint
class DeviceService : LifecycleService() {
    companion object {
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
    lateinit var deviceConnector: SoundcoreDeviceConnector

    @Inject
    lateinit var activateQuickPresetUseCase: ActivateQuickPresetUseCase

    @Inject
    lateinit var notificationBuilder: NotificationBuilder

    @Inject
    lateinit var quickPresetRepository: QuickPresetRepository

    private var quickPresetNames = MutableStateFlow<List<String?>>(emptyList())
    lateinit var connectionManager: DeviceConnectionManager

    private val broadcastReceiver = object : BroadcastReceiver() {
        override fun onReceive(context: Context?, intent: Intent?) {
            when (intent?.action) {
                ACTION_DISCONNECT -> {
                    MainScope().launch {
                        // Disconnecting will trigger stopping this service
                        connectionManager.disconnect()
                    }
                }

                ACTION_QUICK_PRESET -> {
                    val presetId = intent.getIntExtra(INTENT_EXTRA_PRESET_ID, 0)
                    lifecycleScope.launch {
                        activateQuickPresetUseCase(presetId, connectionManager)
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
        connectionManager = DeviceConnectionManager(deviceConnector, lifecycleScope)

        lifecycleScope.launch {
            connectionManager.connectionStatusFlow.collectLatest { connectionStatus ->
                if (connectionStatus is ConnectionStatus.Connected) {
                    var isMigrated = false
                    connectionStatus.device.stateFlow.map { it.model }.collectLatest { deviceModel ->
                        if (deviceModel != null) {
                            if (!isMigrated) {
                                isMigrated = true
                                quickPresetRepository.migrateBleServiceUuids(
                                    deviceModel,
                                    connectionStatus.device.bleServiceUuid,
                                )
                            }
                            quickPresetRepository.getNamesForDevice(deviceModel)
                                .collectLatest { quickPresetNames.value = it }
                        } else {
                            quickPresetNames.value = emptyList()
                        }
                    }
                } else {
                    quickPresetNames.value = emptyList()
                }
            }
        }

        lifecycleScope.launch {
            connectionManager.connectionStatusFlow.first { it is ConnectionStatus.Disconnected }
            stopSelf()
        }
        lifecycleScope.launch {
            connectionManager.connectionStatusFlow.collectLatest {
                if (it is ConnectionStatus.Connected) {
                    it.device.stateFlow.debounce(500.milliseconds).collectLatest {
                        sendNotification()
                    }
                }
            }
        }
        lifecycleScope.launch {
            quickPresetNames.debounce(1.seconds).collectLatest { sendNotification() }
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

    private val connectToDeviceMutex = Mutex()
    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        super.onStartCommand(intent, flags, startId)

        val notification = buildNotification()
        startForeground(NOTIFICATION_ID, notification)

        intent?.getStringExtra(MAC_ADDRESS)?.let { macAddress ->
            lifecycleScope.launch {
                connectToDeviceMutex.withLock {
                    connectionManager.connect(macAddress)
                }
            }
        }

        return START_REDELIVER_INTENT
    }

    override fun onDestroy() {
        super.onDestroy()
        unregisterReceiver(broadcastReceiver)
        cancelNotification()

        MainScope().launch {
            connectionManager.disconnect()
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
        status = connectionManager.connectionStatusFlow.value,
        quickPresetNames = quickPresetNames.value,
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
