package com.oppzippy.openscq30

import androidx.compose.ui.semantics.ProgressBarRangeInfo
import androidx.compose.ui.test.SemanticsMatcher
import androidx.compose.ui.test.assertCountEquals
import androidx.compose.ui.test.assertRangeInfoEquals
import androidx.compose.ui.test.assertTextContains
import androidx.compose.ui.test.hasTextExactly
import androidx.compose.ui.test.junit4.createAndroidComposeRule
import androidx.compose.ui.test.onAllNodesWithTag
import androidx.compose.ui.test.onAllNodesWithText
import androidx.compose.ui.test.onNodeWithContentDescription
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.performTextInput
import androidx.compose.ui.test.performTextReplacement
import androidx.compose.ui.test.performTouchInput
import androidx.compose.ui.test.swipe
import com.oppzippy.openscq30.extensions.empty
import com.oppzippy.openscq30.lib.bindings.AmbientSoundMode
import com.oppzippy.openscq30.lib.bindings.CustomNoiseCanceling
import com.oppzippy.openscq30.lib.bindings.EqualizerConfiguration
import com.oppzippy.openscq30.lib.bindings.NoiseCancelingMode
import com.oppzippy.openscq30.lib.bindings.PresetEqualizerProfile
import com.oppzippy.openscq30.lib.bindings.SoundModes
import com.oppzippy.openscq30.lib.bindings.TransparencyMode
import com.oppzippy.openscq30.lib.bindings.VolumeAdjustments
import com.oppzippy.openscq30.lib.wrapper.SoundcoreDeviceState
import com.oppzippy.openscq30.ui.devicesettings.models.UiDeviceState
import com.oppzippy.openscq30.ui.equalizer.EqualizerSettings
import dagger.hilt.android.testing.HiltAndroidRule
import dagger.hilt.android.testing.HiltAndroidTest
import io.mockk.junit4.MockKRule
import io.mockk.mockk
import io.mockk.verify
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import java.util.UUID

@HiltAndroidTest
class DeviceSettingsEqualizerTest {
    @get:Rule
    val mockkRule = MockKRule(this)

    @get:Rule(order = 1)
    val hiltRule = HiltAndroidRule(this)

    @get:Rule(order = 2)
    val composeRule = createAndroidComposeRule<TestActivity>()

    private lateinit var soundcoreSignature: SemanticsMatcher
    private lateinit var acoustic: SemanticsMatcher
    private lateinit var bassBooster: SemanticsMatcher
    private lateinit var classical: SemanticsMatcher
    private lateinit var custom: SemanticsMatcher

    @Before
    fun setUp() {
        hiltRule.inject()

        soundcoreSignature =
            hasTextExactly(composeRule.activity.getString(R.string.soundcore_signature))
        acoustic = hasTextExactly(composeRule.activity.getString(R.string.acoustic))
        bassBooster = hasTextExactly(composeRule.activity.getString(R.string.bass_booster))
        classical = hasTextExactly(composeRule.activity.getString(R.string.classical))
        custom = hasTextExactly(composeRule.activity.getString(R.string.custom))
    }

    private fun stateWithEqualizerConfiguration(equalizerConfiguration: EqualizerConfiguration): UiDeviceState.Connected {
        return UiDeviceState.Connected(
            "Test Device",
            "00:00:00:00:00:00",
            SoundcoreDeviceState.empty().copy(
                soundModes = SoundModes(
                    AmbientSoundMode.Normal,
                    NoiseCancelingMode.Indoor,
                    TransparencyMode.VocalMode,
                    CustomNoiseCanceling(0),
                ),
                equalizerConfiguration = equalizerConfiguration,
            ),
            deviceBleServiceUuid = UUID(0, 0),
        )
    }

    @Test
    fun selectsInitialEqualizerPresetProfileByDefault() {
        composeRule.setContent {
            EqualizerSettings(
                uiState = stateWithEqualizerConfiguration(
                    EqualizerConfiguration(PresetEqualizerProfile.Classical),
                ),
            )
        }

        composeRule.onNode(soundcoreSignature, true).assertDoesNotExist()
        composeRule.onNode(classical, true).assertExists()

        val sliders = composeRule.onAllNodesWithTag("equalizerSlider")
        val values = listOf(3.0F, 3.0F, -2.0F, -2.0F, 0.0F, 2.0F, 3.0F, 4.0F)
        for (i in 0..7) {
            sliders[i].assertRangeInfoEquals(ProgressBarRangeInfo(values[i], -12.0F..13.5F, 255))
        }
    }

