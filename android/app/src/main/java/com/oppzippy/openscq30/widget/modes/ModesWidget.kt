package com.oppzippy.openscq30.widget.modes

import android.content.Context
import android.content.Intent
import androidx.compose.runtime.Composable
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.TextUnit
import androidx.compose.ui.unit.TextUnitType
import androidx.glance.preview.ExperimentalGlancePreviewApi
import androidx.glance.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.booleanPreferencesKey
import androidx.datastore.preferences.core.stringPreferencesKey
import androidx.glance.Button
import androidx.glance.ButtonDefaults
import androidx.glance.ColorFilter
import androidx.glance.GlanceId
import androidx.glance.GlanceModifier
import androidx.glance.GlanceTheme
import androidx.glance.Image
import androidx.glance.ImageProvider
import androidx.glance.LocalContext
import androidx.glance.LocalSize
import androidx.glance.action.clickable
import androidx.glance.appwidget.GlanceAppWidget
import androidx.glance.appwidget.action.actionStartService
import androidx.glance.appwidget.background
import androidx.glance.appwidget.cornerRadius
import androidx.glance.appwidget.provideContent
import androidx.glance.background
import androidx.glance.currentState
import androidx.glance.layout.Alignment
import androidx.glance.layout.Box
import androidx.glance.layout.Column
import androidx.glance.layout.Row
import androidx.glance.layout.RowScope
import androidx.glance.layout.Spacer
import androidx.glance.layout.fillMaxHeight
import androidx.glance.layout.fillMaxSize
import androidx.glance.layout.fillMaxWidth
import androidx.glance.layout.height
import androidx.glance.layout.padding
import androidx.glance.layout.size
import androidx.glance.layout.width
import androidx.glance.state.PreferencesGlanceStateDefinition
import androidx.glance.text.Text
import androidx.glance.text.TextStyle
import androidx.glance.unit.ColorProvider
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceService
import com.oppzippy.openscq30.features.soundcoredevice.service.SoundcoreDeviceNotification

import androidx.glance.appwidget.SizeMode
import androidx.glance.color.ColorProvider
import androidx.glance.text.FontWeight
import androidx.glance.text.TextAlign

class ModesWidget() : GlanceAppWidget() {

    override val stateDefinition = PreferencesGlanceStateDefinition
    override val sizeMode = SizeMode.Exact

    override suspend fun provideGlance(context: Context, id: GlanceId) {
        provideContent {
            val prefs = currentState<Preferences>()
            val isSupported = prefs[IS_SUPPORTED_KEY] ?: false
            val isConnected = prefs[IS_CONNECTED_KEY] ?: false
            val currentMode = prefs[CURRENT_MODE_KEY]
            val deviceName = prefs[DEVICE_NAME_KEY]
            val lastDeviceMac = prefs[LAST_DEVICE_MAC_KEY]
            Layout(
                isConnected = isConnected,
                isSupported = isSupported,
                currentMode = currentMode,
                deviceName = deviceName,
                lastDeviceMac = lastDeviceMac,
            )
        }
    }

    @OptIn(ExperimentalGlancePreviewApi::class)
    @Preview(widthDp = 400, heightDp = 80)
    @Composable
    private fun NotSupported() {
        Layout(
            isSupported = false,
            currentMode = null,
            deviceName = null,
            lastDeviceMac = null,
        )
    }

    @OptIn(ExperimentalGlancePreviewApi::class)
    @Preview(
        widthDp = 400,
        heightDp = 150,
    )
    @Composable
    private fun NotConnected() {
        Layout(
            isConnected = false,
            currentMode = null,
            deviceName = "Soundcore Liberty 4 NC",
            lastDeviceMac = null,
        )
    }

    @OptIn(ExperimentalGlancePreviewApi::class)
    @Preview(
        widthDp = 400,
        heightDp = 150,
    )
    @Composable
    private fun LargePreview() {
        Layout(
            isSupported = true,
            currentMode = "NoiseCanceling",
            deviceName = "Soundcore Liberty 4 NC",
            lastDeviceMac = null,
        )
    }

