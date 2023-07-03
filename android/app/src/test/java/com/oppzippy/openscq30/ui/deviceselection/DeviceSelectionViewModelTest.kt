package com.oppzippy.openscq30.ui.deviceselection

import android.app.Application
import android.content.pm.PackageManager
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDevice
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDeviceProvider
import io.mockk.every
import io.mockk.impl.annotations.RelaxedMockK
import io.mockk.junit4.MockKRule
import io.mockk.mockk
import org.junit.Assert.assertEquals
import org.junit.Rule
import org.junit.Test

class DeviceSelectionViewModelTest {
    @get:Rule
    val mockkRule = MockKRule(this)

    @RelaxedMockK
    lateinit var application: Application

    @RelaxedMockK
    lateinit var deviceProvider: BluetoothDeviceProvider

    @Test
    fun hasInitialDevices() {
        val devices = listOf(BluetoothDevice("Test Device", "00:00:00:00:00:00"))
        every { deviceProvider.getDevices() } returns devices
        every { application.checkSelfPermission(any()) } returns PackageManager.PERMISSION_GRANTED

        val viewModel = DeviceSelectionViewModel(application, deviceProvider)
        assertEquals(devices, viewModel.devices.value)
    }

    @Test
    fun doesNotUpdateDevicesUntilRefreshDevicesIsCalled() {
        every { application.checkSelfPermission(any()) } returns PackageManager.PERMISSION_GRANTED
        every { deviceProvider.getDevices() } returns emptyList()
        val viewModel = DeviceSelectionViewModel(application, deviceProvider)

        val devices = listOf(BluetoothDevice("Test Device", "00:00:00:00:00:00"))
        every { deviceProvider.getDevices() } returns devices

        assertEquals(emptyList<BluetoothDevice>(), viewModel.devices.value)
        viewModel.refreshDevices()
        assertEquals(devices, viewModel.devices.value)
    }

    @Test
    fun hasNoDevicesWithoutPermission() {
        val devices = listOf(BluetoothDevice("Test Device", "00:00:00:00:00:00"))
        every { deviceProvider.getDevices() } returns devices
        every { application.checkSelfPermission(any()) } returns PackageManager.PERMISSION_DENIED

        val viewModel = DeviceSelectionViewModel(application, deviceProvider)
        assertEquals(emptyList<BluetoothDevice>(), viewModel.devices.value)
    }
}
