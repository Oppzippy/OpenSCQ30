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
import com.oppzippy.openscq30.features.equalizer.storage.CustomProfileDao
import com.oppzippy.openscq30.features.equalizer.storage.toCustomProfile
import com.oppzippy.openscq30.features.quickpresets.storage.FallbackQuickPreset
import com.oppzippy.openscq30.features.quickpresets.storage.QuickPreset
import com.oppzippy.openscq30.features.quickpresets.storage.QuickPresetRepository
import com.oppzippy.openscq30.lib.bindings.AmbientSoundMode
import com.oppzippy.openscq30.lib.bindings.DeviceFeatureFlags
import com.oppzippy.openscq30.lib.bindings.NoiseCancelingMode
import com.oppzippy.openscq30.lib.bindings.PresetEqualizerProfile
import com.oppzippy.openscq30.lib.bindings.TransparencyMode
import com.oppzippy.openscq30.lib.bindings.VolumeAdjustments
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
import java.util.UUID
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
    lateinit var quickPresetRepository: QuickPresetRepository

    @Inject
    lateinit var customProfileDao: CustomProfileDao

    private var featureFlags = DeviceFeatureFlags.all()
    private var deviceUuid = UUID(0, 0)

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
            QuickPresetScreen(featureFlags, deviceUuid)
        }
        assertEquals(null, getFirstPreset()?.name)

        composeRule.onNode(name).performTextInput("Test")
        assertEquals("Test", getFirstPreset()?.name)
    }

    @Test
    fun acceptsAmbientSoundMode() = runTest {
        composeRule.setContent {
            QuickPresetScreen(featureFlags, deviceUuid)
        }
        assertEquals(null, getFirstPreset()?.ambientSoundMode)

        composeRule.onNode(ambientSoundMode).performClick()
        assertEquals(AmbientSoundMode.Normal, getFirstPreset()?.ambientSoundMode)

        composeRule.onNodeWithText(composeRule.activity.getString(R.string.transparency))
            .performClick()
        assertEquals(AmbientSoundMode.Transparency, getFirstPreset()?.ambientSoundMode)
    }

    @Test
    fun acceptsTransparencyMode() = runTest {
        composeRule.setContent {
            QuickPresetScreen(featureFlags, deviceUuid)
        }
        assertEquals(null, getFirstPreset()?.transparencyMode)

        composeRule.onNode(transparencyMode).performClick()
        assertEquals(TransparencyMode.VocalMode, getFirstPreset()?.transparencyMode)

        composeRule.onNodeWithText(composeRule.activity.getString(R.string.fully_transparent))
            .performClick()
        assertEquals(TransparencyMode.FullyTransparent, getFirstPreset()?.transparencyMode)
    }

    @Test
    fun acceptsNoiseCancelingMode() = runTest {
        composeRule.setContent {
            QuickPresetScreen(featureFlags, deviceUuid)
        }
        assertEquals(null, getFirstPreset()?.noiseCancelingMode)

        composeRule.onNode(noiseCancelingMode).performClick()
        assertEquals(NoiseCancelingMode.Transport, getFirstPreset()?.noiseCancelingMode)

        composeRule.onNodeWithText(composeRule.activity.getString(R.string.outdoor)).performClick()
        assertEquals(NoiseCancelingMode.Outdoor, getFirstPreset()?.noiseCancelingMode)
    }

    @Test
    fun acceptsCustomNoiseCanceling() = runTest {
        composeRule.setContent {
            QuickPresetScreen(featureFlags, deviceUuid)
        }
        assertEquals(null, getFirstPreset()?.customNoiseCanceling)

        composeRule.onNode(customNoiseCanceling).performClick()
        assertEquals(
            0.toShort(),
            quickPresetRepository.getForDevice(deviceUuid)
                .getOrNull(0)?.customNoiseCanceling?.value(),
        )

        composeRule.onNodeWithTag("customNoiseCancelingSlider").performClick()
        // clicks in the middle of the 0-10 slider, which is 5
        assertEquals(
            5.toShort(),
            quickPresetRepository.getForDevice(deviceUuid)
                .getOrNull(0)?.customNoiseCanceling?.value(),
        )
    }

    @Test
    fun acceptsPresetEqualizerProfile() = runTest {
        composeRule.setContent {
            QuickPresetScreen(featureFlags, deviceUuid)
        }
        assertEquals(null, getFirstPreset()?.presetEqualizerProfile)

        composeRule.onNode(equalizer).performClick()
        assertEquals(null, getFirstPreset()?.presetEqualizerProfile)

        composeRule.onNode(presetProfile).performClick()
        composeRule.onNodeWithText(composeRule.activity.getString(R.string.classical))
            .performClick()
        assertEquals(PresetEqualizerProfile.Classical, getFirstPreset()?.presetEqualizerProfile)
    }

    @Test
    fun acceptsCustomEqualizerProfile() = runTest {
        customProfileDao.insert(
            VolumeAdjustments(
                doubleArrayOf(
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                ),
            ).toCustomProfile("Test Profile"),
        )
        composeRule.setContent {
            QuickPresetScreen(featureFlags, deviceUuid)
        }
        assertEquals(null, getFirstPreset()?.customEqualizerProfileName)

        composeRule.onNode(equalizer).performClick()
        assertEquals(null, getFirstPreset()?.customEqualizerProfileName)

        composeRule.onNode(customProfile).performClick()
        composeRule.onNodeWithText("Test Profile").performClick()
        assertEquals("Test Profile", getFirstPreset()?.customEqualizerProfileName)
    }

    @Test
    fun acceptsOnlyOneOfPresetOrCustomEqualizerProfile() = runTest {
        customProfileDao.insert(
            VolumeAdjustments(
                doubleArrayOf(
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                ),
            ).toCustomProfile("Test Profile"),
        )
        composeRule.setContent {
            QuickPresetScreen(featureFlags, deviceUuid)
        }
        composeRule.onNode(equalizer).performClick()

        // Select a preset profile
        composeRule.onNode(presetProfile).performClick()
        composeRule.onNodeWithText(composeRule.activity.getString(R.string.acoustic))
            .performClick()
        assertEquals(PresetEqualizerProfile.Acoustic, getFirstPreset()?.presetEqualizerProfile)
        assertEquals(null, getFirstPreset()?.customEqualizerProfileName)

        // Select a custom profile
        composeRule.onNode(customProfile).performClick()
        composeRule.onNodeWithText("Test Profile").performClick()
        assertEquals(null, getFirstPreset()?.presetEqualizerProfile)
        assertEquals("Test Profile", getFirstPreset()?.customEqualizerProfileName)

        // Go back to a preset to make sure the custom profile gets deselected
        composeRule.onNode(presetProfile).performClick()
        composeRule.onNodeWithText(composeRule.activity.getString(R.string.acoustic))
            .performClick()
        assertEquals(PresetEqualizerProfile.Acoustic, getFirstPreset()?.presetEqualizerProfile)
        assertEquals(null, getFirstPreset()?.customEqualizerProfileName)
    }

    @Test
    fun prioritizesDeviceProfilesOverFallback() = runTest {
        quickPresetRepository.insert(
            QuickPreset(
                deviceUuid,
                0,
                "device specific 1",
            ),
        )
        quickPresetRepository.insertFallback(
            FallbackQuickPreset(
                0,
                "fallback 1",
            ),
        )
        quickPresetRepository.insertFallback(
            FallbackQuickPreset(
                1,
                "fallback 2",
            ),
        )

        composeRule.setContent {
            QuickPresetScreen(featureFlags, deviceUuid)
        }
        assertEquals(
            "device specific 1",
            quickPresetRepository.getForDevice(deviceUuid).getOrNull(0)?.name,
        )
        assertEquals(
            "fallback 2",
            quickPresetRepository.getForDevice(deviceUuid).getOrNull(1)?.name,
        )
    }

    private suspend fun getFirstPreset(): QuickPreset? {
        return quickPresetRepository.getForDevice(deviceUuid).getOrNull(0)
    }
}
