package com.oppzippy.openscq30.widget

import android.appwidget.AppWidgetManager
import android.content.Context
import android.content.Intent
import android.util.Log
import androidx.compose.runtime.Composable
import androidx.compose.ui.unit.dp
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.stringPreferencesKey
import androidx.datastore.preferences.core.stringSetPreferencesKey
import androidx.glance.Button
import androidx.glance.GlanceId
import androidx.glance.GlanceModifier
import androidx.glance.GlanceTheme
import androidx.glance.ImageProvider
import androidx.glance.LocalContext
import androidx.glance.LocalGlanceId
import androidx.glance.LocalSize
import androidx.glance.action.Action
import androidx.glance.action.clickable
import androidx.glance.appwidget.GlanceAppWidget
import androidx.glance.appwidget.GlanceAppWidgetManager
import androidx.glance.appwidget.RadioButton
import androidx.glance.appwidget.SizeMode
import androidx.glance.appwidget.Switch
import androidx.glance.appwidget.action.actionSendBroadcast
import androidx.glance.appwidget.action.actionStartActivity
import androidx.glance.appwidget.action.actionStartService
import androidx.glance.appwidget.components.Scaffold
import androidx.glance.appwidget.components.TitleBar
import androidx.glance.appwidget.cornerRadius
import androidx.glance.appwidget.lazy.LazyColumn
import androidx.glance.appwidget.lazy.items
import androidx.glance.appwidget.provideContent
import androidx.glance.appwidget.state.updateAppWidgetState
import androidx.glance.background
import androidx.glance.currentState
import androidx.glance.layout.Alignment
import androidx.glance.layout.Box
import androidx.glance.layout.Column
import androidx.glance.layout.Row
import androidx.glance.layout.Spacer
import androidx.glance.layout.fillMaxSize
import androidx.glance.layout.fillMaxWidth
import androidx.glance.layout.height
import androidx.glance.layout.padding
import androidx.glance.text.Text
import androidx.glance.text.TextStyle
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.features.soundcoredevice.service.ConnectionStatus
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceService
import com.oppzippy.openscq30.lib.bindings.OpenScq30Session
import com.oppzippy.openscq30.lib.bindings.translateDeviceModel
import com.oppzippy.openscq30.lib.bindings.translateSettingId
import com.oppzippy.openscq30.lib.wrapper.PairedDevice
import com.oppzippy.openscq30.lib.wrapper.Select
import com.oppzippy.openscq30.lib.wrapper.Setting
import com.oppzippy.openscq30.lib.wrapper.Value
import com.oppzippy.openscq30.lib.wrapper.toValue
import com.oppzippy.openscq30.ui.theme.OpenSCQ30GlanceTheme
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json

@Serializable
sealed class SettingWidgetState {
    @Serializable
    data class Disconnected(val pairedDevices: List<PairedDevice>) : SettingWidgetState()

    @Serializable
    data class Connecting(val deviceName: String) : SettingWidgetState()

    @Serializable
    data class ConnectedUnconfigured(val deviceName: String) : SettingWidgetState()

    @Serializable
    data class Connected(val deviceName: String, val settings: List<Pair<String, Setting?>>) : SettingWidgetState()
}

