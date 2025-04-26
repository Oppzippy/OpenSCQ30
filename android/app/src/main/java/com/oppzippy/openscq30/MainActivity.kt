package com.oppzippy.openscq30

import android.Manifest
import android.app.Activity
import android.bluetooth.BluetoothDevice
import android.companion.CompanionDeviceManager
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Bundle
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
class MainActivity @Inject constructor(private val session: OpenScq30Session) : ComponentActivity() {
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
                val deviceModel = intent.getStringExtra("deviceModel") ?: return
                if (ActivityCompat.checkSelfPermission(
                        this,
                        Manifest.permission.BLUETOOTH_CONNECT,
                    ) != PackageManager.PERMISSION_GRANTED
                ) {
                    // TODO: Consider calling
                    //    ActivityCompat#requestPermissions
                    // here to request the missing permissions, and then overriding
                    //   public void onRequestPermissionsResult(int requestCode, String[] permissions,
                    //                                          int[] grantResults)
                    // to handle the case where the user grants the permission. See the documentation
                    // for ActivityCompat#requestPermissions for more details.
                    return
                }
                deviceToPair.createBond()
                lifecycleScope.launch {
                    session.pair(
                        PairedDevice(
                            name = deviceToPair.name,
                            macAddress = deviceToPair.address,
                            model = deviceModel,
                        ),
                    )
                }
            }
        }
    }
}
