package com.oppzippy.openscq30.widget

import android.appwidget.AppWidgetManager
import android.content.Intent
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.safeDrawingPadding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.selection.toggleable
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Card
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Switch
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.datastore.preferences.core.Preferences
import androidx.glance.appwidget.GlanceAppWidgetManager
import androidx.glance.appwidget.state.getAppWidgetState
import androidx.glance.appwidget.state.updateAppWidgetState
import androidx.lifecycle.lifecycleScope
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.features.soundcoredevice.service.ConnectionStatus
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceService
import com.oppzippy.openscq30.lib.bindings.OpenScq30Session
import com.oppzippy.openscq30.lib.bindings.translateCategoryId
import com.oppzippy.openscq30.lib.bindings.translateSettingId
import com.oppzippy.openscq30.ui.DeviceServiceConnection
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import dagger.hilt.android.AndroidEntryPoint
import javax.inject.Inject
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.collectLatest
import kotlinx.coroutines.launch

@AndroidEntryPoint
class SettingWidgetConfigurationActivity : ComponentActivity() {
    @Inject
    lateinit var session: OpenScq30Session

    private val deviceServiceConnection = DeviceServiceConnection()
    private val enabledSettingIds: MutableStateFlow<Set<String>?> = MutableStateFlow(null)

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        lifecycleScope.launch {
            DeviceService.isRunning.collectLatest { isRunning ->
                if (isRunning) {
                    bindService(
                        Intent(this@SettingWidgetConfigurationActivity, DeviceService::class.java),
                        deviceServiceConnection,
                        0,
                    )
                }
            }
        }

        val appWidgetId =
            intent?.extras?.getInt(AppWidgetManager.EXTRA_APPWIDGET_ID, AppWidgetManager.INVALID_APPWIDGET_ID)
                ?: AppWidgetManager.INVALID_APPWIDGET_ID
        setResult(RESULT_OK, Intent().putExtra(AppWidgetManager.EXTRA_APPWIDGET_ID, appWidgetId))

        val manager = GlanceAppWidgetManager(this)
        val glanceId = manager.getGlanceIdBy(appWidgetId)
        val widget = SettingWidget()

        lifecycleScope.launch {
            deviceServiceConnection.connectionStatusFlow.collectLatest { connectionStatus ->
                if (connectionStatus is ConnectionStatus.Connected) {
                    val model = connectionStatus.deviceManager.device.model()
                    val preferences =
                        widget.getAppWidgetState<Preferences>(this@SettingWidgetConfigurationActivity, glanceId)
                    enabledSettingIds.value = preferences[SettingWidget.settingIdsKey(model)] ?: emptySet()
                } else {
                    enabledSettingIds.value = null
                }
            }
        }

        enableEdgeToEdge()
        actionBar?.hide()
        setContent {
            OpenSCQ30Theme {
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background,
                ) {
                    Column(
                        Modifier
                            .safeDrawingPadding()
                            .fillMaxSize()
                            .verticalScroll(rememberScrollState()),
                    ) {
                        when (
                            val connectionStatus =
                                deviceServiceConnection.connectionStatusFlow.collectAsState().value
                        ) {
                            ConnectionStatus.Disconnected,
                            ConnectionStatus.AwaitingConnection,
                            is ConnectionStatus.Connecting,
                            -> Text(getString(R.string.awaiting_connection))

                            is ConnectionStatus.Connected -> {
                                val enabledSettingIds = enabledSettingIds.collectAsState().value
                                if (enabledSettingIds != null) {
                                    val device = connectionStatus.deviceManager.device
                                    SettingToggles(
                                        enabledSettingIds = enabledSettingIds,
                                        settingCategories = device.categories()
                                            .map { categoryId ->
                                                Pair(categoryId, device.settingsInCategory(categoryId))
                                            },
                                        onToggle = { settingId, isEnabled ->
                                            setSettingIdEnabled(appWidgetId, device.model(), settingId, isEnabled)
                                        },
                                    )
                                } else {
                                    Text(getString(R.string.loading))
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    override fun onDestroy() {
        super.onDestroy()
        unbindService(deviceServiceConnection)
    }

    private fun setSettingIdEnabled(appWidgetId: Int, model: String, settingId: String, isEnabled: Boolean) {
        val context = this
        lifecycleScope.launch {
            val manager = GlanceAppWidgetManager(context)
            val widget = SettingWidget()
            val glanceId = manager.getGlanceIdBy(appWidgetId)
            val settingIdsKey = SettingWidget.settingIdsKey(model)

            updateAppWidgetState(context, glanceId) { state ->
                val settingIds = state[SettingWidget.settingIdsKey(model)]
                val newSettingIds = if (isEnabled) {
                    settingIds?.toMutableSet()?.apply { add(settingId) } ?: setOf(settingId)
                } else {
                    settingIds?.toMutableSet()?.apply { remove(settingId) } ?: emptySet()
                }

                state[SettingWidget.settingIdsKey(model)] = newSettingIds
            }
            val preferences = widget.getAppWidgetState<Preferences>(context, glanceId)
            enabledSettingIds.value = preferences[settingIdsKey] ?: emptySet()

            updateSettingWidgets(context, session, deviceServiceConnection.connectionStatusFlow.value)
            widget.update(context, glanceId)
        }
    }
}

@Composable
private fun SettingToggles(
    enabledSettingIds: Set<String>,
    settingCategories: List<Pair<String, List<String>>>,
    onToggle: (settingId: String, isEnabled: Boolean) -> Unit,
) {
    settingCategories.forEach { (categoryId, settingIds) ->
        Column {
            TitledCard(translateCategoryId(categoryId)) {
                Column {
                    settingIds.forEach { settingId ->
                        SettingToggle(
                            name = translateSettingId(settingId),
                            isEnabled = enabledSettingIds.contains(settingId),
                            onChange = { onToggle(settingId, it) },
                        )
                    }
                }
            }
        }
    }
}

@Composable
private fun TitledCard(title: String, body: @Composable () -> Unit) {
    Column {
        Text(
            modifier = Modifier.padding(horizontal = 4.dp, vertical = 12.dp),
            text = title,
            style = MaterialTheme.typography.titleMedium,
        )
        Card {
            body()
        }
    }
}

@Composable
private fun SettingToggle(name: String, isEnabled: Boolean, onChange: (Boolean) -> Unit) {
    Row(
        modifier = Modifier
            .toggleable(value = isEnabled, onValueChange = { onChange(it) })
            .fillMaxWidth()
            .padding(horizontal = 4.dp, vertical = 12.dp),
        verticalAlignment = Alignment.CenterVertically,
        horizontalArrangement = Arrangement.SpaceBetween,
    ) {
        Text(name)
        Switch(
            checked = isEnabled,
            onCheckedChange = null,
        )
    }
}
