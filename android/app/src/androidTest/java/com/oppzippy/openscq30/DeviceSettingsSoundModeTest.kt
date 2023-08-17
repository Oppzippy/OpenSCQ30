package com.oppzippy.openscq30

import androidx.compose.ui.test.SemanticsMatcher
import androidx.compose.ui.test.assertIsNotSelected
import androidx.compose.ui.test.assertIsSelected
import androidx.compose.ui.test.hasTestTag
import androidx.compose.ui.test.hasTextExactly
import androidx.compose.ui.test.junit4.createAndroidComposeRule
import androidx.compose.ui.test.performClick
import com.oppzippy.openscq30.lib.bindings.AmbientSoundMode
import com.oppzippy.openscq30.lib.bindings.CustomNoiseCanceling
import com.oppzippy.openscq30.lib.bindings.NoiseCancelingMode
import com.oppzippy.openscq30.lib.bindings.SoundModes
import com.oppzippy.openscq30.lib.bindings.TransparencyMode
import com.oppzippy.openscq30.ui.soundmode.NoiseCancelingType
import com.oppzippy.openscq30.ui.soundmode.SoundModeSettings
import dagger.hilt.android.testing.HiltAndroidRule
import dagger.hilt.android.testing.HiltAndroidTest
import io.mockk.junit4.MockKRule
import io.mockk.mockk
import io.mockk.verify
import org.junit.Before
import org.junit.Rule
import org.junit.Test

@HiltAndroidTest
class DeviceSettingsSoundModeTest {
    @get:Rule
    val mockkRule = MockKRule(this)

    @get:Rule(order = 1)
    val hiltRule = HiltAndroidRule(this)

    @get:Rule(order = 2)
    val composeRule = createAndroidComposeRule<TestActivity>()

    private lateinit var ambientSoundModes: List<SemanticsMatcher>
    private lateinit var normal: SemanticsMatcher
    private lateinit var transparency: SemanticsMatcher
    private lateinit var noiseCanceling: SemanticsMatcher
    private lateinit var noiseCancelingModes: List<SemanticsMatcher>
    private lateinit var outdoor: SemanticsMatcher
    private lateinit var indoor: SemanticsMatcher
    private lateinit var custom: SemanticsMatcher
    private lateinit var transport: SemanticsMatcher
    private lateinit var fullyTransparent: SemanticsMatcher
    private lateinit var vocalMode: SemanticsMatcher
    private lateinit var customNoiseCanceling: SemanticsMatcher

    @Before
    fun setUp() {
        hiltRule.inject()

        normal = hasTextExactly(composeRule.activity.getString(R.string.normal))
        transparency = hasTextExactly(composeRule.activity.getString(R.string.transparency))
        noiseCanceling = hasTextExactly(composeRule.activity.getString(R.string.noise_canceling))
        outdoor = hasTextExactly(composeRule.activity.getString(R.string.outdoor))
        indoor = hasTextExactly(composeRule.activity.getString(R.string.indoor))
        custom = hasTextExactly(composeRule.activity.getString(R.string.custom))
        transport = hasTextExactly(composeRule.activity.getString(R.string.transport))
        fullyTransparent =
            hasTextExactly(composeRule.activity.getString(R.string.fully_transparent))
        vocalMode = hasTextExactly(composeRule.activity.getString(R.string.vocal_mode))
        customNoiseCanceling = hasTestTag("customNoiseCancelingSlider")
        ambientSoundModes = listOf(normal, transparency, noiseCanceling)
        noiseCancelingModes = listOf(outdoor, indoor, transport, custom)
    }

    @Test
    fun loadsInitialSoundModeNormalTransport() {
        renderInitialSoundMode(AmbientSoundMode.Normal, NoiseCancelingMode.Transport)
        assertOneSelected(normal, ambientSoundModes)
        assertOneSelected(transport, noiseCancelingModes)
    }

    @Test
    fun loadsInitialSoundModeNoiseCancelingOutdoor() {
        renderInitialSoundMode(AmbientSoundMode.NoiseCanceling, NoiseCancelingMode.Outdoor)
        assertOneSelected(noiseCanceling, ambientSoundModes)
        assertOneSelected(outdoor, noiseCancelingModes)
    }

