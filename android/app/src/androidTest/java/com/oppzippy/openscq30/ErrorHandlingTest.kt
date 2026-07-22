package com.oppzippy.openscq30

import com.oppzippy.openscq30.actions.addAndConnectToDemoDevice
import com.oppzippy.openscq30.actions.addDemoDevice
import com.oppzippy.openscq30.lib.bindings.DeviceModel
import com.oppzippy.openscq30.lib.bindings.MacAddr6
import com.oppzippy.openscq30.lib.bindings.ManualConnectionBackends
import com.oppzippy.openscq30.lib.bindings.NoHandle
import com.oppzippy.openscq30.lib.bindings.OpenScq30Device
import com.oppzippy.openscq30.lib.bindings.OpenScq30Exception
import com.oppzippy.openscq30.lib.bindings.OpenScq30Session
import com.oppzippy.openscq30.lib.bindings.newSessionWithInMemoryDb
import com.oppzippy.openscq30.lib.bindings.translateDeviceModel
import com.oppzippy.openscq30.lib.hilt.OpenSCQ30SessionModule
import com.oppzippy.openscq30.lib.wrapper.ConnectionDescriptor
import com.oppzippy.openscq30.lib.wrapper.PairedDevice
import dagger.hilt.android.testing.BindValue
import dagger.hilt.android.testing.HiltAndroidTest
import dagger.hilt.android.testing.UninstallModules
import kotlinx.coroutines.runBlocking
import org.junit.Test

@UninstallModules(OpenSCQ30SessionModule::class)
@HiltAndroidTest
class ErrorHandlingTest : OpenSCQ30RootTestBase() {
    val fakeSession = FakeOpenSCQ30Session(runBlocking { newSessionWithInMemoryDb() })

    @BindValue
    val session: OpenScq30Session = fakeSession

    @Test
    fun showsToastWhenErrorPairing() {
        fakeSession.pairThrows = true

        addDemoDevice(composeRule, translateDeviceModel("SoundcoreA3028"))
        composeRule.waitForIdle() // wait for toast to be shown
        composeRule.waitUntil(
            4000,
            { toasts.any { it == getString(R.string.error_pairing) } },
        )
    }

    @Test
    fun showsToastWhenErrorConnecting() {
        fakeSession.connectWithBackendsThrows = true

        addAndConnectToDemoDevice(composeRule, translateDeviceModel("SoundcoreA3028"))

        composeRule.waitForIdle() // wait for toast to be shown
        composeRule.waitUntil(
            4000,
            { toasts.any { it == getString(R.string.error_connecting) } },
        )
    }
}

class FakeOpenSCQ30Session(private val real: OpenScq30Session) : OpenScq30Session(NoHandle) {
    var pairThrows = false
    var connectWithBackendsThrows = false

    override suspend fun pairedDevices(): List<PairedDevice> = real.pairedDevices()

    override suspend fun listDemoDevices(model: DeviceModel): List<ConnectionDescriptor> = real.listDemoDevices(model)

    override suspend fun listDevicesWithBackends(
        backends: ManualConnectionBackends,
        model: DeviceModel,
    ): List<ConnectionDescriptor> = real.listDevicesWithBackends(backends, model)

    override suspend fun unpair(macAddress: MacAddr6) = real.unpair(macAddress)

    override suspend fun pair(pairedDevice: PairedDevice) {
        if (pairThrows) {
            throw OpenScq30Exception.DeviceException("fake")
        }
        real.pair(pairedDevice)
    }

    override suspend fun connectWithBackends(
        backends: ManualConnectionBackends,
        macAddress: MacAddr6,
    ): OpenScq30Device {
        if (connectWithBackendsThrows) {
            throw OpenScq30Exception.DeviceException("fake")
        }
        return real.connectWithBackends(backends, macAddress)
    }
}
