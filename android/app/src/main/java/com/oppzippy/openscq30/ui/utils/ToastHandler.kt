package com.oppzippy.openscq30.ui.utils

import android.widget.Toast
import androidx.annotation.StringRes
import androidx.compose.runtime.Composable
import androidx.compose.runtime.SideEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.platform.LocalContext
import javax.inject.Inject
import javax.inject.Singleton
import kotlinx.coroutines.flow.MutableStateFlow

@Singleton
class ToastHandler @Inject constructor() {
    private val toastRequestFlow = MutableStateFlow<ToastRequest?>(null)

    fun add(@StringRes resId: Int, duration: Int) {
        toastRequestFlow.value = ToastRequest.StringResource(resId, duration)
    }

    fun add(message: String, duration: Int) {
        toastRequestFlow.value = ToastRequest.RawString(message, duration)
    }

    @Composable
    fun Show() {
        toastRequestFlow.collectAsState().value?.let { toastRequest ->
            val context = LocalContext.current
            SideEffect {
                toastRequestFlow.value = null
                when (toastRequest) {
                    is ToastRequest.RawString -> Toast.makeText(context, toastRequest.message, toastRequest.duration)
                    is ToastRequest.StringResource -> Toast.makeText(context, toastRequest.resId, toastRequest.duration)
                }.show()
            }
        }
    }
}

private sealed class ToastRequest {
    data class StringResource(@StringRes val resId: Int, val duration: Int) : ToastRequest()
    data class RawString(val message: String, val duration: Int) : ToastRequest()
}
