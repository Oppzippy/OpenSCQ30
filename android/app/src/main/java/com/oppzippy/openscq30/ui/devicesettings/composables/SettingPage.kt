@file:OptIn(ExperimentalMaterial3Api::class)

package com.oppzippy.openscq30.ui.devicesettings.composables

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Slider
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Modifier
import com.oppzippy.openscq30.lib.bindings.translateSettingId
import com.oppzippy.openscq30.lib.wrapper.ModifiableSelectCommandInner
import com.oppzippy.openscq30.lib.wrapper.Range
import com.oppzippy.openscq30.lib.wrapper.Setting
import com.oppzippy.openscq30.lib.wrapper.Value
import com.oppzippy.openscq30.lib.wrapper.toValue
import com.oppzippy.openscq30.ui.utils.CheckboxWithLabel
import com.oppzippy.openscq30.ui.utils.Select
import kotlin.math.roundToInt
import kotlinx.coroutines.flow.Flow

@Composable
fun SettingPage(
    modifier: Modifier = Modifier,
    settings: List<Pair<String, Setting>>,
    setSettings: (List<Pair<String, Value>>) -> Unit,
) {
    fun setSetting(settingId: String, value: Value) {
        setSettings(listOf(Pair(settingId, value)))
    }
    Column(modifier = modifier) {
        settings.forEach { (settingId, setting) ->
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

                is Setting.I32RangeSetting -> I32Range(
                    name = name,
                    range = setting.setting,
                    value = setting.value,
                    onChange = { setSetting(settingId, it.toValue()) },
                )

                is Setting.InformationSetting -> InformationSetting(name, setting.translatedText)

                is Setting.SelectSetting -> StandardSelect(
                    name = name,
                    setting = setting,
                    onChange = { setSetting(settingId, it.toValue()) },
                )

                is Setting.OptionalSelectSetting -> OptionalSelect(
                    name = name,
                    setting = setting,
                    onChange = { setSetting(settingId, it.toValue()) },
                )

                is Setting.ModifiableSelectSetting -> ModifiableSelect(
                    name = name,
                    setting = setting,
                    onChange = { setSetting(settingId, it) },
                )
            }
        }
    }
}

@Composable
private fun Toggle(name: String, isEnabled: Boolean, onChange: (Boolean) -> Unit) {
    CheckboxWithLabel(name, isEnabled, onChange)
}

@Composable
private fun InformationSetting(name: String, text: String) {
    Row {
        Text(name)
        Text(text)
    }
}

@Composable
private fun I32Range(name: String, range: Range<Int>, value: Int, onChange: (Int) -> Unit) {
    Slider(
        value = value.toFloat(),
        steps = (range.to - range.from) / range.step + 1,
        onValueChangeFinished = { onChange(0) },
        valueRange = range.from.toFloat()..range.to.toFloat(),
        onValueChange = { onChange(it.roundToInt()) },
    )
}

@Composable
private fun Equalizer(name: String, setting: Setting.EqualizerSetting, onChange: (List<Short>) -> Unit) {
    com.oppzippy.openscq30.ui.devicesettings.composables.equalizer.Equalizer(
        bands = setting.setting.bandHz,
        values = setting.value,
        minValue = setting.setting.min,
        maxValue = setting.setting.max,
        fractionDigits = setting.setting.fractionDigits,
        onValueChange = { changedIndex, newValue ->
            onChange(
                setting.value.mapIndexed { index, oldValue ->
                    if (index == changedIndex) {
                        newValue
                    } else {
                        oldValue
                    }
                },
            )
        },
    )
}

@Composable
private fun StandardSelect(name: String, setting: Setting.SelectSetting, onChange: (String) -> Unit) {
    Select(
        name = name,
        options = setting.setting.localizedOptions,
        selectedIndex = setting.setting.options.indexOf(setting.value),
        onSelect = { onChange(setting.setting.options[it]) },
    )
}

@Composable
private fun OptionalSelect(name: String, setting: Setting.OptionalSelectSetting, onChange: (String?) -> Unit) {
    com.oppzippy.openscq30.ui.utils.OptionalSelect(
        name = name,
        options = setting.setting.localizedOptions,
        selectedIndex = setting.value?.let { setting.setting.options.indexOf(it) },
        onSelect = { onChange(it?.let { setting.setting.options[it] }) },
    )
}

@Composable
private fun ModifiableSelect(name: String, setting: Setting.ModifiableSelectSetting, onChange: (Value) -> Unit) {
    Row {
        com.oppzippy.openscq30.ui.utils.ModifiableSelect(
            name = name,
            options = setting.setting.localizedOptions,
            selectedIndex = setting.value?.let { setting.setting.options.indexOf(it) },
            onSelect = { onChange(setting.setting.options[it].toValue()) },
            onAddOption = { onChange(ModifiableSelectCommandInner.Add(it).toValue()) },
            onRemoveOption = { onChange(ModifiableSelectCommandInner.Remove(setting.setting.options[it]).toValue()) },
        )
    }
}
