package com.oppzippy.openscq30.ui.devicesettings.screens

import androidx.annotation.DrawableRes
import androidx.compose.animation.AnimatedVisibility
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ExperimentalLayoutApi
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.bindings.translateCategoryId
import com.oppzippy.openscq30.lib.bindings.translateSettingId
import com.oppzippy.openscq30.lib.wrapper.Setting
import com.oppzippy.openscq30.lib.wrapper.Value
import com.oppzippy.openscq30.lib.wrapper.toValue
import com.oppzippy.openscq30.ui.devicesettings.Screen
import com.oppzippy.openscq30.ui.devicesettings.ScreenInfo
import com.oppzippy.openscq30.ui.utils.NavItem
import com.oppzippy.openscq30.ui.utils.SegmentedSelect

/**
 * The first screen shown after connecting. The settings that are changed most often (sound modes) are available
 * right away along with the battery status, and everything else is one tap away in the list below.
 */
@Composable
fun DeviceHomeScreen(
    screens: List<ScreenInfo>,
    onNavigation: (Screen) -> Unit,
    soundModeSettings: List<Pair<String, Setting>>,
    deviceInformationSettings: List<Pair<String, Setting>>,
    setSettings: (List<Pair<String, Value>>) -> Unit,
) {
    val batteries = batteryStatuses(deviceInformationSettings)
    // A plain scrolling column rather than a LazyColumn: the list is short, and a LazyColumn would keep its scroll
    // position anchored to the category list when the battery and sound mode settings arrive a moment later, leaving
    // the most important controls scrolled out of view.
    Column(
        modifier = Modifier
            .fillMaxSize()
            .verticalScroll(rememberScrollState())
            .padding(vertical = 8.dp)
            .testTag("deviceHome"),
        verticalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        if (batteries.isNotEmpty()) {
            BatteryStatusRow(batteries)
        }
        if (soundModeSettings.isNotEmpty()) {
            SoundModesCard(soundModeSettings, setSettings)
        }
        Text(
            modifier = Modifier.padding(start = 16.dp, top = 8.dp),
            text = stringResource(R.string.more_settings),
            style = MaterialTheme.typography.titleSmall,
            color = MaterialTheme.colorScheme.primary,
        )
        Column {
            screens.forEach { screenInfo ->
                NavItem(
                    modifier = Modifier
                        .fillMaxWidth()
                        .clickable { onNavigation(screenInfo.baseRoute) },
                    icon = { Icon(painter = painterResource(screenInfo.icon), contentDescription = null) },
                    text = screenInfo.name.translated(),
                )
            }
        }
    }
}

// region Battery

data class BatteryStatus(val label: String, val level: String, val isCharging: Boolean, @DrawableRes val icon: Int)

@Composable
private fun batteryStatuses(settings: List<Pair<String, Setting>>): List<BatteryStatus> {
    val byId = settings.toMap()
    fun level(settingId: String) = (byId[settingId] as? Setting.InformationSetting)?.translatedValue
    fun charging(settingId: String) = (byId[settingId] as? Setting.InformationSetting)?.value.let {
        it.equals("yes", ignoreCase = true) || it.equals("true", ignoreCase = true)
    }

    val left = stringResource(R.string.battery_left)
    val right = stringResource(R.string.battery_right)
    val case = stringResource(R.string.battery_case)
    val battery = stringResource(R.string.battery)
    return listOfNotNull(
        level("batteryLevel")?.let { BatteryStatus(battery, it, charging("isCharging"), R.drawable.battery_full_24px) },
        level("batteryLevelLeft")?.let {
            BatteryStatus(left, it, charging("isChargingLeft"), R.drawable.earbuds_2_24px)
        },
        level("batteryLevelRight")?.let {
            BatteryStatus(right, it, charging("isChargingRight"), R.drawable.earbuds_2_24px)
        },
        level("caseBatteryLevel")?.let { BatteryStatus(case, it, false, R.drawable.earbud_case_24px) },
    )
}

