package com.oppzippy.openscq30.features.callautomation

import com.oppzippy.openscq30.lib.bindings.OpenScq30Device
import com.oppzippy.openscq30.lib.bindings.SettingIdValuePair
import com.oppzippy.openscq30.lib.wrapper.Setting
import com.oppzippy.openscq30.lib.wrapper.toValue

internal const val AMBIENT_SOUND_MODE_SETTING_ID = "ambientSoundMode"
internal const val TRANSPARENCY_MODE = "Transparency"

internal data class AmbientModeState(val currentMode: String, val transparencyMode: String)

internal interface AmbientModeGateway {
    fun state(): AmbientModeState?

    suspend fun setMode(mode: String)
}

internal class OpenScq30AmbientModeGateway(private val device: OpenScq30Device) : AmbientModeGateway {
    override fun state(): AmbientModeState? {
        val setting = device.setting(AMBIENT_SOUND_MODE_SETTING_ID) as? Setting.SelectSetting ?: return null
        val transparencyMode = setting.setting.options.firstOrNull {
            it.equals(TRANSPARENCY_MODE, ignoreCase = true)
        } ?: return null
        return AmbientModeState(
            currentMode = setting.value,
            transparencyMode = transparencyMode,
        )
    }

    override suspend fun setMode(mode: String) {
        device.setSettingValues(
            listOf(SettingIdValuePair(AMBIENT_SOUND_MODE_SETTING_ID, mode.toValue())),
        )
    }
}

internal class CallTransparencyController(private val ambientModeGateway: AmbientModeGateway) {
    private var callActive = false
    private var previousMode: String? = null
    private var changedByAutomation = false

    suspend fun onCallStateChanged(active: Boolean) {
        if (active == callActive) {
            return
        }

        if (active) {
            activateTransparency()
        } else {
            restorePreviousMode()
        }
    }

    private suspend fun activateTransparency() {
        val state = ambientModeGateway.state() ?: return
        if (state.currentMode != state.transparencyMode) {
            ambientModeGateway.setMode(state.transparencyMode)
            previousMode = state.currentMode
            changedByAutomation = true
        }
        callActive = true
    }

    private suspend fun restorePreviousMode() {
        val state = ambientModeGateway.state()
        val modeToRestore = previousMode?.takeIf {
            changedByAutomation && state != null && state.currentMode == state.transparencyMode
        }
        callActive = false
        previousMode = null
        changedByAutomation = false
        if (modeToRestore != null) {
            ambientModeGateway.setMode(modeToRestore)
        }
    }
}