    @OptIn(ExperimentalGlancePreviewApi::class)
    @Preview(widthDp = 400, heightDp = 100)
    @Composable
    fun Layout(
        isSupported: Boolean = true,
        isConnected: Boolean = true,
        currentMode: String? = "NoiseCanceling",
        deviceName: String? = "Soundcore Liberty 4 NC",
        lastDeviceMac: String? = null,
    ) {
        val context = LocalContext.current
        val size = LocalSize.current

        // Calculate responsive values based on widget width
        val widthDp = size.width.value
        val heightDp = size.height.value
        val spacing = (widthDp * 0.02f).dp.coerceIn(2.dp, 8.dp)
        val horizontalPadding = (widthDp * 0.05f).dp.coerceIn(4.dp, 20.dp)
        val verticalPadding = (widthDp * 0.05f).dp.coerceIn(4.dp, 20.dp)

        val disconnectedAction = if (!isConnected && lastDeviceMac != null) {
            actionStartService(
                Intent(context, DeviceService::class.java).apply {
                    putExtra(DeviceService.MAC_ADDRESS, lastDeviceMac)
                },
            )
        } else {
            actionStartService<DeviceService>(isForegroundService = true)
        }


        Column(
            modifier = GlanceModifier.then(
                if (!isConnected) GlanceModifier.clickable(disconnectedAction) else GlanceModifier,
            ).fillMaxSize().background(
                GlanceTheme.colors.background,
            ),
            verticalAlignment = Alignment.CenterVertically,
            horizontalAlignment = Alignment.CenterHorizontally,
        ) {
            if (isSupported) {
                Column(
                    modifier = GlanceModifier.padding(
                        horizontal = horizontalPadding,
                    ),
                ) {
                    if (deviceName != null && heightDp > 120f) {
                        Row(
                            modifier = GlanceModifier.fillMaxWidth(),
                            verticalAlignment = Alignment.CenterVertically,
                        ) {
                            Text(
                                text = deviceName,
                                style = TextStyle(
                                    GlanceTheme.colors.onBackground,
                                    fontSize = 16f.sp,
                                    fontWeight = FontWeight.Medium,
                                ),
                                modifier = GlanceModifier.padding(top = 4.dp),
                            )

                            Spacer(modifier = GlanceModifier.defaultWeight())

                            Text(
                                text = if (isConnected) "Connected" else context.getString(R.string.disconnected),
                                style = TextStyle(
                                    color = ColorProvider(
                                        GlanceTheme.colors.onBackground.getColor(context).copy(0.9f),
                                        GlanceTheme.colors.onBackground.getColor(context).copy(0.8f),
                                    ),
                                    fontSize = 14f.sp,
                                ),
                                modifier = GlanceModifier.padding(top = 4.dp),
                            )
                        }
                    }
                    Row(
                        modifier = GlanceModifier.fillMaxSize().padding(
                            bottom = verticalPadding,
                            top = (widthDp * if (heightDp > 120f) 0.03f else 0.05f).dp.coerceIn(4.dp, 20.dp),
                        ).height(90.dp),
                        verticalAlignment = Alignment.CenterVertically,
                        horizontalAlignment = Alignment.CenterHorizontally,
                    ) {
                        ModeButton(
                            "NoiseCanceling",
                            "Noise cancelling",
                            R.drawable.noise_cancelling,
                            currentMode,
                            isConnected,
                            widthDp,
                        )
                        Spacer(GlanceModifier.width(spacing))
                        ModeButton(
                            "Transparency",
                            "Transparency",
                            R.drawable.transparency,
                            currentMode,
                            isConnected,
                            widthDp,
                        )
                        Spacer(GlanceModifier.width(spacing))
                        ModeButton("Normal", "Normal", R.drawable.normal, currentMode, isConnected, widthDp)
                    }
                }
            } else {
                Row(
                    modifier = GlanceModifier.fillMaxSize().padding(8.dp),
                    verticalAlignment = Alignment.CenterVertically,
                    horizontalAlignment = Alignment.CenterHorizontally,
                ) {
                    Image(
                        provider = ImageProvider(R.drawable.baseline_headset_off_24),
                        contentDescription = context.getString(R.string.disconnected),
                        modifier = GlanceModifier.size(24.dp),
                        colorFilter = ColorFilter.tint(
                            GlanceTheme.colors.error,
                        ),
                    )
                    Spacer(GlanceModifier.width(8.dp))
                    Text(
                        text = "Modes not supported",
                        style = TextStyle(
                            color = GlanceTheme.colors.onBackground,
                        ),
                    )
                }
            }
        }
    }

    @Composable
    fun RowScope.ModeButton(
        modeId: String,
        label: String,
        icon: Int,
        currentMode: String?,
        enabled: Boolean = true,
        widgetWidthDp: Float,
    ) {
        val isSelected = if(!enabled) false else modeId == currentMode

        // Calculate responsive values based on widget width
        val buttonPadding = (widgetWidthDp * 0.01f).dp.coerceIn(0.dp, 6.dp)
        val iconPadding = (widgetWidthDp * 0.05f).dp.coerceIn(2.dp, 10.dp)
        val selectedCornerRadius = (widgetWidthDp * 2f).dp.coerceIn(12.dp, 30.dp)
        val unselectedCornerRadius = (widgetWidthDp * 0.5f).dp.coerceIn(6.dp, 12.dp)

        Box(
            modifier = GlanceModifier
                .fillMaxHeight()
                .padding(buttonPadding)
                .defaultWeight()
                .then(
                    if(enabled) GlanceModifier.clickable(
                        actionStartService(
                            Intent(SoundcoreDeviceNotification.ACTION_SET_ANC_MODE).apply {
                                setClass(androidx.glance.LocalContext.current, DeviceService::class.java)
                                putExtra(SoundcoreDeviceNotification.INTENT_EXTRA_ANC_MODE, modeId)
                            },
                        ),
                    )
                    else GlanceModifier,
                )
                .background(
                    if (isSelected) GlanceTheme.colors.primary
                    else GlanceTheme.colors.surfaceVariant,
                )
                .cornerRadius(
                    if (isSelected) selectedCornerRadius else unselectedCornerRadius,
                ),
            contentAlignment = Alignment.Center,
        ) {
            Image(
                provider = ImageProvider(icon),
                contentDescription = label,
                modifier = GlanceModifier.fillMaxSize().padding(iconPadding),
                colorFilter = ColorFilter.tint(
                    if (!enabled) GlanceTheme.colors.outline
                    else if (isSelected) GlanceTheme.colors.onPrimary else GlanceTheme.colors.onSurface,
                ),
            )
        }
    }

    companion object {
        val IS_SUPPORTED_KEY = booleanPreferencesKey("is_supported")
        val IS_CONNECTED_KEY = booleanPreferencesKey("is_connected")
        val CURRENT_MODE_KEY = stringPreferencesKey("current_mode")
        val DEVICE_NAME_KEY = stringPreferencesKey("device_name")
        val LAST_DEVICE_MAC_KEY = stringPreferencesKey("last_device_mac")
    }
}