@OptIn(ExperimentalLayoutApi::class)
@Composable
private fun BatteryStatusRow(batteries: List<BatteryStatus>) {
    val chargingText = stringResource(R.string.charging)
    FlowRow(
        modifier = Modifier
            .fillMaxWidth()
            .padding(horizontal = 8.dp),
        horizontalArrangement = Arrangement.spacedBy(8.dp),
        verticalArrangement = Arrangement.spacedBy(8.dp),
    ) {
        batteries.forEach { battery ->
            Surface(
                modifier = Modifier.semantics {
                    contentDescription = buildString {
                        append(battery.label)
                        append(' ')
                        append(battery.level)
                        if (battery.isCharging) {
                            append(", ")
                            append(chargingText)
                        }
                    }
                },
                shape = RoundedCornerShape(50),
                color = MaterialTheme.colorScheme.secondaryContainer,
                contentColor = MaterialTheme.colorScheme.onSecondaryContainer,
            ) {
                Row(
                    modifier = Modifier.padding(horizontal = 12.dp, vertical = 6.dp),
                    verticalAlignment = Alignment.CenterVertically,
                    horizontalArrangement = Arrangement.spacedBy(6.dp),
                ) {
                    Icon(
                        modifier = Modifier.size(18.dp),
                        painter = painterResource(battery.icon),
                        contentDescription = null,
                    )
                    Text(battery.label, style = MaterialTheme.typography.labelLarge)
                    Text(
                        battery.level,
                        style = MaterialTheme.typography.labelLarge,
                        fontWeight = FontWeight.SemiBold,
                    )
                    if (battery.isCharging) {
                        Icon(
                            modifier = Modifier.size(16.dp),
                            painter = painterResource(R.drawable.battery_charging_full_24px),
                            contentDescription = null,
                        )
                    }
                }
            }
        }
    }
}

// endregion

// region Sound modes

internal object SoundModeSettingIds {
    const val AMBIENT_SOUND_MODE = "ambientSoundMode"
    const val NOISE_CANCELING_MODE = "noiseCancelingMode"
    const val MANUAL_NOISE_CANCELING = "manualNoiseCanceling"
    const val ADAPTIVE_NOISE_CANCELING = "adaptiveNoiseCanceling"
    const val CUSTOM_NOISE_CANCELING = "customNoiseCanceling"
    const val TRANSPORTATION_MODE = "transportationMode"
    const val MULTI_SCENE_NOISE_CANCELING = "multiSceneNoiseCanceling"
    const val ADAPTIVE_NOISE_CANCELING_SENSITIVITY_LEVEL = "adaptiveNoiseCancelingSensitivityLevel"
    const val ENVIRONMENT_DETECTION = "environmentDetection"
    const val TRANSPARENCY_MODE = "transparencyMode"
    const val MANUAL_TRANSPARENCY = "manualTransparency"
    const val WIND_NOISE_SUPPRESSION = "windNoiseSuppression"
    const val WIND_NOISE_DETECTED = "windNoiseDetected"

    val noiseCancelingGroup = listOf(
        NOISE_CANCELING_MODE,
        MANUAL_NOISE_CANCELING,
        ADAPTIVE_NOISE_CANCELING,
        CUSTOM_NOISE_CANCELING,
        TRANSPORTATION_MODE,
        MULTI_SCENE_NOISE_CANCELING,
        ADAPTIVE_NOISE_CANCELING_SENSITIVITY_LEVEL,
        ENVIRONMENT_DETECTION,
    )
    val transparencyGroup = listOf(TRANSPARENCY_MODE, MANUAL_TRANSPARENCY)
}

internal data class SoundModeRelevance(val isVisible: Boolean, val isActive: Boolean)

/**
 * The parent sound mode values a setting depends on that are not currently selected. For example, the noise canceling
 * level only takes effect in noise canceling mode with the manual noise canceling mode selected.
 *
 * Parents are only required when they exist and actually have the relevant option, so that devices with different
 * sound mode structures are not affected.
 */
