package com.oppzippy.openscq30

import androidx.compose.ui.test.SemanticsMatcher
import androidx.compose.ui.test.hasTextExactly
import androidx.compose.ui.test.junit4.createAndroidComposeRule
import com.oppzippy.openscq30.ui.devicesettings.DeviceSettingsScreen
import com.oppzippy.openscq30.ui.devicesettings.models.UiDeviceState
import dagger.hilt.android.testing.HiltAndroidRule
import dagger.hilt.android.testing.HiltAndroidTest
import io.mockk.junit4.MockKRule
import org.junit.Before
import org.junit.Rule
import org.junit.Test

@HiltAndroidTest
class DeviceSettingsLoadingTest {
    @get:Rule
    val mockkRule = MockKRule(this)

    @get:Rule(order = 1)
    val hiltRule = HiltAndroidRule(this)

    @get:Rule(order = 2)
    val composeRule = createAndroidComposeRule<TestActivity>()

    private lateinit var loading: SemanticsMatcher
    private lateinit var disconnected: SemanticsMatcher

    @Before
    fun setUp() {
        hiltRule.inject()

        loading = hasTextExactly(composeRule.activity.getString(R.string.loading))
        disconnected = hasTextExactly(composeRule.activity.getString(R.string.disconnected))
    }

    @Test
    fun showsLoadingScreen() {
        composeRule.setContent {
            DeviceSettingsScreen(deviceState = UiDeviceState.Loading)
        }

        composeRule.onNode(loading).assertExists()
    }

    @Test
    fun showsDisconnectedScreen() {
        composeRule.setContent {
            DeviceSettingsScreen(deviceState = UiDeviceState.Disconnected)
        }

        composeRule.onNode(disconnected).assertExists()
    }
}
