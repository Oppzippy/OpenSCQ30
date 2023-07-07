package com.oppzippy.openscq30.features.soundcoredevice.service

object SoundcoreDeviceNotification {
    const val NOTIFICATION_CHANNEL_ID =
        "com.oppzippy.openscq30.notification.DeviceServiceChannel"
    const val NOTIFICATION_ID = 1
    const val ACTION_QUICK_PRESET = "com.oppzippy.openscq30.broadcast.QuickPreset"
    const val ACTION_DISCONNECT = "com.oppzippy.openscq30.broadcast.Disconnect"
    const val ACTION_SEND_NOTIFICATION = "com.oppzippy.openscq30.broadcast.SendNotification"
    const val INTENT_EXTRA_PRESET_ID = "com.oppzippy.openscq30.presetNumber"
}
