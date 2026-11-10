@file:OptIn(ExperimentalMaterial3Api::class, ExperimentalLayoutApi::class)

package com.oppzippy.openscq30.ui.utils

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ExperimentalLayoutApi
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.IntrinsicSize
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.FilterChip
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.SegmentedButton
import androidx.compose.material3.SegmentedButtonDefaults
import androidx.compose.material3.SingleChoiceSegmentedButtonRow
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.alpha
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

/** How a single choice setting should be presented. */
enum class ChoiceStyle {
    /** All options side by side in one connected row. Best for 2-4 short options. */
    Segmented,

    /** Wrapping row of chips. Good for a handful of options. */
    Chips,

    /** A button that opens a dialog. Used for long lists. */
    Dialog,
}

fun choiceStyleFor(options: List<String>): ChoiceStyle = when {
    options.size in 2..4 && options.all { it.length <= 18 } && options.sumOf { it.length } <= 36 ->
        ChoiceStyle.Segmented

    options.size in 5..MAX_DISCRETE_RANGE_BUTTONS && options.all { it.length <= 3 } -> ChoiceStyle.Segmented

    options.size <= 8 -> ChoiceStyle.Chips

    else -> ChoiceStyle.Dialog
}

/** Discrete integer ranges with this many values or fewer are shown as buttons instead of a slider. */
const val MAX_DISCRETE_RANGE_BUTTONS = 6

/**
 * A label above a full width control, so that controls such as segmented buttons have enough room.
 * When [isActive] is false, the control is dimmed to show that it isn't currently in effect. It stays
 * interactive, since selecting an option will switch to it.
 */
@Composable
fun LabeledChoice(
    label: String,
    modifier: Modifier = Modifier,
    isActive: Boolean = true,
    trailing: (@Composable () -> Unit)? = null,
    content: @Composable () -> Unit,
) {
    Column(
        modifier = modifier
            .fillMaxWidth()
            .alpha(if (isActive) 1f else INACTIVE_ALPHA),
        verticalArrangement = Arrangement.spacedBy(6.dp),
    ) {
        Row(verticalAlignment = Alignment.CenterVertically) {
            Text(
                modifier = Modifier.weight(1f),
                text = label,
                style = MaterialTheme.typography.labelLarge,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
            trailing?.invoke()
        }
        content()
    }
}

const val INACTIVE_ALPHA = 0.5f

/**
 * @param emphasized use the strongest selection color, for the main control of a screen
 */
@Composable
fun SegmentedSelect(
    options: List<String>,
    selectedIndex: Int?,
    onSelect: (Int) -> Unit,
    modifier: Modifier = Modifier,
    testTagPrefix: String? = null,
    icons: List<Int?>? = null,
    emphasized: Boolean = false,
) {
    // The default selected color (secondaryContainer) is hard to tell apart from the background with some dynamic
    // color palettes, especially since we omit the check mark icon to make room for the labels.
    val colors = if (emphasized) {
        SegmentedButtonDefaults.colors(
            activeContainerColor = MaterialTheme.colorScheme.primary,
            activeContentColor = MaterialTheme.colorScheme.onPrimary,
            activeBorderColor = MaterialTheme.colorScheme.primary,
        )
    } else {
        SegmentedButtonDefaults.colors(
            activeContainerColor = MaterialTheme.colorScheme.primaryContainer,
            activeContentColor = MaterialTheme.colorScheme.onPrimaryContainer,
        )
    }
    // IntrinsicSize.Min + fillMaxHeight keeps all segments equally tall when only some labels wrap onto a second line
    // (e.g. "Noise Canceling" next to "Normal" on narrow screens).
    SingleChoiceSegmentedButtonRow(modifier = modifier.fillMaxWidth().height(IntrinsicSize.Min)) {
        options.forEachIndexed { index, option ->
            val icon = icons?.getOrNull(index)
            SegmentedButton(
                modifier = Modifier
                    .fillMaxHeight()
                    .then(if (testTagPrefix != null) Modifier.testTag("$testTagPrefix $index") else Modifier),
                shape = SegmentedButtonDefaults.itemShape(index = index, count = options.size),
                selected = index == selectedIndex,
                onClick = { onSelect(index) },
                colors = colors,
                icon = {},
                label = {
                    if (icon != null) {
                        Column(
                            modifier = Modifier.padding(vertical = 6.dp),
                            horizontalAlignment = Alignment.CenterHorizontally,
                            verticalArrangement = Arrangement.spacedBy(2.dp),
                        ) {
                            Icon(
                                modifier = Modifier.size(24.dp),
                                painter = painterResource(icon),
                                contentDescription = null,
                            )
                            SegmentLabel(option)
                        }
                    } else {
                        SegmentLabel(option)
                    }
                },
            )
        }
    }
}

@Composable
private fun SegmentLabel(text: String) {
    Text(
        text = text,
        maxLines = 2,
        overflow = TextOverflow.Ellipsis,
        textAlign = TextAlign.Center,
        style = MaterialTheme.typography.labelLarge,
    )
}

@Composable
fun ChipSelect(
    options: List<String>,
    selectedIndex: Int?,
    onSelect: (Int) -> Unit,
    modifier: Modifier = Modifier,
    testTagPrefix: String? = null,
) {
    FlowRow(
        modifier = modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.spacedBy(8.dp),
    ) {
        options.forEachIndexed { index, option ->
            FilterChip(
                modifier = if (testTagPrefix != null) Modifier.testTag("$testTagPrefix $index") else Modifier,
                selected = index == selectedIndex,
                onClick = { onSelect(index) },
                label = { Text(option) },
            )
        }
    }
}

/** Single choice selection that picks the most suitable presentation for the number of options. */
@Composable
fun AdaptiveSelect(
    name: String,
    options: List<String>,
    selectedIndex: Int?,
    onSelect: (Int) -> Unit,
    modifier: Modifier = Modifier,
    isActive: Boolean = true,
) {
    when (choiceStyleFor(options)) {
        ChoiceStyle.Segmented -> LabeledChoice(label = name, modifier = modifier, isActive = isActive) {
            SegmentedSelect(
                options = options,
                selectedIndex = selectedIndex,
                onSelect = onSelect,
                testTagPrefix = "$name option",
            )
        }

        ChoiceStyle.Chips -> LabeledChoice(label = name, modifier = modifier, isActive = isActive) {
            ChipSelect(
                options = options,
                selectedIndex = selectedIndex,
                onSelect = onSelect,
                testTagPrefix = "$name option",
            )
        }

        ChoiceStyle.Dialog -> Select(
            modifier = modifier.alpha(if (isActive) 1f else INACTIVE_ALPHA),
            name = name,
            options = options,
            selectedIndex = selectedIndex ?: -1,
            onSelect = onSelect,
        )
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewAdaptiveSelect() {
    OpenSCQ30Theme(dynamicColor = false) {
        Column(Modifier.padding(16.dp), verticalArrangement = Arrangement.spacedBy(16.dp)) {
            AdaptiveSelect(
                name = "Transportation Mode",
                options = listOf("Plane", "Car"),
                selectedIndex = 1,
                onSelect = {},
            )
            AdaptiveSelect(
                name = "Manual Noise Canceling",
                options = listOf("1", "2", "3", "4", "5"),
                selectedIndex = 4,
                onSelect = {},
                isActive = false,
            )
            AdaptiveSelect(
                name = "Auto Power Off",
                options = listOf("Disabled", "10 minutes", "20 minutes", "30 minutes", "60 minutes"),
                selectedIndex = 0,
                onSelect = {},
            )
        }
    }
}