class SettingWidget : GlanceAppWidget() {
    companion object {
        const val TAG = "SettingWidget"
        val STATE_KEY = stringPreferencesKey("setting")

        fun settingIdsKey(deviceModel: String): Preferences.Key<Set<String>> =
            stringSetPreferencesKey("settingIds-$deviceModel")

        suspend fun updatePairedDevices(context: Context, pairedDevices: List<PairedDevice>) {
            val manager = GlanceAppWidgetManager(context)
            val widget = SettingWidget()
            manager.getGlanceIds(widget.javaClass).forEach { glanceId ->
                var isChanged = false
                updateAppWidgetState(context, glanceId) { preferences ->
                    preferences[STATE_KEY]?.let { stateJson ->
                        isChanged = try {
                            val state = Json.decodeFromString<SettingWidgetState>(stateJson)
                            state is SettingWidgetState.Disconnected
                        } catch (ex: Exception) {
                            Log.w(TAG, "error decoding widget state", ex)
                            true
                        }
                        if (isChanged) {
                            preferences[STATE_KEY] =
                                Json.encodeToString<SettingWidgetState>(
                                    SettingWidgetState.Disconnected(pairedDevices = pairedDevices),
                                )
                        }
                    }
                }
                if (isChanged) {
                    widget.update(context, glanceId)
                }
            }
        }

        suspend fun updateSettingWidgets(
            context: Context,
            session: OpenScq30Session,
            connectionStatus: ConnectionStatus,
        ) {
            val manager = GlanceAppWidgetManager(context)
            val widget = SettingWidget()

            manager.getGlanceIds(widget.javaClass).forEach { glanceId ->
                var isChanged = false
                updateAppWidgetState(context, glanceId) { preferences ->
                    val newState = when (connectionStatus) {
                        ConnectionStatus.Disconnected,
                        ConnectionStatus.AwaitingConnection,
                        -> SettingWidgetState.Disconnected(session.pairedDevices())

                        is ConnectionStatus.Connecting -> SettingWidgetState.Connecting(connectionStatus.macAddress)

                        is ConnectionStatus.Connected -> {
                            try {
                                val device = connectionStatus.deviceManager.device
                                val settingIds = preferences[settingIdsKey(device.model())]

                                if (settingIds.isNullOrEmpty()) {
                                    SettingWidgetState.ConnectedUnconfigured(
                                        deviceName = translateDeviceModel(device.model()),
                                    )
                                } else {
                                    SettingWidgetState.Connected(
                                        deviceName = translateDeviceModel(device.model()),
                                        settings = settingIds.map { settingId ->
                                            Pair(
                                                settingId,
                                                device.setting(settingId),
                                            )
                                        },
                                    )
                                }
                            } catch (ex: IllegalStateException) {
                                Log.w(
                                    TAG,
                                    "device was closed, assuming we're actually disconnected",
                                    ex,
                                )
                                SettingWidgetState.Disconnected(emptyList())
                            }
                        }
                    }

                    val jsonEncodedState = Json.encodeToString(newState)
                    if (jsonEncodedState != preferences[STATE_KEY]) {
                        preferences[STATE_KEY] = jsonEncodedState
                        isChanged = true
                    }
                }
                if (isChanged) {
                    Log.d(TAG, "glance widget $glanceId changed, updating")
                    widget.update(context, glanceId)
                }
            }
        }
    }

    override val sizeMode: SizeMode = SizeMode.Exact

    override suspend fun provideGlance(context: Context, id: GlanceId) {
        provideContent {
            val prefs = currentState<Preferences>()
            val state: SettingWidgetState? = prefs[STATE_KEY]?.let { Json.decodeFromString<SettingWidgetState?>(it) }

            Content(context, state)
        }
    }

    @Composable
    private fun Content(context: Context, state: SettingWidgetState?) {
        OpenSCQ30GlanceTheme {
            Scaffold(
                modifier = GlanceModifier.fillMaxSize(),
                titleBar = {
                    val titleText = when (state) {
                        is SettingWidgetState.Connected -> state.deviceName
                        is SettingWidgetState.ConnectedUnconfigured -> state.deviceName
                        is SettingWidgetState.Connecting -> state.deviceName
                        is SettingWidgetState.Disconnected, null -> context.getString(R.string.disconnected)
                    }
                    TitleBar(
                        startIcon = ImageProvider(R.drawable.headphones),
                        title = titleText,
                    )
                },
            ) {
                if (state == null) {
                    Disconnected(context, emptyList())
                } else {
                    when (state) {
                        is SettingWidgetState.Disconnected -> Disconnected(context, state.pairedDevices)

                        is SettingWidgetState.Connecting -> Connecting(context, state.deviceName)

                        is SettingWidgetState.ConnectedUnconfigured -> ConnectedUnconfigured(context, state.deviceName)

                        is SettingWidgetState.Connected -> Connected(
                            context,
                            state.settings,
                        )
                    }
                }
            }
        }
    }

    @Composable
    private fun Disconnected(context: Context, pairedDevices: List<PairedDevice>) {
        LazyColumn {
            items(pairedDevices) { pairedDevice ->
                Column {
                    Row(
                        modifier = GlanceModifier
                            .clickable(actionConnectToPairedDevice(context, pairedDevice.macAddress))
                            .fillMaxWidth()
                            .padding(horizontal = 12.dp, vertical = 6.dp)
                            .background(GlanceTheme.colors.surfaceVariant)
                            .cornerRadius(8.dp),
                    ) {
                        val textStyle = TextStyle(
                            color = GlanceTheme.colors.onSurfaceVariant,
                        )
                        Text(translateDeviceModel(pairedDevice.model), style = textStyle)
                        Spacer(GlanceModifier.defaultWeight())
                        Text(pairedDevice.macAddress, style = textStyle)
                    }
                    Spacer(GlanceModifier.height(4.dp))
                }
            }
        }
    }

    @Composable
    private fun Connecting(context: Context, deviceName: String) {
        Box(GlanceModifier.fillMaxSize(), contentAlignment = Alignment.Center) {
            Text(context.getString(R.string.connecting_to, deviceName))
        }
    }

