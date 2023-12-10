package com.oppzippy.openscq30.features.soundcoredevice.impl

import android.bluetooth.BluetoothGattDescriptor

sealed class Command {
    data object Read : Command()
    class Write(val bytes: ByteArray) : Command() {
        override fun toString(): String {
            return "Command.Write $bytes"
        }
    }

    class WriteDescriptor(val descriptor: BluetoothGattDescriptor, val value: ByteArray) :
        Command() {
        override fun toString(): String {
            return "Command.SetDescriptor ${descriptor.uuid} $value"
        }
    }

    class SetMtu(val mtu: Int) : Command() {
        override fun toString(): String {
            return "Command.SetMtu $mtu"
        }
    }
}