internal fun missingParentSoundModes(settingId: String, settings: Map<String, Setting>): List<Pair<String, String>> {
    val ambientSoundMode = settings[SoundModeSettingIds.AMBIENT_SOUND_MODE] as? Setting.SelectSetting
    val noiseCancelingMode = settings[SoundModeSettingIds.NOISE_CANCELING_MODE] as? Setting.SelectSetting

    fun require(parent: Setting.SelectSetting?, parentId: String, option: String): Pair<String, String>? =
        if (parent != null && parent.value != option && parent.setting.options.contains(option)) {
            Pair(parentId, option)
        } else {
            null
        }

    fun ambient(option: String) = require(ambientSoundMode, SoundModeSettingIds.AMBIENT_SOUND_MODE, option)
    fun noiseCanceling(option: String) = require(noiseCancelingMode, SoundModeSettingIds.NOISE_CANCELING_MODE, option)

    return when (settingId) {
        SoundModeSettingIds.NOISE_CANCELING_MODE,
        SoundModeSettingIds.MULTI_SCENE_NOISE_CANCELING,
        SoundModeSettingIds.ADAPTIVE_NOISE_CANCELING_SENSITIVITY_LEVEL,
        SoundModeSettingIds.ENVIRONMENT_DETECTION,
        -> listOfNotNull(ambient("NoiseCanceling"))

        SoundModeSettingIds.MANUAL_NOISE_CANCELING -> listOfNotNull(ambient("NoiseCanceling"), noiseCanceling("Manual"))

        SoundModeSettingIds.ADAPTIVE_NOISE_CANCELING ->
            listOfNotNull(ambient("NoiseCanceling"), noiseCanceling("Adaptive"))

        SoundModeSettingIds.CUSTOM_NOISE_CANCELING -> listOfNotNull(ambient("NoiseCanceling"), noiseCanceling("Custom"))

        SoundModeSettingIds.TRANSPORTATION_MODE ->
            listOfNotNull(ambient("NoiseCanceling"), noiseCanceling("Transportation"))

        SoundModeSettingIds.TRANSPARENCY_MODE, SoundModeSettingIds.MANUAL_TRANSPARENCY ->
            listOfNotNull(ambient("Transparency"))

        else -> emptyList()
    }
}

/**
 * The changes needed to select [value] for [settingId] and make it take effect. openscq30-lib takes care of sending
 * the packets in an order the device accepts when several sound mode values change at once, so e.g. choosing "car"
 * while in transparency mode switches straight to noise canceling with the car transportation mode in one step.
 */
internal fun soundModeChanges(
    settingId: String,
    value: Value,
    settings: Map<String, Setting>,
): List<Pair<String, Value>> =
    missingParentSoundModes(settingId, settings).map { (id, option) -> Pair(id, option.toValue()) } +
        Pair(settingId, value)

/**
 * Determines whether a sound mode related setting is currently in effect. Settings that are not in effect (such as
 * the noise canceling level while in transparency mode) are still shown, but dimmed. Choosing one of their options
 * switches to the corresponding mode, see [soundModeChanges].
 */
internal fun soundModeRelevance(settingId: String, settings: Map<String, Setting>): SoundModeRelevance {
    val ambientSoundMode = (settings[SoundModeSettingIds.AMBIENT_SOUND_MODE] as? Setting.SelectSetting)?.value
    val isActive = missingParentSoundModes(settingId, settings).isEmpty()
    val isVisible = when (settingId) {
        // Wind noise suppression does nothing in normal mode, and toggling it there would switch to noise canceling,
        // which would be surprising for a switch.
        SoundModeSettingIds.WIND_NOISE_SUPPRESSION -> ambientSoundMode != "Normal"

        // Shown as a badge in the card header instead
        SoundModeSettingIds.WIND_NOISE_DETECTED -> false

        SoundModeSettingIds.AMBIENT_SOUND_MODE -> false

        else -> true
    }
    return SoundModeRelevance(isVisible = isVisible, isActive = isActive)
}

