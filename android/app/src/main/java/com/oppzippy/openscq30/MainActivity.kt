package com.oppzippy.openscq30

import android.Manifest
import android.app.Activity
import android.bluetooth.BluetoothDevice
import android.companion.CompanionDeviceManager
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Bundle
import android.widget.Toast
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.core.app.ActivityCompat
import androidx.core.content.IntentCompat
import androidx.lifecycle.lifecycleScope
import com.oppzippy.openscq30.lib.bindings.OpenScq30Session
import com.oppzippy.openscq30.lib.wrapper.PairedDevice
import com.oppzippy.openscq30.ui.OpenSCQ30Root
import dagger.hilt.android.AndroidEntryPoint
import javax.inject.Inject
import kotlinx.coroutines.launch

@AndroidEntryPoint
class MainActivity : ComponentActivity() {
    @Inject
    lateinit var session: OpenScq30Session

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        actionBar?.hide()
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
        when (requestCode) {
            Activity.RESULT_OK -> {
                val deviceToPair = data?.let {
                    IntentCompat.getParcelableExtra(
                        it,
                        CompanionDeviceManager.EXTRA_DEVICE,
                        BluetoothDevice::class.java,
                    )
                } ?: return
                val pairedDevice =
                    IntentCompat.getParcelableExtra(intent, "pairedDevice", PairedDevice::class.java) ?: return
                if (ActivityCompat.checkSelfPermission(
                        this,
                        Manifest.permission.BLUETOOTH_CONNECT,
                    ) != PackageManager.PERMISSION_GRANTED
                ) {
                    Toast.makeText(this, getString(R.string.bluetooth_permission_is_required), Toast.LENGTH_SHORT)
                        .show()
                    return
                }
                deviceToPair.createBond()
                lifecycleScope.launch { session.pair(pairedDevice) }
            }
        }
    }
}
