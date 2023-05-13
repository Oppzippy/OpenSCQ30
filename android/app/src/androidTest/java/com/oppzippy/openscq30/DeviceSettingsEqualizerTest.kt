package com.oppzippy.openscq30

import androidx.compose.ui.semantics.ProgressBarRangeInfo
import androidx.compose.ui.test.*
import androidx.compose.ui.test.junit4.createAndroidComposeRule
import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDevice
import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDeviceFactory
import com.oppzippy.openscq30.features.soundcoredevice.api.contentEquals
import com.oppzippy.openscq30.features.ui.devicesettings.composables.DeviceSettingsActivityView
import com.oppzippy.openscq30.lib.*
import dagger.hilt.android.testing.HiltAndroidRule
import dagger.hilt.android.testing.HiltAndroidTest
import io.mockk.coEvery
import io.mockk.every
import io.mockk.junit4.MockKRule
import io.mockk.mockk
import io.mockk.verify
import kotlinx.coroutines.flow.MutableStateFlow
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import javax.inject.Inject

@HiltAndroidTest
class DeviceSettingsEqualizerTest {
    @get:Rule
    val mockkRule = MockKRule(this)

    @get:Rule(order = 1)
    val hiltRule = HiltAndroidRule(this)

    @get:Rule(order = 2)
    val composeRule = createAndroidComposeRule<MainActivity>()

    @Inject
    lateinit var deviceFactory: SoundcoreDeviceFactory

    private lateinit var equalizer: SemanticsMatcher
    private lateinit var soundcoreSignature: SemanticsMatcher
    private lateinit var acoustic: SemanticsMatcher
    private lateinit var bassBooster: SemanticsMatcher
    private lateinit var classical: SemanticsMatcher
    private lateinit var custom: SemanticsMatcher

    @Before
    fun initialize() {
        hiltRule.inject()

        equalizer = hasTextExactly(composeRule.activity.getString(R.string.equalizer))
        soundcoreSignature =
            hasTextExactly(composeRule.activity.getString(R.string.soundcore_signature))
        acoustic = hasTextExactly(composeRule.activity.getString(R.string.acoustic))
        bassBooster = hasTextExactly(composeRule.activity.getString(R.string.bass_booster))
        classical = hasTextExactly(composeRule.activity.getString(R.string.classical))
        custom = hasTextExactly(composeRule.activity.getString(R.string.custom))
    }

    @Test
    fun testInitialEqualizerPreset() {
        initializeDeviceFactoryWithOneDevice(
            equalizerConfiguration = EqualizerConfiguration(
                PresetEqualizerProfile.Classical,
            )
        )


        composeRule.setContent {
            DeviceSettingsActivityView(
                macAddress = "",
                onDeviceNotFound = {},
            )
        }
        composeRule.onNode(equalizer, true).performClick()
        composeRule.onNode(soundcoreSignature, true).assertDoesNotExist()
        composeRule.onNode(classical, true).assertExists()
        val sliders = composeRule.onAllNodesWithTag("equalizerSlider")
        val values = listOf(30F, 30F, -20F, -20F, 0F, 20F, 30F, 40F)
        for (i in 0..7) {
            sliders[i].assertRangeInfoEquals(ProgressBarRangeInfo(values[i], -120F..120F, 240))
        }
    }

    @Test
    fun testInitialEqualizerCustom() {
        val values = byteArrayOf(1, 10, -10, 50, 0, 10, -60, 60)

        initializeDeviceFactoryWithOneDevice(
            equalizerConfiguration = EqualizerConfiguration(VolumeAdjustments(values)),
        )

        composeRule.setContent {
            DeviceSettingsActivityView(
                macAddress = "",
                onDeviceNotFound = {},
            )
        }
        composeRule.onNode(equalizer, true).performClick()
        composeRule.onNode(soundcoreSignature, true).assertDoesNotExist()
        composeRule.onNode(custom, true).assertExists()
        val sliders = composeRule.onAllNodesWithTag("equalizerSlider")
        // The 8th slider doesn't fit on the screen, so we would have to scroll to it
        // TODO scroll down to sliders that don't fit on the screen
        for (i in 0..6) {
            sliders[i].assertRangeInfoEquals(
                ProgressBarRangeInfo(
                    values[i].toFloat(), -120F..120F, 240,
                )
            )
        }
    }

