package com.oppzippy.openscq30

import androidx.activity.ComponentActivity
import androidx.compose.ui.test.SemanticsMatcher
import androidx.compose.ui.test.hasTextExactly
import androidx.compose.ui.test.junit4.createAndroidComposeRule
import androidx.test.ext.junit.runners.AndroidJUnit4
import com.oppzippy.openscq30.soundcoredevice.SoundcoreDeviceFactory
import com.oppzippy.openscq30.ui.devicesettings.DeviceSettingsActivityView
import io.mockk.coEvery
import io.mockk.coJustAwait
import io.mockk.impl.annotations.MockK
import io.mockk.junit4.MockKRule
import org.junit.Assert
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class DeviceSettingsLoadingTest {
    @get:Rule
    val mockkRule = MockKRule(this)

    @get:Rule
    val composeRule = createAndroidComposeRule<ComponentActivity>()

    @MockK
    lateinit var deviceFactory: SoundcoreDeviceFactory

    private lateinit var loading: SemanticsMatcher

    @Before
    fun initialize() {
        loading = hasTextExactly(composeRule.activity.getString(R.string.loading))
    }

    @Test
    fun testWithNonexistentDevice() {
        coEvery { deviceFactory.createSoundcoreDevice(any()) } returns null

        var isOnDeviceNotFoundCalled = false
        composeRule.setContent {
            DeviceSettingsActivityView(
                macAddress = "",
                soundcoreDeviceFactory = deviceFactory,
                onDeviceNotFound = {
                    isOnDeviceNotFoundCalled = true
                },
            )
        }

        Assert.assertTrue(
            "onDeviceNotFound should have been called", isOnDeviceNotFoundCalled,
        )
        composeRule.onNode(loading).assertExists()
    }

    @Test
    fun testLoadingScreen() {
        coJustAwait { deviceFactory.createSoundcoreDevice(any()) }

        var isOnDeviceNotFoundCalled = false
        composeRule.setContent {
            DeviceSettingsActivityView(
                macAddress = "",
                soundcoreDeviceFactory = deviceFactory,
                onDeviceNotFound = {
                    isOnDeviceNotFoundCalled = true
                },
            )
        }

        Assert.assertFalse("onDeviceNotFound should not have been called", isOnDeviceNotFoundCalled)
        composeRule.onNode(loading).assertExists()
    }
}