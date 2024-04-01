package com.oppzippy.openscq30.ui.deviceselection

import android.app.Activity
import android.app.Application
import android.companion.AssociationRequest
import android.companion.BluetoothDeviceFilter
import android.companion.CompanionDeviceManager
import android.content.IntentSender
import android.content.pm.PackageManager
import android.os.Build
import android.util.Log
import androidx.lifecycle.AndroidViewModel
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDevice
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDeviceProvider
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import java.util.regex.Pattern
import javax.inject.Inject

@HiltViewModel
class DeviceSelectionViewModel @Inject constructor(
    private val application: Application,
    private val bluetoothDeviceProvider: BluetoothDeviceProvider,
) : AndroidViewModel(application) {
    val devices = MutableStateFlow(getDevices())

    fun pair(activity: Activity) {
        val pairingRequest = AssociationRequest.Builder()
            .addDeviceFilter(
                BluetoothDeviceFilter.Builder().apply {
                    this.setNamePattern(Pattern.compile("Soundcore"))
                    // todo
                    // this.addServiceUuid()
                }.build(),
            )
            .build()
        val deviceManager = application.getSystemService(CompanionDeviceManager::class.java)
        deviceManager.associate(
            pairingRequest,
            object : CompanionDeviceManager.Callback() {
                @Deprecated(
                    "Deprecated in Java",
                    ReplaceWith(
                        "super.onDeviceFound(intentSender)",
                        "android.companion.CompanionDeviceManager.Callback",
                    ),
                )
                override fun onDeviceFound(intentSender: IntentSender) {
                    super.onDeviceFound(intentSender)
                    activity.startIntentSenderForResult(
                        intentSender,
                        0,
                        null,
                        0,
                        0,
                        0,
                    )
                    refreshDevices()
                }

                override fun onFailure(error: CharSequence?) {
                    Log.w("DeviceSelectionViewModel", "error pairing: $error")
                }
            },
            null,
        )
    }

    fun unpair(bluetoothDevice: BluetoothDevice) {
        val deviceManager = application.getSystemService(CompanionDeviceManager::class.java)
        // CompanionDeviceManager.disassociate is case sensitive
        deviceManager
            .associations
            .find { it.equals(bluetoothDevice.address, ignoreCase = true) }
            ?.let { deviceManager.disassociate(it) }
    }

    fun refreshDevices() {
        devices.value = getDevices()
    }

    private fun getDevices(): List<BluetoothDevice> {
        val hasBluetoothPermission = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            application.checkSelfPermission(android.Manifest.permission.BLUETOOTH_CONNECT) == PackageManager.PERMISSION_GRANTED
        } else {
            application.checkSelfPermission(android.Manifest.permission.BLUETOOTH) == PackageManager.PERMISSION_GRANTED
        }
        return if (hasBluetoothPermission) {
            val deviceManager = application.getSystemService(CompanionDeviceManager::class.java)
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
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
        } else {
            emptyList()
        }
    }
}
