package com.oppzippy.openscq30.ui.deviceselection

import android.app.Activity
import android.app.Application
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
import com.oppzippy.openscq30.features.soundcoredevice.connectionBackends
import com.oppzippy.openscq30.lib.bindings.OpenScq30Session
import com.oppzippy.openscq30.lib.wrapper.ConnectionDescriptor
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
    val state = MutableStateFlow<DeviceSelectionState>(DeviceSelectionState.Loading)

    init {
        // Hack to work around older android versions not having onAssociationCreated
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.TIRAMISU) {
            viewModelScope.launch {
                while (true) {
                    delay(5.seconds)
                    // check once to avoid doing unnecessary work
                    if (state.value is DeviceSelectionState.Connect) {
                        val devices = session.pairedDevices()
                        // check again in case the state is no longer Connect, since pairedDevices is suspend
                        if (state.value is DeviceSelectionState.Connect) {
                            state.value = DeviceSelectionState.Connect(devices)
                        }
                    }
                }
            }
        }
        viewModelScope.launch { launchConnectScreen() }
    }

    fun pair(activity: Activity, descriptor: ConnectionDescriptor) {
        val pairingRequest = AssociationRequest.Builder()
            .apply {
                this.setSingleDevice(true)
                this.addDeviceFilter(
                    BluetoothDeviceFilter.Builder().apply {
                        this.setAddress(descriptor.macAddress)
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
                    viewModelScope.launch { launchConnectScreen() }
                }

                @Deprecated("Deprecated in Java")
                override fun onDeviceFound(intentSender: IntentSender) {
                    super.onDeviceFound(intentSender)
                    activity.startIntentSenderForResult(
                        intentSender,
                        0,
                        Intent().putExtra("connectionDescriptor", descriptor),
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

    fun unpair(pairedDevice: PairedDevice) {
        viewModelScope.launch {
            val deviceManager = application.getSystemService(CompanionDeviceManager::class.java)
            // Unpair before removing the association, since if something goes wrong, it's less broken to still be
            // associated but not be paired with openscq30_lib rather than the other way around. The other way around would
            // show the user a device available to connect to that we can't actually connect to.
            session.unpair(pairedDevice.macAddress)
            // CompanionDeviceManager.disassociate is case sensitive
            deviceManager
                .associations
                .find { it.equals(pairedDevice.macAddress, ignoreCase = true) }
                ?.let { deviceManager.disassociate(it) }
            launchConnectScreen()
        }
    }

    private suspend fun launchConnectScreen() {
        state.value = DeviceSelectionState.Loading
        val devices = session.pairedDevices()
        state.value = DeviceSelectionState.Connect(devices)
    }

    fun selectModel(model: String) {
        launchSelectDeviceForPairing(model, false)
    }

    fun setDemoMode(state: DeviceSelectionState.SelectDeviceForPairing, isDemo: Boolean) {
        launchSelectDeviceForPairing(state.model, isDemo)
    }

    private fun launchSelectDeviceForPairing(model: String, isDemo: Boolean) {
        this@DeviceSelectionViewModel.state.value = DeviceSelectionState.Loading
        viewModelScope.launch {
            val devices = if (isDemo) {
                session.listDemoDevices(model)
            } else {
                session.listDevicesWithBackends(connectionBackends(application, viewModelScope), model)
            }
            this@DeviceSelectionViewModel.state.value =
                DeviceSelectionState.SelectDeviceForPairing(model = model, isDemoMode = false, devices = devices)
        }
    }
}

sealed class DeviceSelectionState {
    data object Loading : DeviceSelectionState()
    data class Connect(val devices: List<PairedDevice>) : DeviceSelectionState()
    data object SelectModelForPairing : DeviceSelectionState()

    data class SelectDeviceForPairing(
        val model: String,
        val isDemoMode: Boolean,
        val devices: List<ConnectionDescriptor>,
    ) : DeviceSelectionState()
}
