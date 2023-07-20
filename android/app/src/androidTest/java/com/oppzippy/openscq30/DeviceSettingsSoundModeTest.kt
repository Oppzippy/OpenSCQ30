package com.oppzippy.openscq30

import androidx.compose.ui.test.SemanticsMatcher
import androidx.compose.ui.test.assertIsNotSelected
import androidx.compose.ui.test.assertIsSelected
import androidx.compose.ui.test.hasTextExactly
import androidx.compose.ui.test.junit4.createAndroidComposeRule
import androidx.compose.ui.test.performClick
import com.oppzippy.openscq30.libbindings.AmbientSoundMode
import com.oppzippy.openscq30.libbindings.NoiseCancelingMode
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
    private lateinit var transport: SemanticsMatcher

    @Before
    fun setUp() {
        hiltRule.inject()

        normal = hasTextExactly(composeRule.activity.getString(R.string.normal))
        transparency = hasTextExactly(composeRule.activity.getString(R.string.transparency))
        noiseCanceling = hasTextExactly(composeRule.activity.getString(R.string.noise_canceling))
        outdoor = hasTextExactly(composeRule.activity.getString(R.string.outdoor))
        indoor = hasTextExactly(composeRule.activity.getString(R.string.indoor))
        transport = hasTextExactly(composeRule.activity.getString(R.string.transport))
        ambientSoundModes = listOf(normal, transparency, noiseCanceling)
        noiseCancelingModes = listOf(outdoor, indoor, transport)
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
                ambientSoundMode = AmbientSoundMode.Normal,
                noiseCancelingMode = NoiseCancelingMode.Indoor,
                onAmbientSoundModeChange = onAmbientSoundModeChange,
            )
        }
        composeRule.onNode(transparency).performClick()
        verify(exactly = 1) {
            onAmbientSoundModeChange(AmbientSoundMode.Transparency)
        }
    }

    @Test
    fun setsNoiseCancelingMode() {
        val onNoiseCancelingModeChange =
            mockk<(noiseCancelingMode: NoiseCancelingMode) -> Unit>(relaxed = true)

        composeRule.setContent {
            SoundModeSettings(
                ambientSoundMode = AmbientSoundMode.Normal,
                noiseCancelingMode = NoiseCancelingMode.Indoor,
                onNoiseCancelingModeChange = onNoiseCancelingModeChange,
            )
        }
        composeRule.onNode(outdoor).performClick()
        verify(exactly = 1) {
            onNoiseCancelingModeChange(NoiseCancelingMode.Outdoor)
        }
    }

    private fun renderInitialSoundMode(
        ambientSoundMode: AmbientSoundMode,
        noiseCancelingMode: NoiseCancelingMode,
    ) {
        composeRule.setContent {
            SoundModeSettings(
                ambientSoundMode = ambientSoundMode,
                noiseCancelingMode = noiseCancelingMode,
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