    @Composable
    private fun ConnectedUnconfigured(context: Context, deviceName: String) {
        val glanceId = LocalGlanceId.current
        val appWidgetId = GlanceAppWidgetManager(LocalContext.current).getAppWidgetId(glanceId)

        Box(GlanceModifier.fillMaxSize(), contentAlignment = Alignment.Center) {
            Button(
                text = context.getString(R.string.configure_widget, deviceName),
                onClick = actionStartActivity(
                    Intent(context, SettingWidgetConfigurationActivity::class.java).apply {
                        Intent.FLAG_ACTIVITY_SINGLE_TOP
                        setPackage(context.packageName)

                        addFlags(Intent.FLAG_ACTIVITY_NO_HISTORY)

                        putExtra(AppWidgetManager.EXTRA_APPWIDGET_ID, appWidgetId)
                    },
                ),
            )
        }
    }

    @Composable
    private fun Connected(context: Context, settings: List<Pair<String, Setting?>>) {
        LazyColumn(modifier = GlanceModifier.fillMaxSize()) {
            items(settings.filter { it.second != null }) { (settingId, setting) ->
                ShowSetting(context, settingId, setting!!)
            }
        }
    }

    @Composable
    private fun ShowSetting(context: Context, settingId: String, setting: Setting) {
        when (setting) {
            is Setting.Action -> {
                Button(
                    translateSettingId(settingId),
                    onClick = actionSetSettingValue(context, settingId, true.toValue()),
                )
            }

            is Setting.EqualizerSetting -> ReadOnlySettingValue(settingId, setting.value)

            is Setting.I32RangeSetting -> {
                Row {
                    Text(translateSettingId(settingId), style = defaultTextStyle())
                    Text(setting.value.toString(), style = defaultTextStyle())
                }
            }

            is Setting.ImportStringSetting -> ReadOnlySettingValue(settingId, "")

            is Setting.InformationSetting -> ReadOnlySettingValue(settingId, setting.value)

            is Setting.SelectSetting -> Select(context, settingId, setting.setting, setting.value, isOptional = false)

            is Setting.OptionalSelectSetting -> Select(
                context,
                settingId,
                setting.setting,
                setting.value,
                isOptional = true,
            )

            is Setting.ModifiableSelectSetting -> Select(
                context,
                settingId,
                setting.setting,
                setting.value,
                isOptional = true,
            )

            is Setting.MultiSelectSetting -> TODO()

            is Setting.ToggleSetting -> {
                Row(GlanceModifier.fillMaxWidth()) {
                    Text(translateSettingId(settingId), style = defaultTextStyle())
                    Spacer(GlanceModifier.defaultWeight())
                    Switch(
                        checked = setting.value,
                        onCheckedChange = actionSetSettingValue(context, settingId, (!setting.value).toValue()),
                    )
                }
            }
        }
    }

    @Composable
    private fun Select(context: Context, settingId: String, setting: Select, value: String?, isOptional: Boolean) {
        @Composable
        fun SelectButtons() {
            if (isOptional) {
                RadioButton(
                    text = context.getString(R.string.none),
                    onClick = actionSetSettingValue(context, settingId, Value.OptionalStringValue(null)),
                    checked = value == null,
                )
            }
            setting.options.zip(setting.localizedOptions).forEach { (option, localizedOption) ->
                RadioButton(
                    text = localizedOption,
                    onClick = actionSetSettingValue(context, settingId, option.toValue()),
                    checked = value == option,
                )
            }
        }

        val size = LocalSize.current
        Column(GlanceModifier.fillMaxWidth()) {
            Text(translateSettingId(settingId), style = defaultTextStyle())
            if (size.width >= 650.dp && setting.options.size <= 3) {
                Row { SelectButtons() }
            } else {
                Column { SelectButtons() }
            }
        }
    }

    @Composable
    private fun <T> ReadOnlySettingValue(settingId: String, value: T) {
        Row(GlanceModifier.fillMaxWidth()) {
            Text(translateSettingId(settingId), style = defaultTextStyle())
            Spacer(GlanceModifier.defaultWeight())
            Text(value.toString(), style = defaultTextStyle())
        }
    }

    @Composable
    private fun defaultTextStyle(): TextStyle = TextStyle(color = GlanceTheme.colors.onSurface)

    private fun actionConnectToPairedDevice(context: Context, macAddress: String): Action = actionStartService(
        Intent(context, DeviceService::class.java).apply {
            putExtra(DeviceService.INTENT_EXTRA_MAC_ADDRESS, macAddress)
        },
        isForegroundService = true,
    )

    private fun actionSetSettingValue(context: Context, settingId: String, value: Value): Action = actionSendBroadcast(
        Intent().apply {
            `package` = context.packageName
            action = DeviceService.ACTION_SET_SETTING_VALUE
            putExtra(DeviceService.INTENT_EXTRA_SETTING_ID, settingId)
            putExtra(DeviceService.INTENT_EXTRA_SETTING_VALUE, value)
        },
    )
}
