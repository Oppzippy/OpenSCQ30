package com.oppzippy.openscq30.ui.deviceselection

import android.app.Activity
import android.app.Application
import android.companion.AssociationInfo
import android.companion.AssociationRequest
import android.companion.BluetoothDeviceFilter
import android.companion.CompanionDeviceManager
import android.content.IntentSender
import android.content.pm.PackageManager
import android.os.Build
import android.util.Log
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.viewModelScope
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDevice
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDeviceProvider
import dagger.hilt.android.lifecycle.HiltViewModel
import java.util.regex.Pattern
import javax.inject.Inject
import kotlin.time.Duration.Companion.seconds
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.launch

@HiltViewModel
class DeviceSelectionViewModel @Inject constructor(
    private val application: Application,
    private val bluetoothDeviceProvider: BluetoothDeviceProvider,
) : AndroidViewModel(application) {
    val devices = MutableStateFlow(getDevices())

    init {
        // Hack to work around older android versions not having onAssociationCreated
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.TIRAMISU) {
            viewModelScope.launch {
                while (true) {
                    delay(5.seconds)
                    refreshDevices()
                }
            }
        }
    }

    fun pair(activity: Activity, filtered: Boolean) {
        val pairingRequest = AssociationRequest.Builder()
            .apply {
                if (filtered) {
                    this.addDeviceFilter(
                        BluetoothDeviceFilter.Builder().apply {
                            this.setNamePattern(Pattern.compile("(?i)soundcore"))
                        }.build(),
                    )
                }
            }
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
                override fun onAssociationCreated(associationInfo: AssociationInfo) {
                    super.onAssociationCreated(associationInfo)
                    refreshDevices()
                }

                @Deprecated("Deprecated in Java")
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
        refreshDevices()
    }

    fun refreshDevices() {
        devices.value = getDevices()
    }

    private fun getDevices(): List<BluetoothDevice> {
        val hasBluetoothPermission = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            application.checkSelfPermission(android.Manifest.permission.BLUETOOTH_CONNECT) ==
                PackageManager.PERMISSION_GRANTED
        } else {
            application.checkSelfPermission(android.Manifest.permission.BLUETOOTH) == PackageManager.PERMISSION_GRANTED
        }
        return if (hasBluetoothPermission) {
            bluetoothDeviceProvider.getDevices()
        } else {
            emptyList()
        }
    }
}
