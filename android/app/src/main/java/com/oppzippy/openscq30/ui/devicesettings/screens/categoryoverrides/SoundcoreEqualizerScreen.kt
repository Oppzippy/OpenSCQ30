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
import androidx.compose.ui.draw.drawWithCache
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.Path
import androidx.compose.ui.graphics.PathEffect
import androidx.compose.ui.graphics.PointMode
import androidx.compose.ui.graphics.Shadow
import androidx.compose.ui.graphics.StrokeCap
import androidx.compose.ui.graphics.drawscope.Fill
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.bindings.translateSettingId
import com.oppzippy.openscq30.lib.wrapper.ModifiableSelectCommandInner
import com.oppzippy.openscq30.lib.wrapper.Setting
import com.oppzippy.openscq30.lib.wrapper.Value
import com.oppzippy.openscq30.lib.wrapper.toValue
import com.oppzippy.openscq30.ui.devicesettings.composables.Equalizer
import com.oppzippy.openscq30.ui.utils.ModifiableSelect
import kotlin.math.cos
import kotlin.math.sin

private const val SETTING_ID_PRESET_EQUALIZER_PROFILE = "presetEqualizerProfile"
private const val SETTING_ID_CUSTOM_EQUALIZER_PROFILE = "customEqualizerProfile"
private const val SETTING_ID_VOLUME_ADJUSTMENTS = "volumeAdjustments"

// Since this screen is very specific to a particular set of settings in the Equalizer category, it's safest to enable
// it per-device and otherwise fall back to the generic display.
private val enabledDevices = hashSetOf(
    "SoundcoreA3004",
    "SoundcoreA3027",
    "SoundcoreA3028",
    "SoundcoreA3029",
    "SoundcoreA3030",
    "SoundcoreA3031",
    "SoundcoreA3033",
    "SoundcoreA3035",
    "SoundcoreA3040",
    "SoundcoreA3926",
    "SoundcoreA3930",
    "SoundcoreA3931",
    "SoundcoreA3933",
    "SoundcoreA3936",
    "SoundcoreA3945",
    "SoundcoreA3951",
    "SoundcoreA3939",
    "SoundcoreA3935",
    "SoundcoreA3955",
    "SoundcoreA3959",
    "SoundcoreA3947",
    "SoundcoreA3948",
    "SoundcoreA3949",
)

object SoundcoreEqualizerScreen : CategoryOverride {
    // Be overly cautious and ensure all settings are as expected. It's better to not use this override when we should
    // rather than the other way around.
    override fun shouldOverride(deviceModel: String, settings: List<Pair<String, Setting>>): Boolean {
        if (deviceModel !in enabledDevices) return false
        if (settings.size != 3) return false

        val preset = getSettingById<Setting.OptionalSelectSetting>(
            settings,
            "presetEqualizerProfile",
        ) ?: return false
        getSettingById<Setting.ModifiableSelectSetting>(
            settings,
            "customEqualizerProfile",
        ) ?: return false
        val volumeAdjustments = getSettingById<Setting.EqualizerSetting>(settings, "volumeAdjustments") ?: return false

        if (
            preset.setting.options.size != presetGradients.size ||
            !preset.setting.options.all { presetGradients.contains(it) }
        ) {
            return false
        }
        return volumeAdjustments.setting.bandHz == listOf(
            100.toUShort(),
            200.toUShort(),
            400.toUShort(),
            800.toUShort(),
            1600.toUShort(),
            3200.toUShort(),
            6400.toUShort(),
            12800.toUShort(),
        ) && volumeAdjustments.setting.fractionDigits == 1.toShort()
    }

