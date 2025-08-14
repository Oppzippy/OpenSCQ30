package com.oppzippy.openscq30.ui.utils

import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch

@Composable
fun <T> throttledState(value: T, duration: Long, onValueChange: (T) -> Unit): Pair<T, (T) -> Unit> {
    var displayedValue by remember { mutableStateOf(value) }
    val scope = rememberCoroutineScope()
    var job by remember { mutableStateOf<Job?>(null) }
    DisposableEffect(value) {
        displayedValue = value
        onDispose {
            job?.cancel()
            job = null
        }
    }
    return Pair(
        displayedValue,
        {
            displayedValue = it
            if (job == null) {
                job = scope.launch {
                    delay(duration)
                    onValueChange(displayedValue)
                    job = null
                }
            }
        },
    )
}
