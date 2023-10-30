package com.oppzippy.openscq30.features.soundcoredevice.impl

import android.bluetooth.BluetoothDevice
import android.bluetooth.BluetoothGatt
import android.content.Context
import com.oppzippy.openscq30.lib.bindings.DeviceFeatureFlags
import com.oppzippy.openscq30.lib.bindings.EqualizerConfiguration
import com.oppzippy.openscq30.lib.bindings.PresetEqualizerProfile
import com.oppzippy.openscq30.lib.bindings.StateUpdatePacket
import com.oppzippy.openscq30.lib.wrapper.SoundcoreDeviceState
import com.oppzippy.openscq30.lib.wrapper.toSoundcoreDeviceState
import io.mockk.coJustRun
import io.mockk.every
import io.mockk.impl.annotations.MockK
import io.mockk.junit4.MockKRule
import io.mockk.justRun
import io.mockk.mockk
import io.mockk.mockkStatic
import kotlinx.coroutines.channels.BufferOverflow
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.test.runTest
import org.junit.Assert
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import java.util.UUID
import java.util.concurrent.TimeoutException

class SoundcoreDeviceConnectorImplTest {
    @get:Rule
    val mockkRule = MockKRule(this)

    @MockK
    lateinit var context: Context

    @MockK
    lateinit var deviceFinder: BluetoothDeviceFinder

    @MockK
    lateinit var deviceFactory: SoundcoreDeviceFactory

    @MockK
    lateinit var device: BluetoothDevice

    @MockK
    lateinit var gatt: BluetoothGatt

    @MockK
    lateinit var callbackHandler: SoundcoreDeviceCallbackHandler

    @MockK
    lateinit var callbackHandlerFactory: SoundcoreDeviceCallbackHandlerFactory

    @MockK(relaxed = true)
    lateinit var stateUpdatePacket: StateUpdatePacket

    @Before
    fun setUp() {
        every { deviceFinder.findByMacAddress(any()) } returns device
        every { deviceFactory.createSoundcoreDevice(any(), any(), any(), any()) } returns mockk()
        every { device.name } returns "Mock Device"
        every { device.address } returns "00:00:00:00:00:00"
        every { gatt.device } returns device
        every { device.connectGatt(any(), any(), any(), BluetoothDevice.TRANSPORT_LE) } returns gatt
        every { callbackHandlerFactory.createSoundcoreDeviceCallbackHandler(any()) } returns callbackHandler
        every { callbackHandler.isDisconnected } returns MutableStateFlow(false)
        every { callbackHandler.serviceUuid } returns MutableStateFlow(UUID(0, 0))

        mockkStatic(StateUpdatePacket::toSoundcoreDeviceState)
        every { stateUpdatePacket.toSoundcoreDeviceState() } returns SoundcoreDeviceState(
            featureFlags = DeviceFeatureFlags.all(),
            leftBatteryLevel = 0,
            rightBatteryLevel = 0,
            isLeftBatteryCharging = false,
            isRightBatteryCharging = false,
            equalizerConfiguration = EqualizerConfiguration(PresetEqualizerProfile.SoundcoreSignature),
            soundModes = null,
            ageRange = null,
            gender = null,
            customHearId = null,
            leftFirmwareVersion = null,
            rightFirmwareVersion = null,
            serialNumber = null,
            dynamicRangeCompressionMinFirmwareVersion = null,
        )
    }

    @Test
    fun connectsWhenEverythingGoesAsExpected() = runTest {
        every { gatt.connect() } returns true
        coJustRun { callbackHandler.waitUntilReady() }

        val packetsFlow = MutableSharedFlow<Packet>(5, 0, BufferOverflow.DROP_OLDEST)
        every { callbackHandler.packetsFlow } returns packetsFlow
        every { callbackHandler.queueCommand(any()) } answers {
            packetsFlow.tryEmit(Packet.StateUpdate(stateUpdatePacket))
        }

        val connector =
            SoundcoreDeviceConnectorImpl(
                context,
                deviceFinder,
                callbackHandlerFactory,
                deviceFactory,
            )
        val device = coroutineScope {
            connector.connectToSoundcoreDevice("00:00:00:00:00:00", this)
        }
        Assert.assertNotEquals(null, device)
    }

    @Test
    fun connectsWhenFirstStateRequestFails() = runTest {
        every { gatt.connect() } returns true
        coJustRun { callbackHandler.waitUntilReady() }

        val packetsFlow = MutableSharedFlow<Packet>(5, 0, BufferOverflow.DROP_OLDEST)
        every { callbackHandler.packetsFlow } returns packetsFlow
        var attemptNumber = 0
        every { callbackHandler.queueCommand(any()) } answers {
            attemptNumber++
            if (attemptNumber >= 2) {
                packetsFlow.tryEmit(Packet.StateUpdate(stateUpdatePacket))
            }
        }

        val connector = SoundcoreDeviceConnectorImpl(
            context,
            deviceFinder,
            callbackHandlerFactory,
            deviceFactory,
        )
        val device = coroutineScope {
            connector.connectToSoundcoreDevice("00:00:00:00:00:00", this)
        }
        Assert.assertNotEquals(null, device)
    }

    @Test(expected = TimeoutException::class)
    fun failsWhenAllStateRequestsTimeOut() = runTest {
        every { gatt.connect() } returns true
        coJustRun { callbackHandler.waitUntilReady() }

        val packetsFlow = MutableSharedFlow<Packet>(5, 0, BufferOverflow.DROP_OLDEST)
        every { callbackHandler.packetsFlow } returns packetsFlow
        justRun { callbackHandler.queueCommand(any()) }

        val connector = SoundcoreDeviceConnectorImpl(
            context,
            deviceFinder,
            callbackHandlerFactory,
            deviceFactory,
        )
        coroutineScope {
            connector.connectToSoundcoreDevice("00:00:00:00:00:00", this)
        }
    }
}
