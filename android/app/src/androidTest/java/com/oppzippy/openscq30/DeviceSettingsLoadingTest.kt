package com.oppzippy.openscq30

import androidx.compose.ui.test.SemanticsMatcher
import androidx.compose.ui.test.hasTextExactly
import androidx.compose.ui.test.junit4.createAndroidComposeRule
import com.oppzippy.openscq30.features.soundcoredevice.SoundcoreDeviceFactory
import com.oppzippy.openscq30.features.ui.devicesettings.composables.DeviceSettingsActivityView
import dagger.hilt.android.testing.HiltAndroidRule
import dagger.hilt.android.testing.HiltAndroidTest
import io.mockk.coEvery
import io.mockk.coJustAwait
import io.mockk.junit4.MockKRule
import org.junit.Assert
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import javax.inject.Inject

@HiltAndroidTest
class DeviceSettingsLoadingTest {
    @get:Rule
    val mockkRule = MockKRule(this)

    @get:Rule(order = 1)
    val hiltRule = HiltAndroidRule(this)

    @get:Rule(order = 2)
    val composeRule = createAndroidComposeRule<MainActivity>()

    @Inject
    lateinit var deviceFactory: SoundcoreDeviceFactory

    private lateinit var loading: SemanticsMatcher

    @Before
    fun initialize() {
        hiltRule.inject()

        loading = hasTextExactly(composeRule.activity.getString(R.string.loading))
    }

    @Test
    fun testWithNonexistentDevice() {
        coEvery { deviceFactory.createSoundcoreDevice(any(), any()) } returns null

        var isOnDeviceNotFoundCalled = false
        composeRule.setContent {
            DeviceSettingsActivityView(
                macAddress = "",
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
        coJustAwait { deviceFactory.createSoundcoreDevice(any(), any()) }

        var isOnDeviceNotFoundCalled = false
        composeRule.setContent {
            DeviceSettingsActivityView(
                macAddress = "",
                onDeviceNotFound = {
                    isOnDeviceNotFoundCalled = true
                },
            )
        }

        Assert.assertFalse("onDeviceNotFound should not have been called", isOnDeviceNotFoundCalled)
        composeRule.onNode(loading).assertExists()
    }
}