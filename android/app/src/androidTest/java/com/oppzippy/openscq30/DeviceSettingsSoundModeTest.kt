package com.oppzippy.openscq30

import androidx.activity.ComponentActivity
import androidx.compose.ui.test.*
import androidx.compose.ui.test.junit4.createAndroidComposeRule
import androidx.test.ext.junit.runners.AndroidJUnit4
import com.oppzippy.openscq30.lib.*
import com.oppzippy.openscq30.soundcoredevice.SoundcoreDevice
import com.oppzippy.openscq30.soundcoredevice.SoundcoreDeviceFactory
import com.oppzippy.openscq30.ui.devicesettings.DeviceSettingsActivityView
import io.mockk.*
import io.mockk.impl.annotations.MockK
import io.mockk.junit4.MockKRule
import kotlinx.coroutines.flow.MutableStateFlow
import org.junit.Assert
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class DeviceSettingsSoundModeTest {
    @get:Rule
    val mockkRule = MockKRule(this)

    @get:Rule
    val composeRule = createAndroidComposeRule<ComponentActivity>()

    @MockK
    lateinit var deviceFactory: SoundcoreDeviceFactory

    private lateinit var ambientSoundModes: List<SemanticsMatcher>
    private lateinit var normal: SemanticsMatcher
    private lateinit var transparency: SemanticsMatcher
    private lateinit var noiseCanceling: SemanticsMatcher
    private lateinit var noiseCancelingModes: List<SemanticsMatcher>
    private lateinit var outdoor: SemanticsMatcher
    private lateinit var indoor: SemanticsMatcher
    private lateinit var transport: SemanticsMatcher

    @Before
    fun initialize() {
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
    fun testLoadInitialSoundModeNormalTransport() {
        renderInitialSoundMode(AmbientSoundMode.Normal, NoiseCancelingMode.Transport)
        assertOneSelected(normal, ambientSoundModes)
        assertOneSelected(transport, noiseCancelingModes)
    }

    @Test
    fun testLoadInitialSoundModeNoiseCancelingOutdoor() {
        renderInitialSoundMode(AmbientSoundMode.NoiseCanceling, NoiseCancelingMode.Outdoor)
        assertOneSelected(noiseCanceling, ambientSoundModes)
        assertOneSelected(outdoor, noiseCancelingModes)
    }

    @Test
    fun testSetAmbientSoundMode() {
        val pair = initializeDeviceFactoryWithOneDevice()
        val device = pair.first
        every { device.setSoundMode(any(), any()) } returns Unit

        composeRule.setContent {
            DeviceSettingsActivityView(
                macAddress = "",
                soundcoreDeviceFactory = deviceFactory,
                onDeviceNotFound = {},
            )
        }
        composeRule.onNode(transparency).performClick()
        verify(exactly = 1) {
            device.setSoundMode(
                AmbientSoundMode.Transparency, NoiseCancelingMode.Transport
            )
        }
    }

    @Test
    fun testSetNoiseCancelingMode() {
        val pair = initializeDeviceFactoryWithOneDevice()
        val device = pair.first
        every { device.setSoundMode(any(), any()) } returns Unit

        composeRule.setContent {
            DeviceSettingsActivityView(
                macAddress = "",
                soundcoreDeviceFactory = deviceFactory,
                onDeviceNotFound = {},
            )
        }
        composeRule.onNode(indoor).performClick()
        verify(exactly = 1) {
            device.setSoundMode(
                AmbientSoundMode.Normal, NoiseCancelingMode.Indoor
            )
        }
    }

    private fun renderInitialSoundMode(
        ambientSoundMode: AmbientSoundMode, noiseCancelingMode: NoiseCancelingMode
    ) {
        val pair = initializeDeviceFactoryWithOneDevice()
        val device = pair.first
        val state = pair.second
        every { state.ambientSoundMode() } returns ambientSoundMode
        every { state.noiseCancelingMode() } returns noiseCancelingMode

        composeRule.setContent {
            DeviceSettingsActivityView(
                macAddress = "",
                soundcoreDeviceFactory = deviceFactory,
                onDeviceNotFound = {},
            )
        }
        verify(exactly = 0) { device.setSoundMode(ambientSoundMode, noiseCancelingMode) }
    }

    private fun initializeDeviceFactoryWithOneDevice(): Pair<SoundcoreDevice, SoundcoreDeviceState> {
        val device = mockk<SoundcoreDevice>()
        val initialState = mockk<SoundcoreDeviceState>()
        val stateFlow = MutableStateFlow(initialState)

        val equalizerConfiguration =
            EqualizerConfiguration(PresetEqualizerProfile.SoundcoreSignature)

        coEvery { deviceFactory.createSoundcoreDevice(any()) } returns device
        every { device.state } returns initialState
        every { device.stateFlow } returns stateFlow
        every { device.setEqualizerConfiguration(any()) } returns Unit
        every { device.destroy() } returns Unit
        every { initialState.ambientSoundMode() } returns AmbientSoundMode.Normal
        every { initialState.noiseCancelingMode() } returns NoiseCancelingMode.Transport
        every { initialState.equalizerConfiguration() } returns equalizerConfiguration

        return Pair(device, initialState)
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