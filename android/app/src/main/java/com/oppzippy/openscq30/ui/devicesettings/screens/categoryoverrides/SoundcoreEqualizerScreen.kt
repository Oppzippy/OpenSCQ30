package com.oppzippy.openscq30.ui.devicesettings.screens.categoryoverrides

import androidx.compose.foundation.ScrollState
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.LazyListState
import androidx.compose.foundation.lazy.itemsIndexed
import androidx.compose.foundation.lazy.rememberLazyListState
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.Icon
import androidx.compose.material3.LocalTextStyle
import androidx.compose.material3.PrimaryTabRow
import androidx.compose.material3.Tab
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.blur
import androidx.compose.ui.draw.drawBehind
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.Shadow
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.bindings.translateSettingId
import com.oppzippy.openscq30.lib.wrapper.Equalizer
import com.oppzippy.openscq30.lib.wrapper.ModifiableSelectCommandInner
import com.oppzippy.openscq30.lib.wrapper.Setting
import com.oppzippy.openscq30.lib.wrapper.Value
import com.oppzippy.openscq30.lib.wrapper.toValue
import com.oppzippy.openscq30.ui.devicesettings.composables.Equalizer
import com.oppzippy.openscq30.ui.devicesettings.composables.ReadOnlyEqualizer
import com.oppzippy.openscq30.ui.utils.ModifiableSelect
import kotlin.math.cos
import kotlin.math.sin

private const val SETTING_ID_PRESET_EQUALIZER_PROFILE = "presetEqualizerProfile"
private const val SETTING_ID_CUSTOM_EQUALIZER_PROFILE = "customEqualizerProfile"
private const val SETTING_ID_VOLUME_ADJUSTMENTS = "volumeAdjustments"

val deviceBlacklist = hashSetOf(
    "SoundcoreA3116", // volume adjustments are unknown, so they would be shown as all 0s
)

object SoundcoreEqualizerScreen : CategoryOverride {
    // Be overly cautious and ensure all settings are as expected. It's better to not use this override when we should
    // rather than the other way around.
    override fun shouldOverride(deviceModel: String, settings: List<Pair<String, Setting>>): Boolean {
        if (!deviceModel.startsWith("Soundcore")) return false
        if (deviceModel in deviceBlacklist) return false
        if (settings.size != 3) return false

        getSettingById<Setting.PresetEqualizerProfileSelect>(
            settings,
            SETTING_ID_PRESET_EQUALIZER_PROFILE,
        ) ?: return false
        getSettingById<Setting.ModifiableSelectSetting>(
            settings,
            SETTING_ID_CUSTOM_EQUALIZER_PROFILE,
        ) ?: return false
        getSettingById<Setting.EqualizerSetting>(settings, SETTING_ID_VOLUME_ADJUSTMENTS) ?: return false

        return true
    }

