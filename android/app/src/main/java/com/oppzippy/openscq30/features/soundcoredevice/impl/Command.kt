package com.oppzippy.openscq30.features.soundcoredevice.impl

import android.bluetooth.BluetoothGattDescriptor

sealed class Command {
    object Read: Command()
    class Write(val bytes: ByteArray): Command()
    class WriteDescriptor(val descriptor: BluetoothGattDescriptor, val value: ByteArray): Command()
    class SetMtu(val mtu: Int): Command()
}
