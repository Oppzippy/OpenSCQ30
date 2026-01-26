package com.oppzippy.openscq30

import android.content.Intent
import android.os.Build
import android.os.Bundle
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.appcompat.app.AppCompatActivity
import androidx.lifecycle.lifecycleScope
import com.oppzippy.openscq30.lib.bindings.OpenScq30Session
import com.oppzippy.openscq30.ui.OpenSCQ30Root
import dagger.hilt.android.AndroidEntryPoint
import javax.inject.Inject
import kotlinx.coroutines.launch

@AndroidEntryPoint
class MainActivity : AppCompatActivity() {
    companion object {
        // HACK: On android versions <33, CompanionDeviceManager.Callback lacks onAssociationCreated, so we rely on
        // onActivityResult. The problem is, I'm not sure the proper way of passing data to the activity from
        // onDeviceFound, so we set this global callback as a workaround.
        var onPaired: (() -> Unit)? = null
    }

    @Inject
    lateinit var session: OpenScq30Session

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {
            OpenSCQ30Root()
        }
    }

    @Deprecated(
        "Deprecated in Java",
        ReplaceWith(
            "super.onActivityResult(requestCode, resultCode, data)",
            "androidx.activity.ComponentActivity",
        ),
    )
    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        super.onActivityResult(requestCode, resultCode, data)
        // only used when CompanionDeviceManager.Callback::onAssociationCreated isn't supported
        if (requestCode == 0 && resultCode == RESULT_OK && data != null &&
            Build.VERSION.SDK_INT < Build.VERSION_CODES.TIRAMISU
        ) {
            lifecycleScope.launch {
                onPaired?.let { it() }
            }
        }
    }
}