    @Composable
    override fun Screen(settings: List<Pair<String, Setting>>, setSettings: (List<Pair<String, Value>>) -> Unit) {
        val presetEqualizerProfile =
            settings.find {
                it.first == SETTING_ID_PRESET_EQUALIZER_PROFILE
            }!!.second as Setting.PresetEqualizerProfileSelect
        val selectedPresetIndex = presetEqualizerProfile.select.options.indexOf(presetEqualizerProfile.value)
            .let { if (it == -1) null else it }
        val customEqualizerProfile =
            settings.find {
                it.first == SETTING_ID_CUSTOM_EQUALIZER_PROFILE
            }!!.second as Setting.ModifiableSelectSetting
        val volumeAdjustments =
            settings.find { it.first == SETTING_ID_VOLUME_ADJUSTMENTS }!!.second as Setting.EqualizerSetting

        val isPresetSelected = presetEqualizerProfile.value != null

        var selectedTabIndex by remember { mutableIntStateOf(if (isPresetSelected) 0 else 1) }

        Column {
            PrimaryTabRow(selectedTabIndex) {
                Tab(
                    selected = selectedTabIndex == 0,
                    onClick = { selectedTabIndex = 0 },
                    text = {
                        Row(
                            verticalAlignment = Alignment.CenterVertically,
                            horizontalArrangement = Arrangement.spacedBy(8.dp),
                        ) {
                            Text(stringResource(R.string.preset))
                            if (isPresetSelected) {
                                Icon(
                                    painter = painterResource(R.drawable.check_24px),
                                    contentDescription = stringResource(R.string.in_effect),
                                )
                            }
                        }
                    },
                )
                Tab(
                    selected = selectedTabIndex == 1,
                    onClick = { selectedTabIndex = 1 },
                    text = {
                        Row(
                            verticalAlignment = Alignment.CenterVertically,
                            horizontalArrangement = Arrangement.spacedBy(8.dp),
                        ) {
                            Text(stringResource(R.string.custom))
                            if (!isPresetSelected) {
                                Icon(
                                    painter = painterResource(R.drawable.check_24px),
                                    contentDescription = stringResource(R.string.in_effect),
                                )
                            }
                        }
                    },
                )
            }

            val presetLazyListState = rememberLazyListState(
                initialFirstVisibleItemIndex = selectedPresetIndex ?: 0,
            )
            val customScrollState = rememberScrollState()

            if (selectedTabIndex == 0) {
                Preset(
                    lazyListState = presetLazyListState,
                    options = presetEqualizerProfile.select.options,
                    localizedOptions = presetEqualizerProfile.select.localizedOptions,
                    volumeAdjustments = presetEqualizerProfile.presets,
                    selectedIndex = selectedPresetIndex,
                    equalizer = presetEqualizerProfile.equalizer,
                    onSelected = {
                        val selectedOption = presetEqualizerProfile.select.options[it]
                        setSettings(listOf(SETTING_ID_PRESET_EQUALIZER_PROFILE to selectedOption.toValue()))
                    },
                )
            } else {
                Custom(
                    scrollState = customScrollState,
                    customProfileSetting = customEqualizerProfile,
                    equalizerSetting = volumeAdjustments,
                    onValueChange = { index, value ->
                        setSettings(
                            listOf(
                                SETTING_ID_VOLUME_ADJUSTMENTS to
                                    volumeAdjustments.value.mapIndexed { i, v -> if (i == index) value else v }
                                        .toValue(),
                            ),
                        )
                    },
                    onSelectCustomProfile = {
                        val selectedOption = customEqualizerProfile.setting.options[it]
                        setSettings(listOf(SETTING_ID_CUSTOM_EQUALIZER_PROFILE to selectedOption.toValue()))
                    },
                    onAddCustomProfile = {
                        setSettings(
                            listOf(
                                SETTING_ID_CUSTOM_EQUALIZER_PROFILE to ModifiableSelectCommandInner.Add(it).toValue(),
                            ),
                        )
                    },
                    onRemoveCustomProfile = {
                        val selectedOption = customEqualizerProfile.setting.options[it]
                        setSettings(
                            listOf(
                                SETTING_ID_CUSTOM_EQUALIZER_PROFILE to
                                    ModifiableSelectCommandInner.Remove(selectedOption).toValue(),
                            ),
                        )
                    },
                )
            }
        }
    }
}

private inline fun <reified T> getSettingById(settings: List<Pair<String, Setting>>, settingId: String): T? =
    settings.find { (id, setting) -> id == settingId && setting is T }?.second as T