    @Composable
    override fun Screen(settings: List<Pair<String, Setting>>, setSettings: (List<Pair<String, Value>>) -> Unit) {
        val presetEqualizerProfile =
            settings.find { it.first == SETTING_ID_PRESET_EQUALIZER_PROFILE }!!.second as Setting.OptionalSelectSetting
        val selectedPresetIndex = presetEqualizerProfile.setting.options.indexOf(presetEqualizerProfile.value)
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
                    options = presetEqualizerProfile.setting.options,
                    localizedOptions = presetEqualizerProfile.setting.localizedOptions,
                    selectedIndex = selectedPresetIndex,
                    onSelected = {
                        val selectedOption = presetEqualizerProfile.setting.options[it]
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
    selectedIndex: Int?,
    onSelected: (Int) -> Unit,
    lazyListState: LazyListState,
) {
    LazyColumn(
        state = lazyListState,
        modifier = Modifier.padding(horizontal = 16.dp),
    ) {
        itemsIndexed(options.zip(localizedOptions)) { index, (option, localizedOption) ->
            // we want spacing before the first item, so use a spacer rather than LazyColumn's verticalArrangement
            Spacer(Modifier.height(16.dp))
            PresetCard(
                index = index,
                option = option,
                localizedOption = localizedOption,
                isSelected = index == selectedIndex,
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
    isSelected: Boolean,
    onSelected: (Int) -> Unit,
) {
    val gradient = presetGradients[option]
    Card(
        Modifier
            .height(120.dp)
            .padding(4.dp)
            .clickable { onSelected(index) },
        colors = CardDefaults.cardColors(containerColor = gradient?.left ?: Color.Unspecified),
    ) {
        Box(
            modifier = Modifier.let { modifier ->
                if (gradient != null) {
                    modifier.drawBehind {
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
                    }
                } else {
                    modifier
                }
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
            presetVolumeAdjustments[option]?.let {
                Box(Modifier.fillMaxSize(), contentAlignment = Alignment.TopCenter) {
                    EqualizerLine(
                        Modifier
                            .fillMaxWidth()
                            .height(85.dp)
                            .padding(16.dp),
                        it,
                    )
                }
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

val presetGradients = hashMapOf(
    "SoundcoreSignature" to Gradient(84.31, Color(0xFFF066CD), Color(0xFF98D3FA)),
    "Acoustic" to Gradient(84.31, Color(0xFFAE6F13), Color(0xFFFEBF63)),
    "BassBooster" to Gradient(84.31, Color(0xFF5320F1), Color(0xFFA370FF)),
    "BassReducer" to Gradient(84.31, Color(0xFF4283F6), Color(0xFF60DCDF)),
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

val presetVolumeAdjustments = hashMapOf(
    "SoundcoreSignature" to listOf(0, 0, 0, 0, 0, 0, 0, 0),
    "Acoustic" to listOf(40, 10, 20, 20, 40, 40, 40, 20),
    "BassBooster" to listOf(40, 30, 10, 0, 0, 0, 0, 0),
    "BassReducer" to listOf(-40, -30, -10, 0, 0, 0, 0, 0),
    "Classical" to listOf(30, 30, -20, -20, 0, 20, 30, 40),
    "Podcast" to listOf(-30, 20, 40, 40, 30, 20, 0, -20),
    "Dance" to listOf(20, -30, -10, 10, 20, 20, 10, -30),
    "Deep" to listOf(20, 10, 30, 30, 20, -20, -40, -50),
    "Electronic" to listOf(30, 20, -20, 20, 10, 20, 30, 30),
    "Flat" to listOf(-20, -20, -10, 0, 0, 0, -20, -20),
    "HipHop" to listOf(20, 30, -10, -10, 20, -10, 20, 30),
    "Jazz" to listOf(20, 20, -20, -20, 0, 20, 30, 40),
    "Latin" to listOf(0, 0, -20, -20, -20, 0, 30, 50),
    "Lounge" to listOf(-10, 20, 40, 30, 0, -20, 20, 10),
    "Piano" to listOf(0, 30, 30, 20, 40, 50, 30, 40),
    "Pop" to listOf(-10, 10, 30, 30, 10, -10, -20, -30),
    "RnB" to listOf(60, 20, -20, -20, 20, 30, 30, 40),
    "Rock" to listOf(30, 20, -10, -10, 10, 30, 30, 30),
    "SmallSpeakers" to listOf(40, 30, 10, 0, -20, -30, -40, -40),
    "SpokenWord" to listOf(-30, -20, 10, 20, 20, 10, 0, -30),
    "TrebleBooster" to listOf(-20, -20, -20, -10, 10, 20, 20, 40),
    "TrebleReducer" to listOf(0, 0, 0, -20, -30, -40, -40, -60),
)

@Composable
private fun EqualizerLine(modifier: Modifier = Modifier, volumeAdjustments: List<Int>) {
    Spacer(
        modifier.drawWithCache {
            val strokeSize = 2.dp.toPx()
            val points =
                equalizerLinePoints(size.width, size.height, strokeSize, volumeAdjustments)
            val filledPath = Path().apply {
                moveTo(0f, size.height)
                lineTo(0f, points.first().y)
                points.forEach {
                    lineTo(it.x, it.y)
                }
                lineTo(size.width, points.last().y)
                lineTo(size.width, size.height)
                close()
            }
            onDrawBehind {
                drawPath(
                    path = filledPath,
                    color = Color.White,
                    style = Fill,
                    alpha = 0.5f,
                )
                drawPoints(
                    points = points,
                    color = Color.White,
                    pointMode = PointMode.Polygon,
                    strokeWidth = strokeSize,
                    cap = StrokeCap.Round,
                    pathEffect = PathEffect.cornerPathEffect(strokeSize),
                )
            }
        },
    )
}

private fun equalizerLinePoints(width: Float, height: Float, padding: Float, values: List<Int>): List<Offset> {
    val widthWithoutPadding = width - padding * 2
    val heightWithoutPadding = height - padding * 2
    val minVolume = -60
    val maxVolume = 60
    val range = maxVolume - minVolume

    val points = values.mapIndexed { index, value ->
        val normalizedX = index.toFloat() / (values.size - 1).toFloat()
        val x = normalizedX * widthWithoutPadding + padding
        val normalizedY = 1F - ((value - minVolume) / range.toFloat())
        val y = normalizedY * heightWithoutPadding + padding
        Offset(x, y)
    }
    return points
}

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
