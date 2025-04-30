package com.oppzippy.openscq30.ui.deviceselection

import android.app.Activity
import android.app.Application
import android.bluetooth.BluetoothDevice
import android.bluetooth.BluetoothManager
import android.companion.AssociationInfo
import android.companion.AssociationRequest
import android.companion.BluetoothDeviceFilter
import android.companion.CompanionDeviceManager
import android.content.Intent
import android.content.IntentSender
import android.os.Build
import android.util.Log
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.viewModelScope
import com.oppzippy.openscq30.lib.bindings.OpenScq30Session
import com.oppzippy.openscq30.lib.wrapper.PairedDevice
import dagger.hilt.android.lifecycle.HiltViewModel
import javax.inject.Inject
import kotlin.time.Duration.Companion.seconds
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.launch

@HiltViewModel
class DeviceSelectionViewModel @Inject constructor(
    private val application: Application,
    private val session: OpenScq30Session,
) : AndroidViewModel(application) {
    val devices = MutableStateFlow(emptyList<PairedDevice>())

    init {
        // Hack to work around older android versions not having onAssociationCreated
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.TIRAMISU) {
            viewModelScope.launch {
                while (true) {
                    delay(5.seconds)
                    refreshPairedDevices()
                }
            }
        }
    }

    fun pair(activity: Activity, pairedDevice: PairedDevice) {
        val pairingRequest = AssociationRequest.Builder()
            .apply {
                this.setSingleDevice(true)
                this.addDeviceFilter(
                    BluetoothDeviceFilter.Builder().apply {
                        this.setAddress(pairedDevice.macAddress)
                    }.build(),
                )
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
                    viewModelScope.launch { refreshPairedDevices() }
                }

                @Deprecated("Deprecated in Java")
                override fun onDeviceFound(intentSender: IntentSender) {
                    super.onDeviceFound(intentSender)
                    activity.startIntentSenderForResult(
                        intentSender,
                        0,
                        Intent().putExtra("pairedDevice", pairedDevice),
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
        viewModelScope.launch {
            val deviceManager = application.getSystemService(CompanionDeviceManager::class.java)
            // Unpair before removing the association, since if something goes wrong, it's less broken to still be
            // associated but not be paired with openscq30_lib rather than the other way around. The other way around would
            // show the user a device available to connect to that we can't actually connect to.
            session.unpair(bluetoothDevice.address)
            // CompanionDeviceManager.disassociate is case sensitive
            deviceManager
                .associations
                .find { it.equals(bluetoothDevice.address, ignoreCase = true) }
                ?.let { deviceManager.disassociate(it) }
            refreshPairedDevices()
        }
    }

    private suspend fun refreshPairedDevices() {
        devices.value = session.pairedDevices()
    }

    fun isBluetoothEnabled(): Boolean {
        val bluetoothManager = application.getSystemService(BluetoothManager::class.java)
        return bluetoothManager.adapter.isEnabled
    }
}
