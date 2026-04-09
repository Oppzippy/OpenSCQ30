package com.oppzippy.openscq30

import android.Manifest
import android.bluetooth.BluetoothAdapter
import android.bluetooth.BluetoothDevice
import android.bluetooth.BluetoothManager
import android.content.Context
import android.content.pm.PackageManager
import android.util.Log
import androidx.core.app.ActivityCompat
import com.oppzippy.openscq30.features.soundcoredevice.AndroidRfcommConnectionBackendImpl
import com.oppzippy.openscq30.test.MainDispatcherRule
import io.mockk.clearAllMocks
import io.mockk.every
import io.mockk.junit4.MockKRule
import io.mockk.mockk
import io.mockk.mockkStatic
import io.mockk.verify
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.test.runTest
import org.junit.After
import org.junit.Assert
import org.junit.Rule
import org.junit.Test

class ConnectionBackends {
    @get:Rule
    val mockkRule = MockKRule(this)

    @get:Rule
    val mainDispatcherRule = MainDispatcherRule()

    @After
    fun tearDown() {
        clearAllMocks()
    }

    @Test
    fun testListDevices() = runTest {
        var i = 0
        while (testListDevicesUpToFailurePoint(this, i)) {
            i += 1
        }
    }

    /**
     * @return true if we have not yet reached the maximum value for maxIndex
     */
    suspend fun testListDevicesUpToFailurePoint(coroutineScope: CoroutineScope, maxIndex: Int): Boolean {
        println("running test $maxIndex")

        clearAllMocks()
        mockkStatic(Log::class)
        every { Log.d(any<String>(), any<String>()) } returns 1
        every { Log.i(any<String>(), any<String>()) } returns 1
        every { Log.e(any<String>(), any<String>()) } returns 1
        every { Log.w(any<String>(), any<String>()) } returns 1

        mockkStatic(ActivityCompat::class)
        mockkStatic(ActivityCompat::checkSelfPermission)

        val context: Context = mockk()
        val bluetoothManager: BluetoothManager = mockk()
        val adapter: BluetoothAdapter = mockk()

        val firstDevice = mockk<BluetoothDevice>()
        every { firstDevice.name } returns "First Device"
        every { firstDevice.address } returns "00:00:00:00:00:00"

        val secondDevice = mockk<BluetoothDevice>()
        every { secondDevice.name } returns "Second Device"
        every { secondDevice.address } returns "00:00:00:00:00:01"

        val bondedDevices = setOf(firstDevice, secondDevice)

        every { bluetoothManager.adapter } returns adapter

        val backend = AndroidRfcommConnectionBackendImpl(context, coroutineScope)
        val conditions = listOf(
            Condition(
                name = "null BluetoothManager",
                setSuccess = {
                    every { context.getSystemService(BluetoothManager::class.java) } returns bluetoothManager
                },
                setFailure = {
                    every { context.getSystemService(BluetoothManager::class.java) } returns null
                },
                assertFailureHandled = { verify(exactly = 1) { Log.e(any<String>(), any<String>()) } },
            ),
            Condition(
                name = "no permission",
                setSuccess = {
                    every {
                        ActivityCompat.checkSelfPermission(context, Manifest.permission.BLUETOOTH_CONNECT)
                    } returns PackageManager.PERMISSION_GRANTED
                },
                setFailure = {
                    every {
                        ActivityCompat.checkSelfPermission(context, Manifest.permission.BLUETOOTH_CONNECT)
                    } returns PackageManager.PERMISSION_DENIED
                },
                assertFailureHandled = { verify(exactly = 1) { Log.e(any<String>(), any<String>()) } },
            ),
            Condition(
                name = "null bondedDevices",
                setSuccess = { every { adapter.bondedDevices } returns bondedDevices },
                setFailure = { every { adapter.bondedDevices } returns null },
                assertFailureHandled = { verify(exactly = 1) { Log.e(any<String>(), any<String>()) } },
            ),
            Condition(
                name = "null name on a device",
                setSuccess = {},
                setFailure = {
                    val firstDevice = bondedDevices.first()
                    every { firstDevice.name } returns null
                    every { context.getString(R.string.unknown) } returns "Unknown"
                },
                assertFailureHandled = {
                    verify(exactly = 1) { Log.w(any<String>(), any<String>()) }
                    verify(exactly = 1) { context.getString(R.string.unknown) }
                },
                expectedReturnSetSize = 2,
            ),
        )

        conditions.forEachIndexed { index, condition ->
            if (index < maxIndex) {
                println("setting ${condition.name} to success")
                condition.setSuccess()
            } else {
                condition.setFailure()
            }
        }

        val expectedReturnSetSize = conditions.getOrNull(maxIndex)?.expectedReturnSetSize ?: 2

        Assert.assertEquals(expectedReturnSetSize, backend.devices().size)

        return maxIndex <= conditions.size
    }
}

data class Condition(
    val name: String,
    val setSuccess: () -> Unit,
    val setFailure: () -> Unit,
    val assertFailureHandled: () -> Unit,
    val expectedReturnSetSize: Int = 0,
)
