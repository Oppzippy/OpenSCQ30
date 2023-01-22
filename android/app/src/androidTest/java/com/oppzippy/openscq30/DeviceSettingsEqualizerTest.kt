package com.oppzippy.openscq30

import androidx.activity.ComponentActivity
import androidx.compose.ui.semantics.ProgressBarRangeInfo
import androidx.compose.ui.test.*
import androidx.compose.ui.test.junit4.createAndroidComposeRule
import androidx.test.ext.junit.runners.AndroidJUnit4
import com.oppzippy.openscq30.lib.*
import com.oppzippy.openscq30.soundcoredevice.SoundcoreDevice
import com.oppzippy.openscq30.soundcoredevice.SoundcoreDeviceFactory
import com.oppzippy.openscq30.ui.devicesettings.DeviceSettingsActivityView
import io.mockk.coEvery
import io.mockk.every
import io.mockk.impl.annotations.MockK
import io.mockk.junit4.MockKRule
import io.mockk.mockk
import kotlinx.coroutines.flow.MutableStateFlow
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class DeviceSettingsEqualizerTest {
    @get:Rule
    val mockkRule = MockKRule(this)

    @get:Rule
    val composeRule = createAndroidComposeRule<ComponentActivity>()

    @MockK
    lateinit var deviceFactory: SoundcoreDeviceFactory

    private lateinit var equalizer: SemanticsMatcher
    private lateinit var soundcoreSignature: SemanticsMatcher
    private lateinit var acoustic: SemanticsMatcher
    private lateinit var bassBooster: SemanticsMatcher
    private lateinit var classical: SemanticsMatcher
    private lateinit var custom: SemanticsMatcher

    @Before
    fun initialize() {
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
                soundcoreDeviceFactory = deviceFactory,
                onDeviceNotFound = {},
            )
        }
        composeRule.onNode(equalizer, true).performClick()
        composeRule.onNode(soundcoreSignature, true).assertDoesNotExist()
        composeRule.onNode(classical, true).assertExists()
        val sliders = composeRule.onAllNodesWithTag("equalizerSlider")
        val values = listOf(30F, 30F, -20F, -20F, 0F, 20F, 30F, 40F)
        for (i in 0..7) {
            sliders[i].assertRangeInfoEquals(ProgressBarRangeInfo(values[i], -60F..60F, 120))
        }
    }

    @Test
    fun testInitialEqualizerCustom() {
        val values = byteArrayOf(1, 10, -10, 50, 0, 10, -60, 60)

        initializeDeviceFactoryWithOneDevice(
            equalizerConfiguration = EqualizerConfiguration(EqualizerBandOffsets(values)),
        )

        composeRule.setContent {
            DeviceSettingsActivityView(
                macAddress = "",
                soundcoreDeviceFactory = deviceFactory,
                onDeviceNotFound = {},
            )
        }
        composeRule.onNode(equalizer, true).performClick()
        composeRule.onNode(soundcoreSignature, true).assertDoesNotExist()
        composeRule.onNode(custom, true).assertExists()
        val sliders = composeRule.onAllNodesWithTag("equalizerSlider")
        for (i in 0..7) {
            sliders[i].assertRangeInfoEquals(
                ProgressBarRangeInfo(
                    values[i].toFloat(), -60F..60F, 120
                )
            )
        }
    }

    @Test
    fun testSetPreset() {
        val pair = initializeDeviceFactoryWithOneDevice(
            equalizerConfiguration = EqualizerConfiguration(PresetEqualizerProfile.Acoustic),
        )
        val state = pair.second

        composeRule.setContent {
            DeviceSettingsActivityView(
                macAddress = "",
                soundcoreDeviceFactory = deviceFactory,
                onDeviceNotFound = {},
            )
        }
        composeRule.onNode(equalizer, true).performClick()
        composeRule.onNode(acoustic, true).performClick()
        composeRule.onNode(bassBooster, true).performClick()
        every { state.equalizerConfiguration() } returns EqualizerConfiguration(
            PresetEqualizerProfile.BassBooster
        )

        val values = byteArrayOf(40, 30, 10, 0, 0, 0, 0, 0)
        val sliders = composeRule.onAllNodesWithTag("equalizerSlider")
        for (i in 0..7) {
            sliders[i].assertRangeInfoEquals(
                ProgressBarRangeInfo(
                    values[i].toFloat(), -60F..60F, 120
                )
            )
        }
    }

    @Test
    fun testSetCustom() {
        val pair = initializeDeviceFactoryWithOneDevice(
            equalizerConfiguration = EqualizerConfiguration(PresetEqualizerProfile.SoundcoreSignature),
        )
        val state = pair.second

        composeRule.setContent {
            DeviceSettingsActivityView(
                macAddress = "",
                soundcoreDeviceFactory = deviceFactory,
                onDeviceNotFound = {},
            )
        }
        composeRule.onNode(equalizer).performClick()
        val values = byteArrayOf(0, 10, 15, -15, 60, -60, 10, -5)
        every { state.equalizerConfiguration() } returns EqualizerConfiguration(
            EqualizerBandOffsets(values),
        )

        val sliders = composeRule.onAllNodesWithTag("equalizerSlider")
        sliders[0].performTouchInput {
            swipe(center, centerRight, 100)
        }
        composeRule.onNode(soundcoreSignature, true).assertDoesNotExist()
        composeRule.onNode(custom, true).assertExists()
    }

    private fun initializeDeviceFactoryWithOneDevice(equalizerConfiguration: EqualizerConfiguration): Pair<SoundcoreDevice, SoundcoreDeviceState> {
        val device = mockk<SoundcoreDevice>()
        val state = mockk<SoundcoreDeviceState>()
        val stateFlow = MutableStateFlow(state)

        coEvery { deviceFactory.createSoundcoreDevice(any()) } returns device
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
