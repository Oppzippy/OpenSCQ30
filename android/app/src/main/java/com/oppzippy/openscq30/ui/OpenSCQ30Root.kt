package com.oppzippy.openscq30.ui

import android.content.Context
import android.util.Log
import android.widget.Toast
import androidx.activity.compose.BackHandler
import androidx.compose.animation.AnimatedContent
import androidx.compose.animation.ExperimentalAnimationApi
import androidx.compose.animation.SizeTransform
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.animation.slideInHorizontally
import androidx.compose.animation.slideOutHorizontally
import androidx.compose.animation.togetherWith
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.hilt.navigation.compose.hiltViewModel
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.LifecycleEventObserver
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.ui.deviceselection.DeviceSelectionScreen
import com.oppzippy.openscq30.ui.devicesettings.DeviceSettingsScreen
import com.oppzippy.openscq30.ui.devicesettings.models.UiDeviceState
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@OptIn(ExperimentalAnimationApi::class)
@Composable
fun OpenSCQ30Root(
    viewModel: DeviceSettingsViewModel = hiltViewModel(),
) {
    val lifecycleOwner = LocalLifecycleOwner.current
    val context = LocalContext.current

    OpenSCQ30Theme {
        Surface(modifier = Modifier.fillMaxSize(), color = MaterialTheme.colorScheme.background) {
            DisposableEffect(viewModel) {
                val observer = LifecycleEventObserver { _, event ->
                    when (event) {
                        Lifecycle.Event.ON_START -> viewModel.bindDeviceService()
                        Lifecycle.Event.ON_STOP -> viewModel.unbindDeviceService()
                        Lifecycle.Event.ON_DESTROY -> viewModel.stopServiceIfNotificationIsGone()

                        else -> {}
                    }
                }
                lifecycleOwner.lifecycle.addObserver(observer)
                onDispose {
                    lifecycleOwner.lifecycle.removeObserver(observer)
                }
            }
            val deviceState by viewModel.uiDeviceState.collectAsState()

            val isConnected =
                deviceState is UiDeviceState.Connected || deviceState is UiDeviceState.Loading
            BackHandler(enabled = isConnected) {
                viewModel.deselectDevice()
            }
            AnimatedContent(
                targetState = isConnected,
                transitionSpec = {
                    val widthDivisor = if (targetState) 2 else -2
                    slideInHorizontally { width -> width / widthDivisor } + fadeIn() togetherWith slideOutHorizontally { width -> width / -widthDivisor } + fadeOut() using SizeTransform(
                        clip = false,
                    )
                },
                label = "Selection to Settings animation",
            ) { animationIsConnected ->
                if (animationIsConnected) {
                    DeviceSettingsScreen(
                        deviceState = deviceState,
                        onBack = { viewModel.deselectDevice() },
                        onAmbientSoundModeChange = {
                            withErrorToast(context) {
                                viewModel.setSoundModes(ambientSoundMode = it)
                            }
                        },
                        onTransparencyModeChange = {
                            withErrorToast(context) {
                                viewModel.setSoundModes(transparencyMode = it)
                            }
                        },
                        onNoiseCancelingModeChange = {
                            withErrorToast(context) {
                                viewModel.setSoundModes(noiseCancelingMode = it)
                            }
                        },
                        onCustomNoiseCancelingChange = {
                            withErrorToast(context) {
                                viewModel.setSoundModes(customNoiseCanceling = it)
                            }
                        },
                        onEqualizerConfigurationChange = {
                            withErrorToast(context) {
                                viewModel.setEqualizerConfiguration(it)
                            }
                        },
                    )
                } else {
                    DeviceSelectionScreen(
                        onDeviceSelected = { viewModel.selectDevice(it) },
                    )
                }
            }
        }
    }
}

private fun withErrorToast(context: Context, f: () -> Unit) {
    try {
        f()
    } catch (ex: Exception) {
        Log.e("OpenSCQ30Root", "", ex)
        Toast.makeText(
            context,
            R.string.an_error_has_occurred,
            Toast.LENGTH_SHORT,
        ).show()
    }
}
