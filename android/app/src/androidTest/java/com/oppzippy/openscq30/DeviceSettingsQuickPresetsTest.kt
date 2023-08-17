package com.oppzippy.openscq30

import androidx.compose.ui.test.SemanticsMatcher
import androidx.compose.ui.test.hasTestTag
import androidx.compose.ui.test.hasTextExactly
import androidx.compose.ui.test.junit4.createAndroidComposeRule
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.performTextInput
import androidx.test.rule.GrantPermissionRule
import com.oppzippy.openscq30.features.equalizer.storage.CustomProfile
import com.oppzippy.openscq30.features.equalizer.storage.CustomProfileDao
import com.oppzippy.openscq30.features.quickpresets.storage.QuickPresetDao
import com.oppzippy.openscq30.lib.bindings.AmbientSoundMode
import com.oppzippy.openscq30.lib.bindings.NoiseCancelingMode
import com.oppzippy.openscq30.lib.bindings.PresetEqualizerProfile
import com.oppzippy.openscq30.lib.bindings.TransparencyMode
import com.oppzippy.openscq30.ui.quickpresets.QuickPresetScreen
import dagger.hilt.android.testing.HiltAndroidRule
import dagger.hilt.android.testing.HiltAndroidTest
import io.mockk.junit4.MockKRule
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.test.runTest
import org.junit.Assert.assertEquals
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import javax.inject.Inject

@OptIn(ExperimentalCoroutinesApi::class)
@HiltAndroidTest
class DeviceSettingsQuickPresetsTest {
    @get:Rule
    val permissionRule: GrantPermissionRule =
        GrantPermissionRule.grant(android.Manifest.permission.POST_NOTIFICATIONS)

    @get:Rule
    val mockkRule = MockKRule(this)

    @get:Rule(order = 1)
    val hiltRule = HiltAndroidRule(this)

    @get:Rule(order = 2)
    val composeRule = createAndroidComposeRule<TestActivity>()

    @Inject
    lateinit var quickPresetDao: QuickPresetDao

    @Inject
    lateinit var customProfileDao: CustomProfileDao

    private lateinit var name: SemanticsMatcher
    private lateinit var ambientSoundMode: SemanticsMatcher
    private lateinit var noiseCancelingMode: SemanticsMatcher
    private lateinit var transparencyMode: SemanticsMatcher
    private lateinit var customNoiseCanceling: SemanticsMatcher
    private lateinit var equalizer: SemanticsMatcher
    private lateinit var presetProfile: SemanticsMatcher
    private lateinit var customProfile: SemanticsMatcher

    @Before
    fun setUp() {
        hiltRule.inject()

        name = hasTestTag("quickPresetNameInput")
        ambientSoundMode =
            hasTextExactly(composeRule.activity.getString(R.string.ambient_sound_mode))
        noiseCancelingMode =
            hasTextExactly(composeRule.activity.getString(R.string.noise_canceling_mode))
        transparencyMode =
            hasTextExactly(composeRule.activity.getString(R.string.transparency_mode))
        customNoiseCanceling =
            hasTextExactly(composeRule.activity.getString(R.string.custom_noise_canceling))
        equalizer = hasTextExactly(composeRule.activity.getString(R.string.equalizer))
        presetProfile = hasTestTag("quickPresetPresetEqualizerProfile")
        customProfile = hasTestTag("quickPresetCustomEqualizerProfile")
    }

    @Test
    fun acceptsName() = runTest {
        composeRule.setContent {
            QuickPresetScreen()
        }
        assertEquals(null, quickPresetDao.get(0)?.name)

        composeRule.onNode(name).performTextInput("Test")
        assertEquals("Test", quickPresetDao.get(0)?.name)
    }

    @Test
    fun acceptsAmbientSoundMode() = runTest {
        composeRule.setContent {
            QuickPresetScreen()
        }
        assertEquals(null, quickPresetDao.get(0)?.ambientSoundMode)

        composeRule.onNode(ambientSoundMode).performClick()
        assertEquals(AmbientSoundMode.Normal, quickPresetDao.get(0)?.ambientSoundMode)

        composeRule.onNodeWithText(composeRule.activity.getString(R.string.transparency))
            .performClick()
        assertEquals(AmbientSoundMode.Transparency, quickPresetDao.get(0)?.ambientSoundMode)
    }

