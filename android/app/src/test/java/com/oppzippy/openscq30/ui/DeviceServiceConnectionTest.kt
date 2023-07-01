package com.oppzippy.openscq30.ui

import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDevice
import com.oppzippy.openscq30.features.soundcoredevice.service.ConnectionStatus
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceService
import com.oppzippy.openscq30.lib.SoundcoreDeviceState
import com.oppzippy.openscq30.test.MainDispatcherRule
import com.oppzippy.openscq30.ui.devicesettings.models.UiDeviceState
import io.mockk.every
import io.mockk.impl.annotations.MockK
import io.mockk.junit4.MockKRule
import io.mockk.mockk
import io.mockk.verify
import junit.framework.TestCase.assertEquals
import kotlinx.coroutines.FlowPreview
import kotlinx.coroutines.Job
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.flow.timeout
import kotlinx.coroutines.test.runTest
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import kotlin.time.Duration.Companion.milliseconds

@OptIn(FlowPreview::class)
class DeviceServiceConnectionTest {
    @get:Rule
    val mockkRule = MockKRule(this)

    @get:Rule
    val mainDispatcherRule = MainDispatcherRule()

    @MockK
    lateinit var binder: DeviceService.MyBinder

    @MockK
    lateinit var service: DeviceService

    lateinit var connectionStatusFlow: MutableStateFlow<ConnectionStatus>

    @Before
    fun setUp() {
        connectionStatusFlow = MutableStateFlow(ConnectionStatus.Disconnected)
        every { service.connectionManager.connectionStatusFlow } returns connectionStatusFlow
    }

    @Test
    fun startsDisconnected() {
        val connection = DeviceServiceConnection(unbind = {})
        assertEquals(UiDeviceState.Disconnected, connection.uiDeviceStateFlow.value)
    }

    @Test
    fun movesToLoadingState() = runTest {
        val connection = DeviceServiceConnection(unbind = {})

        every { binder.getService() } returns service
        connection.onServiceConnected(null, binder)
        connectionStatusFlow.value = ConnectionStatus.Connecting("00:00:00:00:00:00", Job())

        // It will throw an exception if it times out, so no need to assert
        connection.uiDeviceStateFlow.timeout(10.milliseconds).first { it is UiDeviceState.Loading }
    }

    @Test
    fun movesToConnectedState() = runTest {
        val connection = DeviceServiceConnection(unbind = {})

        every { binder.getService() } returns service
        connection.onServiceConnected(null, binder)

        // we aren't linked with openscq30_android.so, so we need to mock SoundcoreDeviceState
        val deviceStateFlow = MutableStateFlow<SoundcoreDeviceState>(mockk())
        val device: SoundcoreDevice = mockk()
        every { device.name } returns "Test"
        every { device.macAddress } returns "00:00:00:00:00:00"
        every { device.stateFlow } returns deviceStateFlow

        connectionStatusFlow.value = ConnectionStatus.Connected(device)

        val state = connection.uiDeviceStateFlow.timeout(10.milliseconds)
            .first { it is UiDeviceState.Connected } as UiDeviceState.Connected
        assertEquals(deviceStateFlow.value, state.deviceState)
    }

    @Test
    fun movesToDisconnected() = runTest {
        val connection = DeviceServiceConnection(unbind = {})

        every { binder.getService() } returns service
        connection.onServiceConnected(null, binder)

        // we aren't linked with openscq30_android.so, so we need to mock SoundcoreDeviceState
        val deviceStateFlow = MutableStateFlow<SoundcoreDeviceState>(mockk(relaxed = true))
        val device: SoundcoreDevice = mockk()
        every { device.stateFlow } returns deviceStateFlow
        every { device.name } returns "Test"
        every { device.macAddress } returns "00:00:00:00:00:00"

        connectionStatusFlow.value = ConnectionStatus.Connected(device)

        val state = connection.uiDeviceStateFlow.timeout(10.milliseconds)
            .first { it is UiDeviceState.Connected } as UiDeviceState.Connected
        assertEquals(deviceStateFlow.value, state.deviceState)

        connection.onServiceDisconnected(mockk())
        assertEquals(UiDeviceState.Disconnected, connection.uiDeviceStateFlow.value)
    }

    @Test
    fun unbindsOnDisconnect() {
        val unbind: () -> Unit = mockk(relaxed = true)
        val connection = DeviceServiceConnection(unbind = unbind)

        every { binder.getService() } returns service
        connection.onServiceConnected(null, binder)
        connectionStatusFlow.value = ConnectionStatus.Disconnected

        verify(exactly = 1) { unbind() }
    }
}