    @Test
    fun testSetPreset() {
        val pair = initializeDeviceFactoryWithOneDevice(
            equalizerConfiguration = EqualizerConfiguration(PresetEqualizerProfile.Acoustic),
        )
        val device = pair.first
        val state = pair.second

        composeRule.setContent {
            DeviceSettingsActivityView(
                macAddress = "",
                onDeviceNotFound = {},
            )
        }
        composeRule.onNode(equalizer, true).performClick()
        composeRule.onNode(acoustic, true).performClick()
        composeRule.onNode(bassBooster, true).performClick()
        Thread.sleep(600) // Wait for debounce
        verify {
            device.setEqualizerConfiguration(match {
                it.contentEquals(
                    EqualizerConfiguration(PresetEqualizerProfile.BassBooster)
                )
            })
        }
        every { state.equalizerConfiguration() } returns EqualizerConfiguration(
            PresetEqualizerProfile.BassBooster
        )

        val values = byteArrayOf(40, 30, 10, 0, 0, 0, 0, 0)
        val sliders = composeRule.onAllNodesWithTag("equalizerSlider")
        for (i in 0..7) {
            sliders[i].assertRangeInfoEquals(
                ProgressBarRangeInfo(
                    values[i].toFloat(), -120F..120F, 240
                )
            )
        }
    }

    @Test
    fun testSetCustom() {
        val values = byteArrayOf(0, 10, 15, -15, 60, -60, 10, -5)
        val pair = initializeDeviceFactoryWithOneDevice(
            equalizerConfiguration = EqualizerConfiguration(
                VolumeAdjustments(values),
            ),
        )
        val device = pair.first

        composeRule.setContent {
            DeviceSettingsActivityView(
                macAddress = "",
                onDeviceNotFound = {},
            )
        }
        composeRule.onNode(equalizer).performClick()

        val sliders = composeRule.onAllNodesWithTag("equalizerSlider")
        sliders[0].performTouchInput {
            swipe(center, centerRight, 100)
        }

        Thread.sleep(600) // Wait for debounce
        verify {
            device.setEqualizerConfiguration(match {
                it.volumeAdjustments().adjustments().first() > 0
            })
        }
    }

    @Test
    fun testCustomProfile() {
        initializeDeviceFactoryWithOneDevice(
            equalizerConfiguration = EqualizerConfiguration(
                VolumeAdjustments(byteArrayOf(0, 0, 0, 0, 0, 0, 0, 0))
            ),
        )
        composeRule.setContent {
            DeviceSettingsActivityView(macAddress = "", onDeviceNotFound = {})
        }
        composeRule.onNode(equalizer).performClick()

        composeRule.onNodeWithContentDescription(composeRule.activity.getString(R.string.add))
            .performClick()
        composeRule.onNodeWithText(composeRule.activity.getString(R.string.name))
            .performTextInput("Test Profile")
        composeRule.onNodeWithText(composeRule.activity.getString(R.string.create)).performClick()
        composeRule.onNodeWithText("Test Profile")
            .assertExists("custom profile should be selected upon creation");

        val inputs = composeRule.onAllNodesWithTag("equalizerInput")
        inputs[0].performTextReplacement("6")
        composeRule.onNodeWithText("Test Profile").assertDoesNotExist();
        inputs[0].performTextReplacement("0")
        composeRule.onNodeWithText("Test Profile")
            .assertExists("custom profile should be selected when equalizer values change to match the custom profile");
        composeRule.onNodeWithContentDescription(composeRule.activity.getString(R.string.delete))
            .performClick()
        composeRule.onNodeWithText(composeRule.activity.getString(R.string.delete)).performClick()
        composeRule.onNodeWithText("Test Profile").assertDoesNotExist()
    }

    @Test
    fun testCustomProfileUniqueByName() {
        initializeDeviceFactoryWithOneDevice(
            equalizerConfiguration = EqualizerConfiguration(
                VolumeAdjustments(byteArrayOf(0, 0, 0, 0, 0, 0, 0, 0))
            ),
        )
        composeRule.setContent {
            DeviceSettingsActivityView(macAddress = "", onDeviceNotFound = {})
        }
        composeRule.onNode(equalizer).performClick()

        // Create first profile
        composeRule.onNodeWithContentDescription(composeRule.activity.getString(R.string.add))
            .performClick()
        composeRule.onNodeWithText(composeRule.activity.getString(R.string.name))
            .performTextInput("Test Profile")
        composeRule.onNodeWithText(composeRule.activity.getString(R.string.create)).performClick()

        // Create second profile
        val inputs = composeRule.onAllNodesWithTag("equalizerInput")
        inputs[0].performTextReplacement("6")
        composeRule.onNodeWithContentDescription(composeRule.activity.getString(R.string.add))
            .performClick()
        composeRule.onNodeWithText(composeRule.activity.getString(R.string.name))
            .performTextInput("Test Profile")
        composeRule.onNodeWithText(composeRule.activity.getString(R.string.replace)).performClick()

        // Open dropdown and make sure there is only one
        composeRule.onNodeWithText("Test Profile").performClick()
        // 1 for the text that lists current selection, 1 for the item in the dropdown
        composeRule.onAllNodesWithText("Test Profile").assertCountEquals(2)

        composeRule.onNodeWithContentDescription(composeRule.activity.getString(R.string.delete))
            .performClick()
        composeRule.onNodeWithText(composeRule.activity.getString(R.string.delete)).performClick()
        composeRule.onNodeWithText("Test Profile").assertDoesNotExist()
    }

