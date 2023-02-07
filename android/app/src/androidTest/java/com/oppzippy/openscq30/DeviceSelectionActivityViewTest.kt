package com.oppzippy.openscq30

import androidx.compose.ui.test.*
import androidx.compose.ui.test.junit4.createAndroidComposeRule
import com.oppzippy.openscq30.features.ui.deviceselection.composables.DeviceSelectionPermissionCheck
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDevice
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDeviceProvider
import dagger.hilt.android.testing.HiltAndroidRule
import dagger.hilt.android.testing.HiltAndroidTest
import io.mockk.every
import io.mockk.junit4.MockKRule
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import javax.inject.Inject

@HiltAndroidTest
class DeviceSelectionActivityViewTest {
    @get:Rule
    val mockkRule = MockKRule(this)

    @get:Rule(order = 1)
    val hiltRule = HiltAndroidRule(this)

    @get:Rule(order = 2)
    val composeRule = createAndroidComposeRule<MainActivity>()

    @Inject
    lateinit var deviceProviderMock: BluetoothDeviceProvider

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
        every { deviceProviderMock.getDevices() } returns listOf()

        composeRule.setContent {
            DeviceSelectionPermissionCheck(
                bluetoothDeviceProvider = deviceProviderMock,
                onInfoClick = {},
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
        every { deviceProviderMock.getDevices() } returns deviceModels

        composeRule.setContent {
            DeviceSelectionPermissionCheck(
                bluetoothDeviceProvider = deviceProviderMock,
                onInfoClick = {},
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
        every { deviceProviderMock.getDevices() } returns listOf()

        composeRule.setContent {
            DeviceSelectionPermissionCheck(
                bluetoothDeviceProvider = deviceProviderMock,
                onInfoClick = {},
            )
        }

        composeRule.onNode(noDevicesFound).assertExists()

        val deviceModels = listOf(
            BluetoothDevice("test", "00:00:00:00:00:00"),
        )
        every { deviceProviderMock.getDevices() } returns deviceModels

        composeRule.onNode(refreshButton).performClick()

        deviceModels.forEach {
            composeRule.onNodeWithText(it.name).assertExists().assertHasClickAction()
            composeRule.onNodeWithText(it.address).assertExists().assertHasClickAction()
        }

        composeRule.onNode(noDevicesFound).assertDoesNotExist()
    }

}