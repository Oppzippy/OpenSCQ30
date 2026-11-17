package com.oppzippy.openscq30.features.autoconnect

import android.bluetooth.BluetoothAdapter
import android.bluetooth.BluetoothManager
import android.bluetooth.BluetoothProfile
import android.companion.CompanionDeviceManager
import android.content.Context
import android.os.Build
import android.util.Log
import com.oppzippy.openscq30.lib.bindings.OpenScq30Session
import com.oppzippy.openscq30.lib.wrapper.PairedDevice
import kotlin.coroutines.resume
import kotlinx.coroutines.suspendCancellableCoroutine
import kotlinx.coroutines.withTimeoutOrNull

private const val TAG = "ConnectedAudioDevices"
private const val PROFILE_PROXY_TIMEOUT_MILLIS = 2000L

/**
 * Returns the (upper case) MAC addresses of Bluetooth audio devices that are currently connected to the phone. Returns
 * an empty set if Bluetooth is off or the BLUETOOTH_CONNECT permission has not been granted.
 */
suspend fun connectedAudioDeviceAddresses(context: Context): Set<String> {
    val adapter = context.getSystemService(BluetoothManager::class.java)?.adapter ?: return emptySet()
    if (!adapter.isEnabled) return emptySet()
    val profiles = buildList {
        add(BluetoothProfile.A2DP)
        add(BluetoothProfile.HEADSET)
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            add(BluetoothProfile.LE_AUDIO)
        }
    }
    return profiles.flatMap { connectedDeviceAddresses(context, adapter, it) }
        .map { it.uppercase() }
        .toSet()
}

private suspend fun connectedDeviceAddresses(context: Context, adapter: BluetoothAdapter, profile: Int): List<String> =
    withTimeoutOrNull(PROFILE_PROXY_TIMEOUT_MILLIS) {
        suspendCancellableCoroutine { continuation ->
            val listener = object : BluetoothProfile.ServiceListener {
                override fun onServiceConnected(profile: Int, proxy: BluetoothProfile) {
                    val addresses = try {
                        proxy.connectedDevices.map { it.address }
                    } catch (ex: SecurityException) {
                        Log.w(TAG, "missing permission to list connected devices", ex)
                        emptyList()
                    }
                    adapter.closeProfileProxy(profile, proxy)
                    if (continuation.isActive) continuation.resume(addresses)
                }

                override fun onServiceDisconnected(profile: Int) {}
            }
            val isProxyRequested = try {
                adapter.getProfileProxy(context, listener, profile)
            } catch (ex: SecurityException) {
                Log.w(TAG, "missing permission to get profile proxy", ex)
                false
            }
            if (!isProxyRequested && continuation.isActive) continuation.resume(emptyList())
        }
    } ?: emptyList()

/**
 * Whether the device is paired with OpenSCQ30 and associated with the companion device manager, which is required to
 * be allowed to start the device service from the background.
 */
suspend fun isAutoConnectCandidate(context: Context, session: OpenScq30Session, macAddress: String): Boolean {
    val deviceManager = context.getSystemService(CompanionDeviceManager::class.java)

    @Suppress("DEPRECATION")
    val isAssociated = deviceManager.associations.any { it.equals(macAddress, ignoreCase = true) }
    val isPairedWithOpenSCQ30 = session.pairedDevices().any { it.macAddress.equals(macAddress, ignoreCase = true) }
    return isAssociated && isPairedWithOpenSCQ30
}

/** Paired (non demo) devices that are currently connected over Bluetooth. */
suspend fun connectedPairedDevices(context: Context, session: OpenScq30Session): List<PairedDevice> {
    val connectedAddresses = connectedAudioDeviceAddresses(context)
    if (connectedAddresses.isEmpty()) return emptyList()
    return session.pairedDevices().filter { !it.isDemo && connectedAddresses.contains(it.macAddress.uppercase()) }
}
