package com.oppzippy.openscq30.ui.devicesettings

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import com.oppzippy.openscq30.soundcoredevice.SoundcoreDevice
import com.oppzippy.openscq30.soundcoredevice.SoundcoreDeviceFactory
import com.oppzippy.openscq30.soundcoredevice.contentEquals
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import dagger.hilt.android.AndroidEntryPoint
import kotlinx.coroutines.FlowPreview
import kotlinx.coroutines.flow.*
import javax.inject.Inject
import kotlin.jvm.optionals.getOrNull

@AndroidEntryPoint
class DeviceSettingsActivity : ComponentActivity() {
    @Inject
    lateinit var soundcoreDeviceFactory: SoundcoreDeviceFactory

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        actionBar?.hide()
        val macAddress = intent.getStringExtra("macAddress")
        if (macAddress == null) {
            finish()
            return
        }

        setContent {
            OpenSCQ30Theme {
                Surface(
                    modifier = Modifier.fillMaxSize(), color = MaterialTheme.colorScheme.background,
                ) {
                    var soundcoreDevice by remember { mutableStateOf<SoundcoreDevice?>(null) }
                    LaunchedEffect(true) {
                        soundcoreDevice = soundcoreDeviceFactory.createSoundcoreDevice(macAddress)
                        if (soundcoreDevice == null) {
                            finish()
                        }
                    }
                    DisposableEffect(true) {
                        onDispose {
                            soundcoreDevice?.destroy()
                            soundcoreDevice = null
                        }
                    }
                    if (soundcoreDevice != null) {
                        SoundcoreDeviceSettings(soundcoreDevice!!)
                    } else {
                        Loading()
                    }
                }
            }
        }
    }
}

@OptIn(FlowPreview::class)
@Composable
private fun SoundcoreDeviceSettings(device: SoundcoreDevice) {
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

    val stateUpdateFlow = remember {
        MutableStateFlow(equalizerProfile.toEqualizerConfiguration(equalizerValues.toByteArray()))
    }
    LaunchedEffect(stateUpdateFlow) {
        stateUpdateFlow.distinctUntilChanged { old, new -> old.contentEquals(new) }.debounce(500)
            .collectLatest {
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
            stateUpdateFlow.value =
                equalizerProfile.toEqualizerConfiguration(equalizerValues.toByteArray())
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
            stateUpdateFlow.value =
                equalizerProfile.toEqualizerConfiguration(equalizerValues.toByteArray())
        },
    )
}
