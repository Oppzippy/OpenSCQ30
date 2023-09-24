package com.oppzippy.openscq30.ui

import android.content.Intent
import com.oppzippy.openscq30.OpenSCQ30Application
import com.oppzippy.openscq30.android.IntentFactory
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDevice
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDeviceProvider
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceService
import io.mockk.every
import io.mockk.impl.annotations.MockK
import io.mockk.impl.annotations.RelaxedMockK
import io.mockk.junit4.MockKRule
import io.mockk.mockk
import io.mockk.verify
import org.junit.Rule
import org.junit.Test

class DeviceSettingsViewModelTest {
    @get:Rule
    val mockkRule = MockKRule(this)

    @RelaxedMockK
    lateinit var application: OpenSCQ30Application

    @RelaxedMockK
    lateinit var deviceProvider: BluetoothDeviceProvider

    @MockK
    lateinit var intentFactory: IntentFactory

    @Test
    fun startsServiceWhenSelectingDevice() {
        val mockIntent: Intent = mockk(relaxed = true)
        every { intentFactory(any(), any()) } returns mockIntent

        val device = BluetoothDevice("Test Device", "00:00:00:00:00:00")
        every { deviceProvider.getDevices() } returns listOf(device)
        val viewModel = DeviceSettingsViewModel(application, intentFactory)

        viewModel.selectDevice(device)
        verify { mockIntent.putExtra(DeviceService.MAC_ADDRESS, "00:00:00:00:00:00") }
        verify(exactly = 1) { application.startForegroundService(mockIntent) }
        verify(atMost = 2) { application.bindService(mockIntent, any(), any() as Int) }
    }

    @Test
    fun stopsServiceWhenDeselectingDevice() {
        every { intentFactory(application, DeviceService::class.java) } returns mockk()

        val viewModel = DeviceSettingsViewModel(application, intentFactory)
        viewModel.deselectDevice()
        verify(exactly = 1) { application.stopService(any()) }
        verify(exactly = 1) { application.unbindService(any()) }
    }
}
