package com.oppzippy.openscq30.ui.quickpresets

import com.oppzippy.openscq30.features.equalizer.storage.CustomProfileDao
import com.oppzippy.openscq30.features.quickpresets.storage.QuickPreset
import com.oppzippy.openscq30.features.quickpresets.storage.QuickPresetDao
import com.oppzippy.openscq30.test.MainDispatcherRule
import io.mockk.coEvery
import io.mockk.impl.annotations.MockK
import io.mockk.junit4.MockKRule
import kotlinx.coroutines.flow.MutableStateFlow
import org.junit.Assert.assertEquals
import org.junit.Before
import org.junit.Rule
import org.junit.Test

class QuickPresetsViewModelTest {
    @get:Rule
    val mockkRule = MockKRule(this)

    @get:Rule
    val mainDispatcherRule = MainDispatcherRule()

    @MockK
    lateinit var customProfileDao: CustomProfileDao

    @MockK
    lateinit var quickPresetDao: QuickPresetDao

    @Before
    fun setUp() {
        coEvery { quickPresetDao.get(0) } returns null
        coEvery { customProfileDao.allNames() } returns MutableStateFlow(emptyList())
    }

    @Test
    fun selectsDefaultQuickPreset() {
        val viewModel = QuickPresetViewModel(
            customProfileDao = customProfileDao,
            quickPresetDao = quickPresetDao,
        )
        assertEquals(0, viewModel.quickPreset.value?.id)
    }

    @Test
    fun selectsExistingQuickPreset() {
        val viewModel = QuickPresetViewModel(
            customProfileDao = customProfileDao,
            quickPresetDao = quickPresetDao,
        )

        coEvery { quickPresetDao.get(100) } returns QuickPreset(id = 100, name = "Quick Preset 100")
        viewModel.selectQuickPreset(100)
        assertEquals(100, viewModel.quickPreset.value?.id)
        assertEquals("Quick Preset 100", viewModel.quickPreset.value?.name)
    }

    @Test
    fun selectsNewQuickPreset() {
        val viewModel = QuickPresetViewModel(
            customProfileDao = customProfileDao,
            quickPresetDao = quickPresetDao,
        )

        coEvery { quickPresetDao.get(100) } returns null
        viewModel.selectQuickPreset(100)
        assertEquals(100, viewModel.quickPreset.value?.id)
    }
}
