package com.oppzippy.openscq30

import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.test.SemanticsMatcher
import androidx.compose.ui.test.assertIsNotSelected
import androidx.compose.ui.test.assertIsSelected
import androidx.compose.ui.test.hasAnyAncestor
import androidx.compose.ui.test.hasTestTag
import androidx.compose.ui.test.hasTextExactly
import androidx.compose.ui.test.junit4.createAndroidComposeRule
import androidx.compose.ui.test.performClick
import com.oppzippy.openscq30.lib.wrapper.AmbientSoundMode
import com.oppzippy.openscq30.lib.wrapper.AmbientSoundModeCycle
import com.oppzippy.openscq30.lib.wrapper.NoiseCancelingMode
import com.oppzippy.openscq30.lib.wrapper.SoundModes
import com.oppzippy.openscq30.lib.wrapper.TransparencyMode
import com.oppzippy.openscq30.ui.soundmode.NoiseCancelingType
import com.oppzippy.openscq30.ui.soundmode.SoundModeSettings
import dagger.hilt.android.testing.HiltAndroidRule
import dagger.hilt.android.testing.HiltAndroidTest
import io.mockk.junit4.MockKRule
import io.mockk.mockk
import io.mockk.verify
import kotlinx.coroutines.flow.MutableStateFlow
import org.junit.Assert
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
    private lateinit var ambientSoundModeNormal: SemanticsMatcher
    private lateinit var ambientSoundModeTransparency: SemanticsMatcher
    private lateinit var ambientSoundModeNoiseCanceling: SemanticsMatcher
    private lateinit var ambientSoundModeCycleNormal: SemanticsMatcher
    private lateinit var ambientSoundModeCycleTransparency: SemanticsMatcher
    private lateinit var ambientSoundModeCycleNoiseCanceling: SemanticsMatcher
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

        val normal = hasTextExactly(composeRule.activity.getString(R.string.normal))
        val transparency = hasTextExactly(composeRule.activity.getString(R.string.transparency))
        val noiseCanceling =
            hasTextExactly(composeRule.activity.getString(R.string.noise_canceling))

        val isAmbientSoundModeSelection = hasAnyAncestor(hasTestTag("ambientSoundModeSelection"))
        ambientSoundModeNormal = normal.and(isAmbientSoundModeSelection)
        ambientSoundModeTransparency = transparency.and(isAmbientSoundModeSelection)
        ambientSoundModeNoiseCanceling = noiseCanceling.and(isAmbientSoundModeSelection)

        val isAmbientSoundModeCycleSelection =
            hasAnyAncestor(hasTestTag("ambientSoundModeCycleSelection"))
        ambientSoundModeCycleNormal = normal.and(isAmbientSoundModeCycleSelection)
        ambientSoundModeCycleTransparency = transparency.and(isAmbientSoundModeCycleSelection)
        ambientSoundModeCycleNoiseCanceling = noiseCanceling.and(isAmbientSoundModeCycleSelection)

        outdoor = hasTextExactly(composeRule.activity.getString(R.string.outdoor))
        indoor = hasTextExactly(composeRule.activity.getString(R.string.indoor))
        custom = hasTextExactly(composeRule.activity.getString(R.string.custom))
        transport = hasTextExactly(composeRule.activity.getString(R.string.transport))
        fullyTransparent =
            hasTextExactly(composeRule.activity.getString(R.string.fully_transparent))
        vocalMode = hasTextExactly(composeRule.activity.getString(R.string.vocal_mode))
        customNoiseCanceling = hasTestTag("customNoiseCancelingSlider")
        ambientSoundModes = listOf(
            ambientSoundModeNormal,
            ambientSoundModeTransparency,
            ambientSoundModeNoiseCanceling
        )
        noiseCancelingModes = listOf(outdoor, indoor, transport, custom)
    }

    @Test
    fun loadsInitialSoundModeNormalTransport() {
        renderInitialSoundMode(AmbientSoundMode.Normal, NoiseCancelingMode.Transport)
        assertOneSelected(ambientSoundModeNormal, ambientSoundModes)
        assertOneSelected(transport, noiseCancelingModes)
    }

    @Test
    fun loadsInitialSoundModeNoiseCancelingOutdoor() {
        renderInitialSoundMode(AmbientSoundMode.NoiseCanceling, NoiseCancelingMode.Outdoor)
        assertOneSelected(ambientSoundModeNoiseCanceling, ambientSoundModes)
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
                    0u,
                ),
                onAmbientSoundModeChange = onAmbientSoundModeChange,
                hasTransparencyModes = false,
                noiseCancelingType = NoiseCancelingType.None,
                ambientSoundModeCycle = null,
            )
        }
        composeRule.onNode(ambientSoundModeTransparency).performClick()
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
                    0u,
                ),
                onTransparencyModeChange = onTransparencyModeChange,
                hasTransparencyModes = true,
                noiseCancelingType = NoiseCancelingType.None,
                ambientSoundModeCycle = null,
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
                    0u,
                ),
                onNoiseCancelingModeChange = onNoiseCancelingModeChange,
                hasTransparencyModes = true,
                noiseCancelingType = NoiseCancelingType.Custom,
                ambientSoundModeCycle = null
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
            mockk<(customNoiseCanceling: UByte) -> Unit>(relaxed = true)

        composeRule.setContent {
            SoundModeSettings(
                soundModes = SoundModes(
                    AmbientSoundMode.Normal,
                    NoiseCancelingMode.Custom,
                    TransparencyMode.VocalMode,
                    0u,
                ),
                onCustomNoiseCancelingChange = onCustomNoiseCancelingChange,
                hasTransparencyModes = false,
                noiseCancelingType = NoiseCancelingType.Custom,
                ambientSoundModeCycle = null
            )
        }
        // clicks in the middle of the 0-10 slider, so 5
        composeRule.onNode(customNoiseCanceling).performClick()
        verify(exactly = 1) {
            onCustomNoiseCancelingChange(5u)
        }
    }

    @Test
    fun setsAmbientSoundModeCycle() {
        val cycleFlow = MutableStateFlow(
            AmbientSoundModeCycle(
                noiseCancelingMode = true,
                normalMode = false,
                transparencyMode = true,
            )
        )
        composeRule.setContent {
            val cycle by cycleFlow.collectAsState()
            SoundModeSettings(
                soundModes = SoundModes(
                    AmbientSoundMode.Normal,
                    NoiseCancelingMode.Indoor,
                    TransparencyMode.VocalMode,
                    0u,
                ),
                hasTransparencyModes = false,
                noiseCancelingType = NoiseCancelingType.Normal,
                ambientSoundModeCycle = cycle,
                onAmbientSoundModeCycleChange = { cycleFlow.value = it }
            )
        }
        composeRule.onNode(ambientSoundModeCycleNoiseCanceling).performClick()
        composeRule.onNode(ambientSoundModeCycleNormal).performClick()
        Assert.assertEquals(
            AmbientSoundModeCycle(
                noiseCancelingMode = false,
                normalMode = true,
                transparencyMode = true,
            ),
            cycleFlow.value
        )
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
                    0u,
                ),
                hasTransparencyModes = true,
                noiseCancelingType = NoiseCancelingType.Custom,
                ambientSoundModeCycle = null
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
