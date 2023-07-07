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
import androidx.lifecycle.LifecycleService
import androidx.lifecycle.lifecycleScope
import com.oppzippy.openscq30.features.quickpresets.storage.QuickPresetDao
import com.oppzippy.openscq30.features.quickpresets.storage.QuickPresetIdAndName
import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDeviceFactory
import com.oppzippy.openscq30.features.soundcoredevice.service.SoundcoreDeviceNotification.ACTION_DISCONNECT
import com.oppzippy.openscq30.features.soundcoredevice.service.SoundcoreDeviceNotification.ACTION_QUICK_PRESET
import com.oppzippy.openscq30.features.soundcoredevice.service.SoundcoreDeviceNotification.INTENT_PRESET_NUMBER
import com.oppzippy.openscq30.features.soundcoredevice.service.SoundcoreDeviceNotification.NOTIFICATION_CHANNEL_ID
import com.oppzippy.openscq30.features.soundcoredevice.service.SoundcoreDeviceNotification.NOTIFICATION_ID
import com.oppzippy.openscq30.features.soundcoredevice.usecases.ActivateQuickPresetUseCase
import dagger.hilt.android.AndroidEntryPoint
import kotlinx.coroutines.FlowPreview
import kotlinx.coroutines.MainScope
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.collectLatest
import kotlinx.coroutines.flow.debounce
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.launch
import javax.inject.Inject
import kotlin.time.Duration.Companion.milliseconds
import kotlin.time.Duration.Companion.seconds

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
    lateinit var factory: SoundcoreDeviceFactory

    @Inject
    lateinit var activateQuickPresetUseCase: ActivateQuickPresetUseCase

    @Inject
    lateinit var notificationBuilder: NotificationBuilder

    @Inject
    lateinit var quickPresetDao: QuickPresetDao

    private lateinit var quickPresetNames: StateFlow<List<QuickPresetIdAndName>>
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
                    val presetNumber = intent.getIntExtra(INTENT_PRESET_NUMBER, 0)
                    lifecycleScope.launch {
                        activateQuickPresetUseCase(presetNumber, connectionManager)
                    }
                }
            }
        }
    }

    override fun onCreate() {
        super.onCreate()
        connectionManager = DeviceConnectionManager(factory, lifecycleScope)

        quickPresetNames =
            quickPresetDao.allNames().stateIn(lifecycleScope, SharingStarted.Eagerly, emptyList())

        lifecycleScope.launch {
            connectionManager.connectionStatusFlow.first { it is ConnectionStatus.Disconnected }
            stopSelf()
        }

        val filter = IntentFilter(ACTION_DISCONNECT).apply {
            addAction(ACTION_QUICK_PRESET)
        }
        registerReceiver(broadcastReceiver, filter)
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        super.onStartCommand(intent, flags, startId)

        createNotificationChannel()
        val notification = buildNotification()
        startForeground(NOTIFICATION_ID, notification)

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

        intent?.getStringExtra(MAC_ADDRESS)?.let { macAddress ->
            lifecycleScope.launch {
                connectionManager.connect(macAddress)
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

    private fun buildNotification(): Notification {
        return notificationBuilder(
            status = connectionManager.connectionStatusFlow.value,
            quickPresetNames = quickPresetNames.value.let {
                listOf(it.getOrNull(0)?.name, it.getOrNull(1)?.name)
            },
        )
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
