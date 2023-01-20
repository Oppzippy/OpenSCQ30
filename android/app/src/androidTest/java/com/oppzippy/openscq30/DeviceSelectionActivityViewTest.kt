package com.oppzippy.openscq30

import androidx.activity.ComponentActivity
import androidx.compose.ui.test.*
import androidx.compose.ui.test.junit4.createAndroidComposeRule
import androidx.test.ext.junit.runners.AndroidJUnit4
import com.oppzippy.openscq30.ui.deviceselection.BluetoothDeviceModel
import com.oppzippy.openscq30.ui.deviceselection.BluetoothDeviceProvider
import com.oppzippy.openscq30.ui.deviceselection.DeviceSelectionActivityView
import io.mockk.every
import io.mockk.mockk

import org.junit.Test
import org.junit.runner.RunWith

import org.junit.Assert.*
import org.junit.Before
import org.junit.Rule

@RunWith(AndroidJUnit4::class)
class DeviceSelectionActivityViewTest {
    @get:Rule
    val composeRule = createAndroidComposeRule<ComponentActivity>()

    private lateinit var noDevicesFound: SemanticsMatcher
    private lateinit var refreshButton: SemanticsMatcher

    @Before
    fun initialize() {
        noDevicesFound = hasText(composeRule.activity.getString(R.string.no_devices_found))
        refreshButton = hasContentDescriptionExactly(composeRule.activity.getString(R.string.refresh))
    }


    @Test
    fun testWithNoDevices() {
        val deviceProviderMock = mockk<BluetoothDeviceProvider>()
        every { deviceProviderMock.getDevices() } returns listOf()

        composeRule.setContent {
            DeviceSelectionActivityView(bluetoothDeviceProvider = deviceProviderMock)
        }

        composeRule.onNode(noDevicesFound).assertExists()
    }

    @Test
    fun testWithDevices() {
        val deviceProviderMock = mockk<BluetoothDeviceProvider>()
        val deviceModels = listOf(
            BluetoothDeviceModel("test", "00:00:00:00:00:00"),
            BluetoothDeviceModel("test2", "00:00:00:00:00:01"),
        )
        every { deviceProviderMock.getDevices() } returns deviceModels

        composeRule.setContent {
            DeviceSelectionActivityView(bluetoothDeviceProvider = deviceProviderMock)
        }

        deviceModels.forEach {
            composeRule.onNodeWithText(it.name).assertExists().assertHasClickAction()
            composeRule.onNodeWithText(it.address).assertExists().assertHasClickAction()
        }

        composeRule.onNode(noDevicesFound).assertDoesNotExist()
    }

    @Test
    fun testWithNoDevicesAndThenRefreshWithDevices() {
        val deviceProviderMock = mockk<BluetoothDeviceProvider>()
        every { deviceProviderMock.getDevices() } returns listOf()

        composeRule.setContent {
            DeviceSelectionActivityView(bluetoothDeviceProvider = deviceProviderMock)
        }

        composeRule.onNode(noDevicesFound).assertExists()

        val deviceModels = listOf(
            BluetoothDeviceModel("test", "00:00:00:00:00:00"),
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