    @Test
    fun selectsInitialEqualizerCustomProfileByDefault() {
        val values = doubleArrayOf(0.1, 1.0, -1.0, 5.0, 0.0, 1.0, -6.0, 6.0)

        composeRule.setContent {
            EqualizerSettings(
                uiState = stateWithEqualizerConfiguration(
                    EqualizerConfiguration(VolumeAdjustments(values)),
                ),
            )
        }
        composeRule.onNode(soundcoreSignature, true).assertDoesNotExist()
        composeRule.onNode(custom, true).assertExists()
        val sliders = composeRule.onAllNodesWithTag("equalizerSlider")
        for (i in 0..7) {
            sliders[i].assertRangeInfoEquals(
                ProgressBarRangeInfo(
                    values[i].toFloat(),
                    -12.0F..13.5F,
                    255,
                ),
            )
        }
    }

    @Test
    fun changesBetweenPresetProfiles() {
        val onEqualizerConfigurationChange =
            mockk<(equalizerConfiguration: EqualizerConfiguration) -> Unit>(relaxed = true)
        composeRule.setContent {
            EqualizerSettings(
                uiState = stateWithEqualizerConfiguration(
                    EqualizerConfiguration(PresetEqualizerProfile.Acoustic),
                ),
                onEqualizerConfigurationChange = onEqualizerConfigurationChange,
            )
        }
        composeRule.onNode(acoustic, true).performClick()
        composeRule.onNode(bassBooster, true).performClick()
        Thread.sleep(600) // Wait for debounce
        verify {
            onEqualizerConfigurationChange(EqualizerConfiguration(PresetEqualizerProfile.BassBooster))
        }

        val values = floatArrayOf(4.0F, 3.0F, 1.0F, 0.0F, 0.0F, 0.0F, 0.0F, 0.0F)
        val sliders = composeRule.onAllNodesWithTag("equalizerSlider")
        for (i in 0..7) {
            sliders[i].assertRangeInfoEquals(
                ProgressBarRangeInfo(
                    values[i],
                    -12.0F..13.5F,
                    255,
                ),
            )
        }
    }

    @Test
    fun selectsCustomPresetProfileWhenMovingSliders() {
        val onEqualizerConfigurationChange =
            mockk<(equalizerConfiguration: EqualizerConfiguration) -> Unit>(relaxed = true)

        composeRule.setContent {
            EqualizerSettings(
                uiState = stateWithEqualizerConfiguration(
                    EqualizerConfiguration(PresetEqualizerProfile.SoundcoreSignature),
                ),
                onEqualizerConfigurationChange = onEqualizerConfigurationChange,
            )
        }

        val sliders = composeRule.onAllNodesWithTag("equalizerSlider")
        sliders[0].performTouchInput {
            swipe(center, centerRight, 100)
        }
        composeRule.onNode(custom, useUnmergedTree = true).assertExists()

        Thread.sleep(600) // Wait for debounce
        verify {
            onEqualizerConfigurationChange(
                match {
                    it.volumeAdjustments().adjustments().first() > 0
                },
            )
        }
    }