    @Test
    fun testCustomProfileUniqueByValues() {
        initializeDeviceFactoryWithOneDevice(
            equalizerConfiguration = EqualizerConfiguration(
                VolumeAdjustments(byteArrayOf(0, 0, 0, 0, 0, 0, 0, 0))
            ),
        )
        composeRule.setContent {
            DeviceSettingsActivityView(macAddress = "", onDeviceNotFound = {})
        }
        composeRule.onNode(equalizer).performClick()

        // Create first profile
        composeRule.onNodeWithContentDescription(composeRule.activity.getString(R.string.add))
            .performClick()
        composeRule.onNodeWithText(composeRule.activity.getString(R.string.name))
            .performTextInput("Test Profile 1")
        composeRule.onNodeWithText(composeRule.activity.getString(R.string.create)).performClick()

        // The button to create another profile shouldn't even be visible
        composeRule.onNodeWithText("Delete").assertDoesNotExist()
    }

    @Test
    fun testReplaceExistingCustomProfile() {
        initializeDeviceFactoryWithOneDevice(
            equalizerConfiguration = EqualizerConfiguration(
                VolumeAdjustments(byteArrayOf(0, 0, 0, 0, 0, 0, 0, 0)),
            )
        )

        composeRule.setContent {
            DeviceSettingsActivityView(macAddress = "", onDeviceNotFound = {})
        }
        composeRule.onNode(equalizer).performClick()

        // Create a profile
        composeRule.onNodeWithContentDescription(composeRule.activity.getString(R.string.add))
            .performClick()
        composeRule.onNodeWithText(composeRule.activity.getString(R.string.name))
            .performTextInput("Test Profile 1")
        composeRule.onNodeWithText(composeRule.activity.getString(R.string.create)).performClick()

        // Adjust values
        val inputs = composeRule.onAllNodesWithTag("equalizerInput")
        inputs[0].performTextReplacement("2.3")

        // Replace the old profile
        composeRule.onNodeWithContentDescription(composeRule.activity.getString(R.string.replace_existing_profile))
            .performClick()
        composeRule.onNodeWithText("Test Profile 1").performClick()

        // Make sure dialog is closed
        composeRule.onNodeWithText(composeRule.activity.getString(R.string.replace_existing_profile))
            .assertDoesNotExist()
        composeRule.onNodeWithText("Test Profile 1").assertExists()
        // Make sure the values were not changed
        inputs[0].assertTextContains("2.3")
    }

    @Test
    fun testReplaceExistingCustomProfileButtonDoesNotShowWithNoCustomProfiles() {
        initializeDeviceFactoryWithOneDevice(
            equalizerConfiguration = EqualizerConfiguration(
                VolumeAdjustments(byteArrayOf(0, 0, 0, 0, 0, 0, 0, 0)),
            )
        )

        composeRule.setContent {
            DeviceSettingsActivityView(macAddress = "", onDeviceNotFound = {})
        }
        composeRule.onNode(equalizer).performClick()

        composeRule.onNodeWithContentDescription(composeRule.activity.getString(R.string.replace_existing_profile))
            .assertDoesNotExist()
    }

    private fun initializeDeviceFactoryWithOneDevice(equalizerConfiguration: EqualizerConfiguration): Pair<SoundcoreDevice, SoundcoreDeviceState> {
        val device = mockk<SoundcoreDevice>()
        val state = mockk<SoundcoreDeviceState>()
        val stateFlow = MutableStateFlow(state)

        coEvery { deviceFactory.createSoundcoreDevice(any(), any()) } returns device
        every { device.name } returns "Test Q30"
        every { device.macAddress } returns "00:00:00:00:00:00"
        every { device.state } returns state
        every { device.stateFlow } returns stateFlow
        every { device.setEqualizerConfiguration(any()) } returns Unit
        every { device.destroy() } returns Unit
        every { state.ambientSoundMode() } returns AmbientSoundMode.Normal
        every { state.noiseCancelingMode() } returns NoiseCancelingMode.Transport
        every { state.equalizerConfiguration() } returns equalizerConfiguration

        return Pair(device, state)
    }
}
