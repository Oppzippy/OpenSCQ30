package com.oppzippy.openscq30.ui.devicesettings

import androidx.compose.runtime.*
import com.oppzippy.openscq30.soundcoredevice.SoundcoreDevice
import com.oppzippy.openscq30.soundcoredevice.contentEquals
import kotlinx.coroutines.FlowPreview
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.collectLatest
import kotlinx.coroutines.flow.debounce
import kotlinx.coroutines.flow.distinctUntilChanged
import kotlin.jvm.optionals.getOrNull

@OptIn(FlowPreview::class)
@Composable
fun SoundcoreDeviceSettings(device: SoundcoreDevice) {
    var ambientSoundMode by remember { mutableStateOf(device.state.ambientSoundMode()) }
    var noiseCancelingMode by remember { mutableStateOf(device.state.noiseCancelingMode()) }
    var equalizerProfile by remember {
        mutableStateOf(
            EqualizerProfile.fromPresetProfile(
                device.state.equalizerConfiguration().presetProfile().getOrNull()
            )
        )
    }
    var equalizerValues by remember {
        mutableStateOf(
            device.state.equalizerConfiguration().bandOffsets().volumeOffsets().asList()
        )
    }
    LaunchedEffect(device) {
        device.stateFlow.collectLatest {
            ambientSoundMode = it.ambientSoundMode()
            noiseCancelingMode = it.noiseCancelingMode()
            equalizerValues = it.equalizerConfiguration().bandOffsets().volumeOffsets().asList()
        }
    }

    val equalizerConfigurationUpdateFlow = remember {
        MutableStateFlow(equalizerProfile.toEqualizerConfiguration(equalizerValues.toByteArray()))
    }
    LaunchedEffect(equalizerConfigurationUpdateFlow) {
        equalizerConfigurationUpdateFlow.distinctUntilChanged { old, new -> old.contentEquals(new) }
            .debounce(500).collectLatest {
                device.setEqualizerConfiguration(it)
            }
    }

    DeviceSettings(
        ambientSoundMode = ambientSoundMode,
        noiseCancelingMode = noiseCancelingMode,
        equalizerProfile = equalizerProfile,
        equalizerValues = equalizerValues,
        onAmbientSoundModeChange = {
            ambientSoundMode = it
            device.setSoundMode(it, noiseCancelingMode)
        },
        onNoiseCancelingModeChange = {
            noiseCancelingMode = it
            device.setSoundMode(ambientSoundMode, it)
        },
        onEqualizerProfileChange = {
            equalizerProfile = it
            val newEqualizerConfiguration =
                equalizerProfile.toEqualizerConfiguration(equalizerValues.toByteArray())
            equalizerConfigurationUpdateFlow.value = newEqualizerConfiguration
            equalizerValues = newEqualizerConfiguration.bandOffsets().volumeOffsets().asList()
        },
        onEqualizerValueChange = { changedIndex, changedValue ->
            equalizerProfile = EqualizerProfile.Custom
            equalizerValues = equalizerValues.mapIndexed { index, value ->
                if (index == changedIndex) {
                    changedValue
                } else {
                    value
                }
            }
            equalizerConfigurationUpdateFlow.value =
                equalizerProfile.toEqualizerConfiguration(equalizerValues.toByteArray())
        },
    )
}