    @Test
    fun setsAmbientSoundMode() {
        val onAmbientSoundModeChange =
            mockk<(ambientSoundMode: AmbientSoundMode) -> Unit>(relaxed = true)

        composeRule.setContent {
            SoundModeSettings(
                soundModes = SoundModes(
                    AmbientSoundMode.Normal,
                    NoiseCancelingMode.Indoor,
                    TransparencyMode.VocalMode,
                    CustomNoiseCanceling(0),
                ),
                onAmbientSoundModeChange = onAmbientSoundModeChange,
                hasTransparencyModes = false,
                noiseCancelingType = NoiseCancelingType.None,
            )
        }
        composeRule.onNode(transparency).performClick()
        verify(exactly = 1) {
            onAmbientSoundModeChange(AmbientSoundMode.Transparency)
        }
    }

    @Test
    fun setsTransparencyMode() {
        val onTransparencyModeChange =
            mockk<(transparencyMode: TransparencyMode) -> Unit>(relaxed = true)

        composeRule.setContent {
            SoundModeSettings(
                soundModes = SoundModes(
                    AmbientSoundMode.Normal,
                    NoiseCancelingMode.Indoor,
                    TransparencyMode.VocalMode,
                    CustomNoiseCanceling(0),
                ),
                onTransparencyModeChange = onTransparencyModeChange,
                hasTransparencyModes = true,
                noiseCancelingType = NoiseCancelingType.None,
            )
        }
        composeRule.onNode(fullyTransparent).performClick()
        verify(exactly = 1) {
            onTransparencyModeChange(TransparencyMode.FullyTransparent)
        }
    }

    @Test
    fun setsNoiseCancelingMode() {
        val onNoiseCancelingModeChange =
            mockk<(noiseCancelingMode: NoiseCancelingMode) -> Unit>(relaxed = true)

        composeRule.setContent {
            SoundModeSettings(
                soundModes = SoundModes(
                    AmbientSoundMode.Normal,
                    NoiseCancelingMode.Indoor,
                    TransparencyMode.VocalMode,
                    CustomNoiseCanceling(0),
                ),
                onNoiseCancelingModeChange = onNoiseCancelingModeChange,
                hasTransparencyModes = true,
                noiseCancelingType = NoiseCancelingType.Custom,
            )
        }
        composeRule.onNode(outdoor).performClick()
        verify(exactly = 1) {
            onNoiseCancelingModeChange(NoiseCancelingMode.Outdoor)
        }
    }

    @Test
    fun setsCustomNoiseCanceling() {
        val onCustomNoiseCancelingChange =
            mockk<(customNoiseCanceling: CustomNoiseCanceling) -> Unit>(relaxed = true)

        composeRule.setContent {
            SoundModeSettings(
                soundModes = SoundModes(
                    AmbientSoundMode.Normal,
                    NoiseCancelingMode.Custom,
                    TransparencyMode.VocalMode,
                    CustomNoiseCanceling(0),
                ),
                onCustomNoiseCancelingChange = onCustomNoiseCancelingChange,
                hasTransparencyModes = false,
                noiseCancelingType = NoiseCancelingType.Custom,
            )
        }
        // clicks in the middle of the 0-10 slider, so 5
        composeRule.onNode(customNoiseCanceling).performClick()
        verify(exactly = 1) {
            onCustomNoiseCancelingChange(any())
        }
    }

    private fun renderInitialSoundMode(
        ambientSoundMode: AmbientSoundMode,
        noiseCancelingMode: NoiseCancelingMode,
    ) {
        composeRule.setContent {
            SoundModeSettings(
                soundModes = SoundModes(
                    ambientSoundMode,
                    noiseCancelingMode,
                    TransparencyMode.VocalMode,
                    CustomNoiseCanceling(0),
                ),
                hasTransparencyModes = true,
                noiseCancelingType = NoiseCancelingType.Custom,
            )
        }
    }

    private fun assertOneSelected(
        selectedOne: SemanticsMatcher,
        group: List<SemanticsMatcher>,
    ) {
        group.forEach {
            if (it == selectedOne) {
                composeRule.onNode(it).assertIsSelected()
            } else {
                composeRule.onNode(it).assertIsNotSelected()
            }
        }
    }
}