/** Orders sound mode settings so that each mode's options are grouped together. */
internal fun orderSoundModeSettings(settings: List<Pair<String, Setting>>): List<Pair<String, Setting>> {
    val byId = settings.toMap()
    val grouped = SoundModeSettingIds.noiseCancelingGroup + SoundModeSettingIds.transparencyGroup
    return grouped.mapNotNull { id -> byId[id]?.let { Pair(id, it) } } +
        settings.filter { (id, _) -> id !in grouped }
}

@DrawableRes
private fun ambientSoundModeIcon(option: String): Int? = when (option) {
    "NoiseCanceling" -> R.drawable.noise_control_off_24px
    "Transparency" -> R.drawable.hearing_24px
    "Normal" -> R.drawable.headphones_24px
    else -> null
}

@Composable
fun SoundModesCard(
    settings: List<Pair<String, Setting>>,
    setSettings: (List<Pair<String, Value>>) -> Unit,
    modifier: Modifier = Modifier,
) {
    val byId = settings.toMap()
    val ambientSoundMode = byId[SoundModeSettingIds.AMBIENT_SOUND_MODE] as? Setting.SelectSetting
    val isWindNoiseDetected =
        (byId[SoundModeSettingIds.WIND_NOISE_DETECTED] as? Setting.InformationSetting)?.value == "true"

    Card(
        modifier = modifier
            .fillMaxWidth()
            .testTag("soundModesCard"),
        colors = CardDefaults.cardColors(containerColor = MaterialTheme.colorScheme.surfaceContainer),
    ) {
        Column(
            modifier = Modifier.padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(16.dp),
        ) {
            Row(verticalAlignment = Alignment.CenterVertically) {
                Text(
                    modifier = Modifier.weight(1f),
                    text = translateCategoryId("soundModes"),
                    style = MaterialTheme.typography.titleMedium,
                )
                AnimatedVisibility(visible = isWindNoiseDetected) {
                    WindNoiseDetectedBadge()
                }
            }

            if (ambientSoundMode != null) {
                SegmentedSelect(
                    options = ambientSoundMode.setting.localizedOptions,
                    selectedIndex = ambientSoundMode.setting.options.indexOf(ambientSoundMode.value)
                        .takeIf { it != -1 },
                    onSelect = { index ->
                        setSettings(
                            listOf(
                                Pair(
                                    SoundModeSettingIds.AMBIENT_SOUND_MODE,
                                    ambientSoundMode.setting.options[index].toValue(),
                                ),
                            ),
                        )
                    },
                    icons = ambientSoundMode.setting.options.map { ambientSoundModeIcon(it) },
                    testTagPrefix = "${translateSettingId(SoundModeSettingIds.AMBIENT_SOUND_MODE)} option",
                    emphasized = true,
                )
            }

            orderSoundModeSettings(settings).forEach { (settingId, setting) ->
                val relevance = soundModeRelevance(settingId, byId)
                AnimatedVisibility(visible = relevance.isVisible) {
                    SettingControl(
                        settingId = settingId,
                        setting = setting,
                        setSetting = { value -> setSettings(soundModeChanges(settingId, value, byId)) },
                        isActive = relevance.isActive,
                    )
                }
            }
        }
    }
}

@Composable
private fun WindNoiseDetectedBadge() {
    Surface(
        shape = RoundedCornerShape(50),
        color = MaterialTheme.colorScheme.tertiaryContainer,
        contentColor = MaterialTheme.colorScheme.onTertiaryContainer,
    ) {
        Row(
            modifier = Modifier.padding(horizontal = 10.dp, vertical = 4.dp),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.spacedBy(4.dp),
        ) {
            Icon(
                modifier = Modifier.size(16.dp),
                painter = painterResource(R.drawable.air_24px),
                contentDescription = null,
            )
            Text(
                text = translateSettingId(SoundModeSettingIds.WIND_NOISE_DETECTED),
                style = MaterialTheme.typography.labelMedium,
            )
        }
    }
}

// endregion