@Composable
private fun Preset(
    options: List<String>,
    localizedOptions: List<String>,
    volumeAdjustments: List<List<Short>>,
    equalizer: Equalizer,
    selectedIndex: Int?,
    onSelected: (Int) -> Unit,
    lazyListState: LazyListState,
) {
    LazyColumn(
        state = lazyListState,
        modifier = Modifier.padding(horizontal = 16.dp),
    ) {
        itemsIndexed(
            options.zip(localizedOptions).zip(volumeAdjustments),
        ) { index, (optionAndLocalizedOption, volumeAdjustments) ->
            val option = optionAndLocalizedOption.first
            val localizedOption = optionAndLocalizedOption.second
            // we want spacing before the first item, so use a spacer rather than LazyColumn's verticalArrangement
            Spacer(Modifier.height(16.dp))
            PresetCard(
                index = index,
                option = option,
                localizedOption = localizedOption,
                isSelected = index == selectedIndex,
                volumeAdjustments = volumeAdjustments,
                equalizer = equalizer,
                onSelected = onSelected,
            )
        }
    }
}

@Composable
private fun PresetCard(
    index: Int,
    option: String,
    localizedOption: String,
    equalizer: Equalizer,
    volumeAdjustments: List<Short>,
    isSelected: Boolean,
    onSelected: (Int) -> Unit,
) {
    val gradient = presetGradients.getOrDefault(option, fallbackPresetGradient)
    Card(
        Modifier
            .height(120.dp)
            .padding(4.dp)
            .clickable { onSelected(index) },
        colors = CardDefaults.cardColors(containerColor = gradient?.left ?: Color.Unspecified),
    ) {
        Box(
            modifier = Modifier.drawBehind {
                // hacky angled gradient. only works when the angle is close to 90 degrees,
                // since otherwise rightSideY will be extraordinarily high, leading to most of
                // the gradient being off the top of the screen

                val angle = Math.toRadians(90 - gradient.angleInDegrees * -1)
                val x = cos(angle)
                val y = sin(angle)
                val slope = y / x
                val rightSideY = size.width * slope

                drawRect(
                    brush = Brush.linearGradient(
                        colors = listOf(gradient.left, gradient.right),
                        start = Offset(0f, 0f),
                        end = Offset(size.width, rightSideY.toFloat()),
                    ),
                    size = size,
                )
            },
        ) {
            if (isSelected) {
                Box(Modifier.fillMaxSize(), contentAlignment = Alignment.TopEnd) {
                    // fake shadow since using Modifier.shadow with CircleShape doesn't show the shadow inside the
                    // transparent checkmark
                    Icon(
                        modifier = Modifier.blur(2.dp),
                        tint = Color.Black.copy(alpha = 0.5f),
                        painter = painterResource(R.drawable.check_circle_24px),
                        contentDescription = null,
                    )
                    Icon(
                        tint = Color.White,
                        painter = painterResource(R.drawable.check_circle_24px),
                        contentDescription = stringResource(R.string.selected),
                    )
                }
            }

            Box(Modifier.fillMaxSize(), contentAlignment = Alignment.TopCenter) {
                ReadOnlyEqualizer(
                    modifier = Modifier
                        .fillMaxWidth()
                        .height(85.dp)
                        .padding(16.dp),
                    color = Color.White,
                    bands = equalizer.bandHz,
                    values = volumeAdjustments,
                    minValue = equalizer.min,
                    maxValue = equalizer.max,
                    fractionDigits = equalizer.fractionDigits,
                    drawHorizontalGuide = false,
                )
            }
            Box(
                Modifier
                    .fillMaxSize()
                    .padding(horizontal = 16.dp, vertical = 10.dp),
                contentAlignment = Alignment.BottomStart,
            ) {
                Text(
                    text = localizedOption,
                    style = LocalTextStyle.current.copy(
                        color = Color.White,
                        shadow = Shadow(color = Color.Black, blurRadius = 5f),
                    ),
                )
            }
        }
    }
}