    @Test
    fun selectsSpecificCustomProfileWhenVolumeAdjustmentsMatch() {
        composeRule.setContent {
            EqualizerSettings(
                uiState = stateWithEqualizerConfiguration(
                    EqualizerConfiguration(
                        VolumeAdjustments(doubleArrayOf(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)),
                    ),
                ),
            )
        }

        composeRule.onNodeWithContentDescription(composeRule.activity.getString(R.string.add))
            .performClick()
        composeRule.onNodeWithText(composeRule.activity.getString(R.string.name))
            .performTextInput("Test Profile")
        composeRule.onNodeWithText(composeRule.activity.getString(R.string.create)).performClick()
        composeRule.onNodeWithText("Test Profile")
            .assertExists("custom profile should be selected upon creation")

        val inputs = composeRule.onAllNodesWithTag("equalizerInput")
        inputs[0].performTextReplacement("6")
        composeRule.onNodeWithText("Test Profile").assertDoesNotExist()
        inputs[0].performTextReplacement("0")
        composeRule.onNodeWithText("Test Profile")
            .assertExists("custom profile should be selected when equalizer values change to match the custom profile")
        composeRule.onNodeWithContentDescription(composeRule.activity.getString(R.string.delete))
            .performClick()
        composeRule.onNodeWithText(composeRule.activity.getString(R.string.delete)).performClick()
        composeRule.onNodeWithText("Test Profile").assertDoesNotExist()
    }

    @Test
    fun overwritesCustomProfileWhenCreatingUsingExistingName() {
        composeRule.setContent {
            EqualizerSettings(
                uiState = stateWithEqualizerConfiguration(
                    EqualizerConfiguration(
                        VolumeAdjustments(doubleArrayOf(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)),
                    ),
                ),
            )
        }

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
    fun doesNotAllowCreatingMultipleCustomProfilesWithTheSameVolumeAdjustments() {
        composeRule.setContent {
            EqualizerSettings(
                stateWithEqualizerConfiguration(
                    EqualizerConfiguration(
                        VolumeAdjustments(doubleArrayOf(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)),
                    ),
                ),
            )
        }

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
    fun replacesExistingCustomProfilesUsingReplaceButton() {
        composeRule.setContent {
            EqualizerSettings(
                stateWithEqualizerConfiguration(
                    EqualizerConfiguration(
                        VolumeAdjustments(doubleArrayOf(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)),
                    ),
                ),
            )
        }

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
    fun doesNotShowReplaceExistingCustomProfileButtonWithNoCustomProfiles() {
        composeRule.setContent {
            EqualizerSettings(
                stateWithEqualizerConfiguration(
                    EqualizerConfiguration(
                        VolumeAdjustments(doubleArrayOf(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)),
                    ),
                ),
            )
        }

        composeRule.onNodeWithContentDescription(composeRule.activity.getString(R.string.replace_existing_profile))
            .assertDoesNotExist()
    }

    @Test
    fun doesNotSelectBothPresetAndCustomProfile() {
        composeRule.setContent {
            EqualizerSettings(
                stateWithEqualizerConfiguration(
                    EqualizerConfiguration(
                        VolumeAdjustments(doubleArrayOf(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)),
                    ),
                ),
            )
        }

        // Create custom profile with same volume adjustments as Soundcore Signature
        composeRule.onNodeWithContentDescription(composeRule.activity.getString(R.string.add))
            .performClick()
        composeRule.onNodeWithText(composeRule.activity.getString(R.string.name))
            .performTextInput("Test Profile 1")
        composeRule.onNodeWithText(composeRule.activity.getString(R.string.create)).performClick()

        // Soundcore Signature should not become selected
        composeRule.onNode(soundcoreSignature, true).assertDoesNotExist()
        composeRule.onNodeWithText("Test Profile 1").assertExists()

        // Select Soundcore Signature and ensure the custom profile no longer is selected
        composeRule.onNode(custom, true).performClick()
        composeRule.onNode(soundcoreSignature, true).performClick()
        composeRule.onNode(soundcoreSignature, true).assertExists()
        composeRule.onNode(custom, true).assertDoesNotExist()
    }

    @Test
    fun hidesCreateAndDeleteButtonsWhenPresetIsSelected() {
        composeRule.setContent {
            EqualizerSettings(
                stateWithEqualizerConfiguration(
                    EqualizerConfiguration(
                        VolumeAdjustments(doubleArrayOf(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)),
                    ),
                ),
            )
        }

        composeRule.onNodeWithText(composeRule.activity.getString(R.string.replace))
            .assertDoesNotExist()
        composeRule.onNodeWithText(composeRule.activity.getString(R.string.create))
            .assertDoesNotExist()
        composeRule.onNodeWithText(composeRule.activity.getString(R.string.delete))
            .assertDoesNotExist()
    }
}
