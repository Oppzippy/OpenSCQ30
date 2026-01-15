package com.oppzippy.openscq30.widget

import android.appwidget.AppWidgetManager
import android.content.Intent
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.selection.toggleable
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Check
import androidx.compose.material3.Button
import androidx.compose.material3.Card
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Switch
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
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
import kotlinx.serialization.json.Json

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

        lifecycleScope.launch {
            updateAppWidgetState(this@SettingWidgetConfigurationActivity, glanceId) { state ->
                state[SettingWidget.STATE_KEY] =
                    Json.encodeToString<SettingWidgetState>(SettingWidgetState.Disconnected(session.pairedDevices()))
            }
            widget.update(applicationContext, glanceId)

            // we want to exit this activity as quickly as possible, so do this in the background rather than waiting for
            // the service to bind
            sendBroadcast(
                Intent().apply {
                    `package` = packageName
                    action = DeviceService.ACTION_UPDATE_WIDGET
                    putExtra(AppWidgetManager.EXTRA_APPWIDGET_ID, appWidgetId)
                },
            )
        }

        enableEdgeToEdge()
        actionBar?.hide()
        setContent {
            Content(
                connectionStatus = deviceServiceConnection.connectionStatusFlow.collectAsState().value,
                enabledSettingIds = enabledSettingIds.collectAsState().value,
                onCancel = { cancel(appWidgetId) },
                onFinish = { finish() },
                onSetSettingIdEnabled = { deviceModel, settingId, isEnabled ->
                    setSettingIdEnabled(
                        appWidgetId,
                        deviceModel,
                        settingId,
                        isEnabled,
                    )
                },
            )
        }
    }

    fun cancel(appWidgetId: Int) {
        setResult(RESULT_CANCELED, Intent().putExtra(AppWidgetManager.EXTRA_APPWIDGET_ID, appWidgetId))
        finish()
    }

    override fun onDestroy() {
        super.onDestroy()
        try {
            unbindService(deviceServiceConnection)
        } catch (_: IllegalArgumentException) {
            // If the service was never bound, that's fine
        }
    }

    private fun setSettingIdEnabled(appWidgetId: Int, deviceModel: String, settingId: String, isEnabled: Boolean) {
        val context = this
        lifecycleScope.launch {
            val manager = GlanceAppWidgetManager(context)
            val widget = SettingWidget()
            val glanceId = manager.getGlanceIdBy(appWidgetId)
            val settingIdsKey = SettingWidget.settingIdsKey(deviceModel)

            updateAppWidgetState(context, glanceId) { state ->
                val settingIds = state[SettingWidget.settingIdsKey(deviceModel)]
                val newSettingIds = if (isEnabled) {
                    settingIds?.toMutableSet()?.apply { add(settingId) } ?: setOf(settingId)
                } else {
                    settingIds?.toMutableSet()?.apply { remove(settingId) } ?: emptySet()
                }

                state[SettingWidget.settingIdsKey(deviceModel)] = newSettingIds
            }
            val preferences = widget.getAppWidgetState<Preferences>(context, glanceId)
            enabledSettingIds.value = preferences[settingIdsKey] ?: emptySet()

            SettingWidget().updateConnectionStatus(
                context,
                session,
                deviceServiceConnection.connectionStatusFlow.value,
                glanceId,
            )
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun Content(
    connectionStatus: ConnectionStatus,
    enabledSettingIds: Set<String>?,
    onCancel: () -> Unit,
    onFinish: () -> Unit,
    onSetSettingIdEnabled: (deviceModel: String, settingId: String, isEnabled: Boolean) -> Unit,
) {
    OpenSCQ30Theme {
        Scaffold(
            modifier = Modifier.fillMaxSize(),
            topBar = {
                TopAppBar(
                    title = {
                        Text(text = stringResource(R.string.configure_widget))
                    },
                    actions = {
                        if (connectionStatus is ConnectionStatus.Connected) {
                            IconButton(onClick = { onFinish() }) {
                                Icon(
                                    imageVector = Icons.Filled.Check,
                                    contentDescription = stringResource(R.string.confirm),
                                )
                            }
                        }
                    },
                )
            },
        ) { contentPadding ->
            Box(
                modifier = Modifier
                    .fillMaxSize()
                    .padding(contentPadding),
            ) {
                when (connectionStatus) {
                    ConnectionStatus.Disconnected,
                    ConnectionStatus.AwaitingConnection,
                    is ConnectionStatus.Connecting,
                    -> Column(
                        modifier = Modifier.fillMaxSize(),
                        horizontalAlignment = Alignment.CenterHorizontally,
                        verticalArrangement = Arrangement.Center,
                    ) {
                        // With API <28, this will be shown initially when adding a widget (if not connected)
                        // since android:widgetFeatures="configuration_optional" is not available.
                        // Otherwise, the user won't be prompted for configuration until they connect to a device
                        Text(stringResource(R.string.connect_to_a_device_to_configure_the_widget))
                        Row {
                            Button(onClick = { onCancel() }) { Text(stringResource(R.string.cancel)) }
                            Spacer(Modifier.width(8.dp))
                            Button(onClick = { onFinish() }) { Text(stringResource(R.string.configure_later)) }
                        }
                    }

                    is ConnectionStatus.Connected -> {
                        if (enabledSettingIds != null) {
                            val device = connectionStatus.deviceManager.device
                            SettingToggles(
                                enabledSettingIds = enabledSettingIds,
                                settingCategories = device.categories()
                                    .map { categoryId ->
                                        Pair(categoryId, device.settingsInCategory(categoryId))
                                    },
                                onToggle = { settingId, isEnabled ->
                                    onSetSettingIdEnabled(device.model(), settingId, isEnabled)
                                },
                            )
                        } else {
                            Box(contentAlignment = Alignment.Center) { Text(stringResource(R.string.loading)) }
                        }
                    }
                }
            }
        }
    }
}

@Composable
private fun SettingToggles(
    enabledSettingIds: Set<String>,
    settingCategories: List<Pair<String, List<String>>>,
    onToggle: (settingId: String, isEnabled: Boolean) -> Unit,
) {
    Column(
        Modifier
            .fillMaxSize()
            .verticalScroll(rememberScrollState()),
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
