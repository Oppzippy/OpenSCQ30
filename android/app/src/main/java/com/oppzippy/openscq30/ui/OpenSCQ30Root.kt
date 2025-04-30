package com.oppzippy.openscq30.ui

import android.content.Context
import android.util.Log
import android.widget.Toast
import androidx.activity.compose.BackHandler
import androidx.compose.animation.AnimatedContent
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
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.hilt.navigation.compose.hiltViewModel
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.features.soundcoredevice.service.ConnectionStatus
import com.oppzippy.openscq30.ui.deviceselection.DeviceSelectionScreen
import com.oppzippy.openscq30.ui.devicesettings.DeviceSettingsScreen
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@Composable
fun OpenSCQ30Root(viewModel: DeviceSettingsViewModel = hiltViewModel()) {
    OpenSCQ30Theme {
        Surface(modifier = Modifier.fillMaxSize(), color = MaterialTheme.colorScheme.background) {
            val connectionStatus by viewModel.uiDeviceState.collectAsState()

            val isConnected =
                connectionStatus is ConnectionStatus.Connected || connectionStatus is ConnectionStatus.Connecting
            BackHandler(enabled = isConnected) {
                viewModel.deselectDevice()
            }
            AnimatedContent(
                targetState = isConnected,
                transitionSpec = {
                    val widthDivisor = if (targetState) 2 else -2
                    slideInHorizontally { width -> width / widthDivisor } + fadeIn() togetherWith
                        slideOutHorizontally { width -> width / -widthDivisor } + fadeOut() using
                        SizeTransform(
                            clip = false,
                        )
                },
                label = "Selection to Settings animation",
            ) { animationIsConnected ->
                if (animationIsConnected) {
                    DeviceSettingsScreen(
                        connectionStatus = connectionStatus,
                        onBack = { viewModel.deselectDevice() },
                        categoryIds = viewModel.getCategoriesFlow().collectAsState(emptyList()).value,
                        getSettingsInCategoryFlow = { viewModel.getSettingsInCategoryFlow(it) },
                        setSettings = { viewModel.setSettingValues(it) },
                    )
                } else {
                    DeviceSelectionScreen(
                        onDeviceSelected = { viewModel.selectDevice(it.macAddress) },
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
