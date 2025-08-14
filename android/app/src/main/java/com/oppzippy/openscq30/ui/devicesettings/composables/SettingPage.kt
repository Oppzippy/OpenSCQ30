@file:OptIn(ExperimentalMaterial3Api::class)

package com.oppzippy.openscq30.ui.devicesettings.composables

import android.content.ClipData
import android.content.ClipboardManager
import android.util.Log
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Check
import androidx.compose.material.icons.filled.ContentCopy
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Button
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.Slider
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import androidx.core.content.getSystemService
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.bindings.translateSettingId
import com.oppzippy.openscq30.lib.wrapper.ModifiableSelectCommandInner
import com.oppzippy.openscq30.lib.wrapper.Range
import com.oppzippy.openscq30.lib.wrapper.Setting
import com.oppzippy.openscq30.lib.wrapper.Value
import com.oppzippy.openscq30.lib.wrapper.toValue
import com.oppzippy.openscq30.ui.utils.Labeled
import com.oppzippy.openscq30.ui.utils.LabeledSwitch
import com.oppzippy.openscq30.ui.utils.Select
import com.oppzippy.openscq30.ui.utils.throttledState
import kotlin.math.roundToInt

@Composable
fun SettingPage(
    modifier: Modifier = Modifier,
    settings: List<Pair<String, Setting>>,
    setSettings: (List<Pair<String, Value>>) -> Unit,
) {
    fun setSetting(settingId: String, value: Value) {
        setSettings(listOf(Pair(settingId, value)))
    }
    Column(
        modifier = modifier.verticalScroll(rememberScrollState()),
        verticalArrangement = Arrangement.spacedBy(4.dp),
    ) {
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

                is Setting.InformationSetting -> InformationSetting(name, setting.translatedValue)

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

                is Setting.MultiSelectSetting -> MultiSelect(
                    name = name,
                    setting = setting,
                    onChange = { setSetting(settingId, it) },
                )

                is Setting.ImportStringSetting -> ImportString(
                    name = name,
                    confirmationMessage = setting.confirmationMessage,
                    onImport = { setSetting(settingId, it.toValue()) },
                )
            }
        }
    }
}

@Composable
private fun Toggle(name: String, isEnabled: Boolean, onChange: (Boolean) -> Unit) {
    LabeledSwitch(name, isEnabled, onChange)
}

@Composable
private fun InformationSetting(name: String, text: String) {
    val context = LocalContext.current
    Labeled(label = name) {
        Row(verticalAlignment = Alignment.CenterVertically) {
            Text(
                modifier = Modifier.weight(1f, fill = false),

                text = text,
            )
            IconButton(
                onClick = {
                    val clipboardManager = context.getSystemService<ClipboardManager>()
                    if (clipboardManager != null) {
                        clipboardManager.setPrimaryClip(ClipData.newPlainText(name, text))
                    } else {
                        Log.w("InformationSetting", "failed to acquire ClipboardManager")
                    }
                },
            ) {
                Icon(
                    imageVector = Icons.Filled.ContentCopy,
                    contentDescription = stringResource(
                        R.string.copy_x_to_clipboard,
                        name,
                    ),
                )
            }
        }
    }
}

@Composable
private fun I32Range(name: String, range: Range<Int>, value: Int, onChange: (Int) -> Unit) {
    Labeled(label = name) {
        val (displayedValue, setDisplayedValue) = throttledState(
            value = value,
            duration = 250,
            onValueChange = { onChange(it) },
        )
        Slider(
            modifier = Modifier.testTag("$name slider"),
            value = displayedValue.toFloat(),
            steps = (range.end - range.start) / range.step + 1,
            valueRange = range.start.toFloat()..range.end.toFloat(),
            onValueChange = { setDisplayedValue(it.roundToInt()) },
        )
    }
}

@Composable
private fun Equalizer(name: String, setting: Setting.EqualizerSetting, onChange: (List<Short>) -> Unit) {
    Equalizer(
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
    com.oppzippy.openscq30.ui.utils.ModifiableSelect(
        name = name,
        options = setting.setting.localizedOptions,
        selectedIndex = setting.value?.let { setting.setting.options.indexOf(it) },
        onSelect = { onChange(setting.setting.options[it].toValue()) },
        onAddOption = { onChange(ModifiableSelectCommandInner.Add(it).toValue()) },
        onRemoveOption = { onChange(ModifiableSelectCommandInner.Remove(setting.setting.options[it]).toValue()) },
    )
}

@Composable
private fun MultiSelect(name: String, setting: Setting.MultiSelectSetting, onChange: (Value) -> Unit) {
    val values = setting.values.toSet()
    com.oppzippy.openscq30.ui.utils.MultiSelect(
        name = name,
        options = setting.setting.localizedOptions,
        selectedOptions = setting.setting.options.mapIndexed { index, option ->
            if (values.contains(option)) index else null
        }.filterNotNull().toSet(),
        onChange = { onChange(it.map { index -> setting.setting.options[index] }.toValue()) },
    )
}

@Composable
private fun ImportString(name: String, confirmationMessage: String, onImport: (String) -> Unit) {
    var isDialogOpen by remember { mutableStateOf(false) }
    var text by remember { mutableStateOf("") }
    if (isDialogOpen) {
        AlertDialog(
            onDismissRequest = { isDialogOpen = false },
            title = { Text(name) },
            text = { Text(confirmationMessage) },
            confirmButton = {
                Button(
                    onClick = {
                        onImport(text)
                        isDialogOpen = false
                        text = ""
                    },
                ) {
                    Text(stringResource(R.string.confirm))
                }
            },
            dismissButton = {
                Button(onClick = { isDialogOpen = false }) {
                    Text(stringResource(R.string.cancel))
                }
            },
        )
    }
    Row {
        TextField(
            modifier = Modifier.fillMaxWidth(),
            label = { Text(name) },
            value = text,
            onValueChange = { text = it },
            trailingIcon = {
                IconButton(onClick = { isDialogOpen = true }) {
                    Icon(
                        imageVector = Icons.Filled.Check,
                        contentDescription = stringResource(R.string.import_),
                    )
                }
            },
        )
    }
}
