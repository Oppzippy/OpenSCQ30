package com.oppzippy.openscq30

import androidx.compose.runtime.collectAsState
import androidx.compose.ui.test.SemanticsMatcher
import androidx.compose.ui.test.assertHasClickAction
import androidx.compose.ui.test.hasContentDescriptionExactly
import androidx.compose.ui.test.hasText
import androidx.compose.ui.test.junit4.createAndroidComposeRule
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
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
    val mockkRule = MockKRule(this)

    @get:Rule(order = 1)
    val hiltRule = HiltAndroidRule(this)

    @get:Rule(order = 2)
    val composeRule = createAndroidComposeRule<TestActivity>()

    private lateinit var noDevicesFound: SemanticsMatcher
    private lateinit var refreshButton: SemanticsMatcher

    @Before
    fun initialize() {
        hiltRule.inject()

        noDevicesFound = hasText(composeRule.activity.getString(R.string.no_devices_found))
        refreshButton =
            hasContentDescriptionExactly(composeRule.activity.getString(R.string.refresh))
    }

    @Test
    fun testWithNoDevices() {
        composeRule.setContent {
            DeviceSelection(
                devices = emptyList(),
            )
        }

        composeRule.onNode(noDevicesFound).assertExists()
    }

    @Test
    fun testWithDevices() {
        val deviceModels = listOf(
            BluetoothDevice("test1", "00:00:00:00:00:00"),
            BluetoothDevice("test2", "00:00:00:00:00:01"),
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

        composeRule.onNode(noDevicesFound).assertDoesNotExist()
    }

    @Test
    fun testWithNoDevicesAndThenRefreshWithDevices() {
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

        composeRule.onNode(noDevicesFound).assertExists()

        devices = listOf(
            BluetoothDevice("test", "00:00:00:00:00:00"),
        )
        composeRule.onNode(noDevicesFound).assertExists()

        composeRule.onNode(refreshButton).performClick()

        devices.forEach {
            composeRule.onNodeWithText(it.name).assertExists().assertHasClickAction()
            composeRule.onNodeWithText(it.address).assertExists().assertHasClickAction()
        }

        composeRule.onNode(noDevicesFound).assertDoesNotExist()
    }
}
