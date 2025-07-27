package com.oppzippy.openscq30.actions

import androidx.activity.ComponentActivity
import androidx.compose.ui.test.hasText
import androidx.compose.ui.test.junit4.AndroidComposeTestRule
import androidx.compose.ui.test.onNodeWithContentDescription
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.performScrollToNode
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.bindings.OpenScq30Session
import com.oppzippy.openscq30.lib.wrapper.PairedDevice
import org.junit.rules.TestRule

fun <Rule : TestRule, A : ComponentActivity> addAndConnectToDemoDevice(
    composeRule: AndroidComposeTestRule<Rule, A>,
    modelName: String,
) {
    addDemoDevice(composeRule, modelName)
    composeRule.onNodeWithText(modelName).performClick()
}

fun <Rule : TestRule, A : ComponentActivity> addDemoDevice(
    composeRule: AndroidComposeTestRule<Rule, A>,
    modelName: String,
) {
    composeRule.onNodeWithContentDescription(composeRule.activity.getString(R.string.add)).performClick()
    composeRule.onNodeWithTag("modelList").performScrollToNode(hasText(modelName))
    composeRule.onNodeWithText(modelName).performClick()
    composeRule.onNodeWithText(composeRule.activity.getString(R.string.demo_mode)).performClick()
    composeRule.onNodeWithText(modelName).performClick()
}
