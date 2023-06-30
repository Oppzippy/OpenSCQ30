package com.oppzippy.openscq30.features.soundcoredevice.service

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.graphics.drawable.Icon
import android.os.Binder
import android.os.IBinder
import androidx.lifecycle.LifecycleService
import androidx.lifecycle.lifecycleScope
import com.oppzippy.openscq30.MainActivity
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDeviceFactory
import dagger.hilt.android.AndroidEntryPoint
import kotlinx.coroutines.MainScope
import kotlinx.coroutines.flow.collectLatest
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.launch
import javax.inject.Inject

@AndroidEntryPoint
class DeviceService : LifecycleService() {
    companion object {
        private const val NOTIFICATION_ID = 1
        private const val CHANNEL_ID = "com.oppzippy.openscq30.notification.DeviceServiceChannel"
        private const val DISCONNECT = "com.oppzippy.openscq30.broadcast.Disconnect"

        /** Intent extra for setting mac address when launching service */
        const val MAC_ADDRESS = "com.oppzippy.openscq30.macAddress"
    }

    @Inject
    lateinit var factory: SoundcoreDeviceFactory
    lateinit var connectionManager: DeviceConnectionManager

    private val broadcastReceiver = object : BroadcastReceiver() {
        override fun onReceive(context: Context?, intent: Intent?) {
            when (intent?.action) {
                DISCONNECT -> {
                    MainScope().launch {
                        // Disconnecting will trigger stopping this service
                        connectionManager.disconnect()
                    }
                }
            }
        }
    }

    override fun onCreate() {
        super.onCreate()
        connectionManager = DeviceConnectionManager(factory, lifecycleScope)

        lifecycleScope.launch {
            connectionManager.connectionStateFlow.first { it is ConnectionStatus.Disconnected }
            stopSelf()
        }

        val filter = IntentFilter(DISCONNECT).apply {}
        registerReceiver(broadcastReceiver, filter)
    }

    override fun onDestroy() {
        super.onDestroy()
        unregisterReceiver(broadcastReceiver)

        val notificationManager =
            getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
        notificationManager.cancel(NOTIFICATION_ID)

        MainScope().launch {
            connectionManager.disconnect()
        }
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        super.onStartCommand(intent, flags, startId)

        createNotificationChannel()
        val notification = buildNotification()
        startForeground(NOTIFICATION_ID, notification)

        lifecycleScope.launch {
            connectionManager.connectionStateFlow.collectLatest {
                val notificationManager =
                    getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
                notificationManager.notify(NOTIFICATION_ID, buildNotification())
            }
        }

        intent?.getStringExtra(MAC_ADDRESS)?.let { macAddress ->
            lifecycleScope.launch {
                setMacAddress(macAddress)
            }
        }

        return START_REDELIVER_INTENT
    }

    private fun createNotificationChannel() {
        val channel = NotificationChannel(
            CHANNEL_ID,
            "Device Connection Service",
            NotificationManager.IMPORTANCE_LOW,
        )
        channel.enableVibration(false)
        channel.enableLights(false)
        channel.setSound(null, null)
        val manager = getSystemService(NotificationManager::class.java)
        manager.createNotificationChannel(channel)
    }

    private fun buildNotification(): Notification {
        val openAppIntent = Intent(this, MainActivity::class.java)
        openAppIntent.addFlags(Intent.FLAG_ACTIVITY_SINGLE_TOP)

        val status = connectionManager.connectionStateFlow.value

        val builder = Notification.Builder(this, CHANNEL_ID).setOngoing(true)
            .setSmallIcon(R.drawable.headphones).setContentTitle(
                when (status) {
                    is ConnectionStatus.AwaitingConnection -> getString(R.string.awaiting_connection)
                    is ConnectionStatus.Connected -> getString(R.string.connected_to).format(status.device.name)

                    is ConnectionStatus.Connecting -> getString(R.string.connecting_to).format(
                        status.macAddress
                    )

                    ConnectionStatus.Disconnected -> getString(R.string.disconnected)
                }
            ).setContentIntent(
                PendingIntent.getActivity(
                    this,
                    1,
                    openAppIntent,
                    PendingIntent.FLAG_IMMUTABLE or PendingIntent.FLAG_UPDATE_CURRENT,
                )
            ).addAction(
                Notification.Action.Builder(
                    Icon.createWithResource(this, R.drawable.headphones),
                    getString(R.string.disconnect),
                    PendingIntent.getBroadcast(
                        this,
                        1,
                        Intent().apply {
                            action = DISCONNECT
                        },
                        PendingIntent.FLAG_IMMUTABLE,
                    )
                ).build()
            )
        return builder.build()
    }

    private val binder = MyBinder()

    override fun onBind(intent: Intent): IBinder {
        super.onBind(intent)
        return binder
    }

    suspend fun setMacAddress(macAddress: String?) {
        if (macAddress != null) {
            connectionManager.connect(macAddress)
        } else {
            connectionManager.disconnect()
        }
    }

    inner class MyBinder : Binder() {
        fun getService(): DeviceService = this@DeviceService
    }

}
