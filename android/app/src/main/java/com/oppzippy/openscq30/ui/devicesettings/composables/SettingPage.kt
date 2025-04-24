package com.oppzippy.openscq30.ui.devicesettings.composables

import androidx.compose.foundation.layout.Column
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Modifier
import com.oppzippy.openscq30.lib.bindings.translateSettingId
import com.oppzippy.openscq30.lib.wrapper.Setting
import com.oppzippy.openscq30.lib.wrapper.Value
import com.oppzippy.openscq30.lib.wrapper.toValue
import com.oppzippy.openscq30.ui.utils.CheckboxWithLabel
import kotlin.math.pow
import kotlin.math.roundToInt
import kotlinx.coroutines.flow.Flow

@Composable
fun SettingPage(
    modifier: Modifier = Modifier,
    settingIds: List<String>,
    getSettingFlow: (String) -> Flow<Setting?>,
    setSetting: (String, Value) -> Unit,
) {
    Column(modifier = modifier) {
        settingIds.forEach { settingId ->
            getSettingFlow(settingId).collectAsState(null).value?.let { setting ->
                val name = translateSettingId(settingId)
                when (setting) {
                    is Setting.ToggleSetting -> Toggle(
                        name = name,
                        isEnabled = setting.value,
                        onChange = { setSetting(settingId, it.toValue()) },
                    )

                    is Setting.EqualizerSetting -> Equalizer(
                        name = name,
                        setting = setting,
                        onChange = { setSetting(settingId, it.toValue()) },
                    )

                    is Setting.I32RangeSetting -> TODO()
                    is Setting.InformationSetting -> TODO()
                    is Setting.ModifiableSelectSetting -> ModifiableSelect(name, setting) { TODO() }
                    is Setting.OptionalSelectSetting -> OptionalSelect(name, setting) { TODO() }
                    is Setting.SelectSetting -> StandardSelect(name, setting) { TODO() }
                }
            }
        }
    }
}

@Composable
private fun Toggle(name: String, isEnabled: Boolean, onChange: (Boolean) -> Unit) {
    CheckboxWithLabel(name, isEnabled, onChange)
}

@Composable
private fun Equalizer(name: String, setting: Setting.EqualizerSetting, onChange: (List<Short>) -> Unit) {
    val divisor = 10.0.pow(setting.setting.fractionDigits.toInt())
    com.oppzippy.openscq30.ui.devicesettings.composables.equalizer.Equalizer(
        values = setting.value.map { it.toDouble() / divisor },
        onValueChange = { changedIndex, newValue ->
            onChange(
                setting.value.mapIndexed { index, oldValue ->
                    if (index == changedIndex) {
                        (newValue * divisor).roundToInt().toShort()
                    } else {
                        oldValue
                    }
                },
            )
        },
        texts = setting.setting.bandHz.map { it.toString() },
        onTextChanged = { _, _ -> TODO() },
    )
}

@Composable
private fun StandardSelect(name: String, setting: Setting.SelectSetting, onChange: (String) -> Unit) {
}

@Composable
private fun OptionalSelect(name: String, setting: Setting.OptionalSelectSetting, onChange: (String?) -> Unit) {
}

@Composable
private fun ModifiableSelect(name: String, setting: Setting.ModifiableSelectSetting, onChange: (String?) -> Unit) {
}
