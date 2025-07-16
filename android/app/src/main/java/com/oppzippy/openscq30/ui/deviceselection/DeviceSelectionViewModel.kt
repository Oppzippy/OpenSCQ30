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
import android.widget.Toast
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.viewModelScope
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.features.soundcoredevice.connectionBackends
import com.oppzippy.openscq30.lib.bindings.OpenScq30Exception
import com.oppzippy.openscq30.lib.bindings.OpenScq30Session
import com.oppzippy.openscq30.lib.wrapper.ConnectionDescriptor
import com.oppzippy.openscq30.lib.wrapper.PairedDevice
import com.oppzippy.openscq30.ui.utils.ToastHandler
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
    private val toastHandler: ToastHandler,
) : AndroidViewModel(application) {
    val pageState = MutableStateFlow<DeviceSelectionPage>(DeviceSelectionPage.Loading)

    companion object {
        const val TAG = "DeviceSelectionViewModel"
    }

    init {
        // Hack to work around older android versions not having onAssociationCreated
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.TIRAMISU) {
            viewModelScope.launch {
                while (true) {
                    delay(5.seconds)
                    // check once to avoid doing unnecessary work
                    if (pageState.value is DeviceSelectionPage.Connect) {
                        val devices = session.pairedDevices()
                        // check again in case the state is no longer Connect, since pairedDevices is suspend
                        if (pageState.value is DeviceSelectionPage.Connect) {
                            pageState.value = DeviceSelectionPage.Connect(devices)
                        }
                    }
                }
            }
        }
        viewModelScope.launch { launchConnectScreen() }
    }

    fun pair(activity: Activity, pairedDevice: PairedDevice) {
        viewModelScope.launch {
            try {
                if (pairedDevice.isDemo) {
                    pairDemo(pairedDevice)
                } else {
                    pairReal(activity, pairedDevice)
                }
                launchConnectScreen()
            } catch (ex: OpenScq30Exception) {
                Log.e(TAG, "error pairing with ${pairedDevice.model}", ex)
                toastHandler.add(R.string.error_pairing, Toast.LENGTH_SHORT)
            }
        }
    }

    private suspend fun pairDemo(pairedDevice: PairedDevice) {
        session.pair(pairedDevice)
    }

    private fun pairReal(activity: Activity, pairedDevice: PairedDevice) {
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
                    viewModelScope.launch { launchConnectScreen() }
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
                    Log.w(TAG, "error pairing: $error")
                    toastHandler.add(R.string.error_pairing, Toast.LENGTH_SHORT)
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

    fun refreshDevices() {
        viewModelScope.launch { launchConnectScreen() }
    }

    suspend fun launchConnectScreen() {
        pageState.value = DeviceSelectionPage.Loading
        val devices = session.pairedDevices()
        pageState.value = DeviceSelectionPage.Connect(devices)
    }

    fun selectModel(model: String) {
        launchSelectDeviceForPairing(model, false)
    }

    fun setDemoMode(pageState: DeviceSelectionPage.SelectDeviceForPairing, isDemo: Boolean) {
        launchSelectDeviceForPairing(pageState.model, isDemo)
    }

    private fun launchSelectDeviceForPairing(model: String, isDemoMode: Boolean) {
        this@DeviceSelectionViewModel.pageState.value = DeviceSelectionPage.Loading
        viewModelScope.launch {
            val devices = if (isDemoMode) {
                session.listDemoDevices(model)
            } else {
                session.listDevicesWithBackends(connectionBackends(application, viewModelScope), model)
            }
            this@DeviceSelectionViewModel.pageState.value =
                DeviceSelectionPage.SelectDeviceForPairing(model = model, isDemoMode = isDemoMode, devices = devices)
        }
    }

    fun back() {
        pageState.value.let {
            if (it is Back) {
                it.back(this)
            }
        }
    }

    val hasBack: Boolean
        get() = pageState.value is Back
}

interface Back {
    fun back(viewModel: DeviceSelectionViewModel)
}

sealed class DeviceSelectionPage {
    data object Loading : DeviceSelectionPage()
    data class Connect(val devices: List<PairedDevice>) : DeviceSelectionPage()
    data object SelectModelForPairing : DeviceSelectionPage(), Back {
        override fun back(viewModel: DeviceSelectionViewModel) {
            viewModel.viewModelScope.launch { viewModel.launchConnectScreen() }
        }
    }

    data class SelectDeviceForPairing(
        val model: String,
        val isDemoMode: Boolean,
        val devices: List<ConnectionDescriptor>,
    ) : DeviceSelectionPage(),
        Back {
        override fun back(viewModel: DeviceSelectionViewModel) {
            viewModel.pageState.value = SelectModelForPairing
        }
    }

    data object Info : DeviceSelectionPage(), Back {
        override fun back(viewModel: DeviceSelectionViewModel) {
            viewModel.viewModelScope.launch { viewModel.launchConnectScreen() }
        }
    }

    data object Settings : DeviceSelectionPage(), Back {
        override fun back(viewModel: DeviceSelectionViewModel) {
            viewModel.viewModelScope.launch { viewModel.launchConnectScreen() }
        }
    }
}
