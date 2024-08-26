package com.oppzippy.openscq30.features.bluetoothdeviceprovider

import android.companion.CompanionDeviceManager
import android.content.Context
import android.os.Build
import androidx.annotation.RequiresPermission
import dagger.hilt.android.qualifiers.ApplicationContext
import javax.inject.Inject

class BluetoothDeviceProviderImpl @Inject constructor(@ApplicationContext private val context: Context) :
    BluetoothDeviceProvider {
    /**
     * The caller is responsible for checking for bluetooth permission
     */
    @RequiresPermission(value = "android.permission.BLUETOOTH_CONNECT")
    override fun getDevices(): List<BluetoothDevice> {
        val deviceManager = context.getSystemService(CompanionDeviceManager::class.java)
        return if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            deviceManager.myAssociations.mapNotNull { associationInfo ->
                val macAddress = associationInfo.deviceMacAddress
                if (macAddress != null) {
                    BluetoothDevice(
                        associationInfo.displayName?.toString() ?: "Unknown",
                        macAddress.toString().uppercase(),
                    )
                } else {
                    null
                }
            }
        } else {
            deviceManager.associations.map { macAddress ->
                BluetoothDevice("Unknown", macAddress)
            }
        }
    }
}