val fallbackPresetGradient = Gradient(84.31, Color(0xFF666666), Color(0xFFAAAAAA))
val presetGradients = hashMapOf(
    "SoundcoreSignature" to Gradient(84.31, Color(0xFFF066CD), Color(0xFF98D3FA)),
    "Acoustic" to Gradient(84.31, Color(0xFFAE6F13), Color(0xFFFEBF63)),
    "Balanced" to Gradient(84.31, Color(0xFFAE6F13), Color(0xFFFEBF63)),
    "BassBooster" to Gradient(84.31, Color(0xFF5320F1), Color(0xFFA370FF)),
    "BassReducer" to Gradient(84.31, Color(0xFF4283F6), Color(0xFF60DCDF)),
    "VolumeBooster" to Gradient(84.31, Color(0xFF4283F6), Color(0xFF60DCDF)),
    "Classical" to Gradient(95.69, Color(0xFF111B2B), Color(0xFFE69E52)),
    "Podcast" to Gradient(84.31, Color(0xFFC33231), Color(0xFFF98D34)),
    "Dance" to Gradient(95.69, Color(0xFFE7407B), Color(0xFF733BE0)),
    "Deep" to Gradient(84.31, Color(0xFF18289A), Color(0xFF6878EA)),
    "Electronic" to Gradient(84.31, Color(0xFF4A3DB8), Color(0xFFE782CF)),
    "Flat" to Gradient(84.31, Color(0xFF236929), Color(0xFF73B979)),
    "HipHop" to Gradient(84.31, Color(0xFFF06C14), Color(0xFFEDC359)),
    "Jazz" to Gradient(84.31, Color(0xFF2C65AD), Color(0xFFB7B2AF)),
    "Latin" to Gradient(84.31, Color(0xFF7C4C3A), Color(0xFF997F5A)),
    "Lounge" to Gradient(84.31, Color(0xFF5D9FD6), Color(0xFFE1B584)),
    "Piano" to Gradient(95.69, Color(0xFFE69E52), Color(0xFF111B2B)),
    "Pop" to Gradient(95.69, Color(0xFFE91D94), Color(0xFF13ACFC)),
    "RnB" to Gradient(84.31, Color(0xFF639EFA), Color(0xFFB3EEFF)),
    "Rock" to Gradient(95.69, Color(0xFFD11836), Color(0xFF733BE0)),
    "SmallSpeakers" to Gradient(84.31, Color(0xFF733BE0), Color(0xFFE7407B)),
    "SpokenWord" to Gradient(84.31, Color(0xFF60DCDF), Color(0xFF4283F6)),
    "TrebleBooster" to Gradient(95.69, Color(0xFF733BE0), Color(0xFFD11836)),
    "TrebleReducer" to Gradient(84.31, Color(0xFF2A62C8), Color(0xFFB784DC)),
)

data class Gradient(val angleInDegrees: Double, val left: Color, val right: Color)

@Composable
fun Custom(
    scrollState: ScrollState,
    customProfileSetting: Setting.ModifiableSelectSetting,
    equalizerSetting: Setting.EqualizerSetting,
    onValueChange: (index: Int, value: Short) -> Unit,
    onSelectCustomProfile: (Int) -> Unit,
    onAddCustomProfile: (String) -> Unit,
    onRemoveCustomProfile: (Int) -> Unit,
) {
    Column(
        modifier = Modifier
            .verticalScroll(scrollState)
            .padding(horizontal = 16.dp),
    ) {
        Spacer(Modifier.height(16.dp))
        ModifiableSelect(
            name = translateSettingId(SETTING_ID_CUSTOM_EQUALIZER_PROFILE),
            showLabel = false,
            options = customProfileSetting.setting.options,
            selectedIndex = customProfileSetting.setting.options.indexOf(customProfileSetting.value)
                .let { if (it == -1) null else it },
            onSelect = onSelectCustomProfile,
            onAddOption = onAddCustomProfile,
            onRemoveOption = onRemoveCustomProfile,
        )
        Spacer(Modifier.height(16.dp))
        Equalizer(
            bands = equalizerSetting.setting.bandHz,
            values = equalizerSetting.value,
            minValue = equalizerSetting.setting.min,
            maxValue = equalizerSetting.setting.max,
            fractionDigits = equalizerSetting.setting.fractionDigits,
            onValueChange = onValueChange,
        )
    }
}
