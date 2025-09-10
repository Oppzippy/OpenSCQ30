package com.oppzippy.openscq30.ui

import android.content.Intent
import com.oppzippy.openscq30.OpenSCQ30Application
import com.oppzippy.openscq30.android.IntentFactory
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceService
import com.oppzippy.openscq30.lib.bindings.OpenScq30Session
import com.oppzippy.openscq30.lib.wrapper.ConnectionDescriptor
import io.mockk.coEvery
import io.mockk.every
import io.mockk.impl.annotations.MockK
import io.mockk.impl.annotations.RelaxedMockK
import io.mockk.junit4.MockKRule
import io.mockk.mockk
import io.mockk.verify
import org.junit.Rule
import org.junit.Test

class DeviceSettingsManagerTest {
    @get:Rule
    val mockkRule = MockKRule(this)

    @RelaxedMockK
    lateinit var application: OpenSCQ30Application

    @MockK
    lateinit var intentFactory: IntentFactory

    @Test
    fun startsServiceWhenSelectingDevice() {
        val mockIntent: Intent = mockk(relaxed = true)
        every { intentFactory(any(), any()) } returns mockIntent

        val viewModel = OpenSCQ30RootViewModel(
            session = mockk<OpenScq30Session>().apply {
                coEvery { listDemoDevices(any()) } returns listOf(ConnectionDescriptor("Test", "00:00:00:00:00:00"))
            },
            quickPresetSlotDao = mockk(relaxed = true),
            legacyEqualizerProfileDao = mockk(relaxed = true),
            featuredSettingSlotDao = mockk(relaxed = true),
            intentFactory = intentFactory,
            application = application,
            version2BreakingChangesMessage = mockk(),
        )

        viewModel.selectDevice("00:00:00:00:00:00")
        verify { mockIntent.putExtra(DeviceService.MAC_ADDRESS, "00:00:00:00:00:00") }
        verify(exactly = 1) { application.startForegroundService(mockIntent) }
        verify(atMost = 2) { application.bindService(mockIntent, any(), any() as Int) }
    }

    @Test
    fun stopsServiceWhenDeselectingDevice() {
        every { intentFactory(application, DeviceService::class.java) } returns mockk()

        val viewModel = OpenSCQ30RootViewModel(
            session = mockk(relaxed = true),
            quickPresetSlotDao = mockk(relaxed = true),
            legacyEqualizerProfileDao = mockk(relaxed = true),
            featuredSettingSlotDao = mockk(relaxed = true),
            intentFactory = intentFactory,
            application = application,
            version2BreakingChangesMessage = mockk(),
        )
        viewModel.deselectDevice()
        verify(exactly = 1) { application.stopService(any()) }
        verify(exactly = 1) { application.unbindService(any()) }
    }
}
