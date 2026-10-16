package com.oppzippy.openscq30.ui

import android.util.Log
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceConnectionManager
import com.oppzippy.openscq30.features.statusnotification.storage.QuickPresetSlotDao
import com.oppzippy.openscq30.lib.bindings.OpenScq30Device
import com.oppzippy.openscq30.lib.bindings.OpenScq30Exception
import com.oppzippy.openscq30.lib.bindings.OpenScq30Session
import com.oppzippy.openscq30.lib.bindings.QuickPresetsHandler
import com.oppzippy.openscq30.ui.utils.ToastHandler
import io.mockk.coEvery
import io.mockk.coJustRun
import io.mockk.coVerify
import io.mockk.every
import io.mockk.mockk
import io.mockk.mockkStatic
import io.mockk.unmockkStatic
import kotlinx.coroutines.Job
import org.junit.After
import org.junit.Before
import org.junit.Test

class DeleteQuickPresetTest {
    private val device = mockk<OpenScq30Device>()
    private val quickPresetsHandler = mockk<QuickPresetsHandler>()
    private val quickPresetSlotDao = mockk<QuickPresetSlotDao>(relaxed = true)
    private lateinit var manager: DeviceSettingsManager

    @Before
    fun setUp() {
        mockkStatic(Log::class)
        every { Log.e(any<String>(), any<String>(), any<Throwable>()) } returns 1

        every { device.model() } returns MODEL
        coEvery { quickPresetsHandler.quickPresets(any()) } returns emptyList()

        val session = mockk<OpenScq30Session>()
        every { session.quickPresetHandler() } returns quickPresetsHandler
        val deviceManager = mockk<DeviceConnectionManager>()
        every { deviceManager.device } returns device

        manager = DeviceSettingsManager(
            session = session,
            parent = Job(),
            deviceManager = deviceManager,
            quickPresetSlotDao = quickPresetSlotDao,
            featuredSettingSlotDao = mockk(relaxed = true),
            legacyEqualizerProfileDao = mockk(relaxed = true),
            toastHandler = ToastHandler(),
        )
    }

    @After
    fun tearDown() {
        manager.close()
        unmockkStatic(Log::class)
    }

    @Test
    fun deletingQuickPresetClearsNotificationSlotsThatReferToIt() {
        coJustRun { quickPresetsHandler.delete(any(), any()) }

        manager.deleteQuickPreset("My Preset")

        coVerify(timeout = TIMEOUT_MS) { quickPresetSlotDao.deleteByName(MODEL, "My Preset") }
    }

    @Test
    fun failingToDeleteQuickPresetKeepsNotificationSlots() {
        coEvery { quickPresetsHandler.delete(any(), any()) } throws OpenScq30Exception.StorageException("failed")

        manager.deleteQuickPreset("My Preset")

        // Presets are fetched once on creation and once more after the failed delete
        coVerify(timeout = TIMEOUT_MS, atLeast = 2) { quickPresetsHandler.quickPresets(any()) }
        coVerify(exactly = 0) { quickPresetSlotDao.deleteByName(any(), any()) }
    }

    companion object {
        private const val MODEL = "SoundcoreA3028"
        private const val TIMEOUT_MS = 5000L
    }
}
