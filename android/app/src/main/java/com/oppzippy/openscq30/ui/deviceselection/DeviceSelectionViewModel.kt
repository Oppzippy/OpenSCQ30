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
import com.oppzippy.openscq30.widget.SettingWidget
import dagger.hilt.android.lifecycle.HiltViewModel
import javax.inject.Inject
import kotlin.time.Duration.Companion.seconds
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.collectLatest
import kotlinx.coroutines.launch

@HiltViewModel
class DeviceSelectionViewModel @Inject constructor(
    private val application: Application,
    private val session: OpenScq30Session,
    private val toastHandler: ToastHandler,
) : AndroidViewModel(application) {
    companion object {
        const val TAG = "DeviceSelectionViewModel"
    }

    private val _pairedDevices = MutableStateFlow<List<PairedDevice>?>(null)
    val pairedDevices = _pairedDevices.asStateFlow()

    init {
        viewModelScope.launch { refreshPairedDevices() }

        viewModelScope.launch {
            val widget = SettingWidget()
            // TODO instead make this a global event of some sort that can be subscribed to
            pairedDevices.collectLatest {
                widget.updatePairedDevices(application, session.pairedDevices())
            }
        }
    }

    // Hack to work around older android versions not having onAssociationCreated
    suspend fun pollPairedDevicesOnOldAndroidVersions() {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.TIRAMISU) {
            while (true) {
                delay(5.seconds)
                refreshPairedDevices()
            }
        }
    }

    fun pair(activity: Activity, pairedDevice: PairedDevice, onPaired: () -> Unit) {
        viewModelScope.launch {
            try {
                if (pairedDevice.isDemo) {
                    pairDemo(pairedDevice, onPaired)
                } else {
                    pairReal(activity, pairedDevice, onPaired)
                }
                refreshPairedDevices()
            } catch (ex: OpenScq30Exception) {
                Log.e(TAG, "error pairing with ${pairedDevice.model}", ex)
                toastHandler.add(R.string.error_pairing, Toast.LENGTH_SHORT)
            }
        }
    }

    private suspend fun pairDemo(pairedDevice: PairedDevice, onPaired: () -> Unit) {
        session.pair(pairedDevice)
        onPaired()
    }

    private fun pairReal(activity: Activity, pairedDevice: PairedDevice, onPaired: () -> Unit) {
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
        val deviceManager = activity.getSystemService(CompanionDeviceManager::class.java)
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
                    viewModelScope.launch {
                        session.pair(pairedDevice)
                        onPaired()
                    }
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

    fun unpair(activity: Activity, pairedDevice: PairedDevice) {
        viewModelScope.launch {
            // Unpair before removing the association, since if something goes wrong, it's less broken to still be
            // associated but not be paired with openscq30_lib rather than the other way around. The other way around would
            // show the user a device available to connect to that we can't actually connect to.
            session.unpair(pairedDevice.macAddress)
            refreshPairedDevices()
            if (!pairedDevice.isDemo) {
                val deviceManager = activity.getSystemService(CompanionDeviceManager::class.java)
                // CompanionDeviceManager.disassociate is case sensitive
                deviceManager
                    .associations
                    .find { it.equals(pairedDevice.macAddress, ignoreCase = true) }
                    ?.let {
                        Log.i(TAG, "disassociating from $it")
                        deviceManager.disassociate(it)
                    }
            }
        }
    }

    fun refreshPairedDevices() {
        viewModelScope.launch { _pairedDevices.value = session.pairedDevices() }
    }

    suspend fun listDevices(model: String, isDemoMode: Boolean): List<ConnectionDescriptor> = if (isDemoMode) {
        session.listDemoDevices(model)
    } else {
        session.listDevicesWithBackends(connectionBackends(application, viewModelScope), model)
    }
}
