package com.oppzippy.openscq30

import android.os.Build
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.test.SemanticsMatcher
import androidx.compose.ui.test.assertHasClickAction
import androidx.compose.ui.test.hasContentDescriptionExactly
import androidx.compose.ui.test.junit4.createAndroidComposeRule
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.test.rule.GrantPermissionRule
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDevice
import com.oppzippy.openscq30.ui.deviceselection.composables.DeviceSelection
import dagger.hilt.android.testing.HiltAndroidRule
import dagger.hilt.android.testing.HiltAndroidTest
import io.mockk.junit4.MockKRule
import kotlinx.coroutines.flow.MutableStateFlow
import org.junit.Before
import org.junit.Rule
import org.junit.Test

@HiltAndroidTest
class DeviceSelectionTest {
    @get:Rule
    val permissionRule: GrantPermissionRule = GrantPermissionRule.grant(
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            android.Manifest.permission.BLUETOOTH_CONNECT
        } else {
            android.Manifest.permission.BLUETOOTH
        },
    )

    @get:Rule
    val mockkRule = MockKRule(this)

    @get:Rule(order = 1)
    val hiltRule = HiltAndroidRule(this)

    @get:Rule(order = 2)
    val composeRule = createAndroidComposeRule<TestActivity>()

    private lateinit var refreshButton: SemanticsMatcher

    @Before
    fun setUp() {
        hiltRule.inject()

        refreshButton =
            hasContentDescriptionExactly(composeRule.activity.getString(R.string.refresh))
    }

    @Test
    fun showsAllAvailableDevices() {
        val deviceModels = listOf(
            BluetoothDevice("test1", "00:00:00:00:00:00", true),
            BluetoothDevice("test2", "00:00:00:00:00:01", true),
        )

        composeRule.setContent {
            DeviceSelection(
                devices = deviceModels,
            )
        }

        deviceModels.forEach {
            composeRule.onNodeWithText(it.name).assertExists().assertHasClickAction()
            composeRule.onNodeWithText(it.address).assertExists().assertHasClickAction()
        }
    }

    @Test
    fun addsDevicesToTheListWhenRefreshIsClicked() {
        var devices: List<BluetoothDevice> = emptyList()
        val devicesFlow = MutableStateFlow(devices)

        composeRule.setContent {
            DeviceSelection(
                devices = devicesFlow.collectAsState().value,
                onRefreshDevices = {
                    devicesFlow.value = devices
                },
            )
        }

        devices = listOf(
            BluetoothDevice("test", "00:00:00:00:00:00", true),
        )
        composeRule.onNodeWithText("00:00:00:00:00:00").assertDoesNotExist()

        composeRule.onNode(refreshButton).performClick()

        devices.forEach {
            composeRule.onNodeWithText(it.name).assertExists().assertHasClickAction()
            composeRule.onNodeWithText(it.address).assertExists().assertHasClickAction()
        }

        composeRule.onNodeWithText("00:00:00:00:00:00").assertExists()
    }
}