    @Test
    fun acceptsTransparencyMode() = runTest {
        composeRule.setContent {
            QuickPresetScreen()
        }
        assertEquals(null, quickPresetDao.get(0)?.transparencyMode)

        composeRule.onNode(transparencyMode).performClick()
        assertEquals(TransparencyMode.VocalMode, quickPresetDao.get(0)?.transparencyMode)

        composeRule.onNodeWithText(composeRule.activity.getString(R.string.fully_transparent))
            .performClick()
        assertEquals(TransparencyMode.FullyTransparent, quickPresetDao.get(0)?.transparencyMode)
    }

    @Test
    fun acceptsNoiseCancelingMode() = runTest {
        composeRule.setContent {
            QuickPresetScreen()
        }
        assertEquals(null, quickPresetDao.get(0)?.noiseCancelingMode)

        composeRule.onNode(noiseCancelingMode).performClick()
        assertEquals(NoiseCancelingMode.Transport, quickPresetDao.get(0)?.noiseCancelingMode)

        composeRule.onNodeWithText(composeRule.activity.getString(R.string.outdoor)).performClick()
        assertEquals(NoiseCancelingMode.Outdoor, quickPresetDao.get(0)?.noiseCancelingMode)
    }

    @Test
    fun acceptsCustomNoiseCanceling() = runTest {
        composeRule.setContent {
            QuickPresetScreen()
        }
        assertEquals(null, quickPresetDao.get(0)?.customNoiseCanceling)

        composeRule.onNode(customNoiseCanceling).performClick()
        assertEquals(0.toShort(), quickPresetDao.get(0)?.customNoiseCanceling?.value())

        composeRule.onNodeWithTag("customNoiseCancelingSlider").performClick()
        // clicks in the middle of the 0-10 slider, which is 5
        assertEquals(5.toShort(), quickPresetDao.get(0)?.customNoiseCanceling?.value())
    }

    @Test
    fun acceptsPresetEqualizerProfile() = runTest {
        composeRule.setContent {
            QuickPresetScreen()
        }
        assertEquals(null, quickPresetDao.get(0)?.presetEqualizerProfile)

        composeRule.onNode(equalizer).performClick()
        assertEquals(null, quickPresetDao.get(0)?.presetEqualizerProfile)

        composeRule.onNode(presetProfile).performClick()
        composeRule.onNodeWithText(composeRule.activity.getString(R.string.classical))
            .performClick()
        assertEquals(
            PresetEqualizerProfile.Classical,
            quickPresetDao.get(0)?.presetEqualizerProfile,
        )
    }

    @Test
    fun acceptsCustomEqualizerProfile() = runTest {
        customProfileDao.insert(
            CustomProfile(name = "Test Profile", values = listOf(0, 0, 0, 0, 0, 0, 0, 0)),
        )
        composeRule.setContent {
            QuickPresetScreen()
        }
        assertEquals(null, quickPresetDao.get(0)?.customEqualizerProfileName)

        composeRule.onNode(equalizer).performClick()
        assertEquals(null, quickPresetDao.get(0)?.customEqualizerProfileName)

        composeRule.onNode(customProfile).performClick()
        composeRule.onNodeWithText("Test Profile").performClick()
        assertEquals("Test Profile", quickPresetDao.get(0)?.customEqualizerProfileName)
    }

    @Test
    fun acceptsOnlyOneOfPresetOrCustomEqualizerProfile() = runTest {
        customProfileDao.insert(
            CustomProfile(name = "Test Profile", values = listOf(0, 0, 0, 0, 0, 0, 0, 0)),
        )
        composeRule.setContent {
            QuickPresetScreen()
        }
        composeRule.onNode(equalizer).performClick()

        // Select a preset profile
        composeRule.onNode(presetProfile).performClick()
        composeRule.onNodeWithText(composeRule.activity.getString(R.string.acoustic))
            .performClick()
        assertEquals(
            PresetEqualizerProfile.Acoustic,
            quickPresetDao.get(0)?.presetEqualizerProfile,
        )
        assertEquals(null, quickPresetDao.get(0)?.customEqualizerProfileName)

        // Select a custom profile
        composeRule.onNode(customProfile).performClick()
        composeRule.onNodeWithText("Test Profile").performClick()
        assertEquals(null, quickPresetDao.get(0)?.presetEqualizerProfile)
        assertEquals("Test Profile", quickPresetDao.get(0)?.customEqualizerProfileName)

        // Go back to a preset to make sure the custom profile gets deselected
        composeRule.onNode(presetProfile).performClick()
        composeRule.onNodeWithText(composeRule.activity.getString(R.string.acoustic))
            .performClick()
        assertEquals(
            PresetEqualizerProfile.Acoustic,
            quickPresetDao.get(0)?.presetEqualizerProfile,
        )
        assertEquals(null, quickPresetDao.get(0)?.customEqualizerProfileName)
    }
}
