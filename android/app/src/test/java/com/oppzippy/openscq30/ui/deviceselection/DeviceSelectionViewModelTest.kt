package com.oppzippy.openscq30.ui.deviceselection

import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDevice
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDeviceProvider
import io.mockk.every
import io.mockk.impl.annotations.RelaxedMockK
import io.mockk.junit4.MockKRule
import org.junit.Assert.assertEquals
import org.junit.Rule
import org.junit.Test

class DeviceSelectionViewModelTest {
    @get:Rule
    val mockkRule = MockKRule(this)

    @RelaxedMockK
    lateinit var deviceProvider: BluetoothDeviceProvider

    @Test
    fun hasInitialDevices() {
        val devices = listOf(BluetoothDevice("Test Device", "00:00:00:00:00:00"))
        every { deviceProvider.getDevices() } returns devices

        val viewModel = DeviceSelectionViewModel(deviceProvider)
        assertEquals(devices, viewModel.devices.value)
    }

    @Test
    fun doesNotUpdateDevicesUntilRefreshDevicesIsCalled() {
        every { deviceProvider.getDevices() } returns emptyList()
        val viewModel = DeviceSelectionViewModel(deviceProvider)

        val devices = listOf(BluetoothDevice("Test Device", "00:00:00:00:00:00"))
        every { deviceProvider.getDevices() } returns devices

        assertEquals(emptyList<BluetoothDevice>(), viewModel.devices.value)
        viewModel.refreshDevices()
        assertEquals(devices, viewModel.devices.value)
    }
}
