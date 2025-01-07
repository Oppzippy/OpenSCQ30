package com.oppzippy.openscq30.features.soundcoredevice

import android.bluetooth.BluetoothDevice
import android.bluetooth.BluetoothGatt
import android.bluetooth.BluetoothManager
import android.bluetooth.BluetoothSocket
import android.bluetooth.BluetoothSocketException
import android.content.Context
import android.os.ParcelUuid
import android.util.Log
import com.oppzippy.openscq30.features.soundcoredevice.impl.BluetoothDeviceFinder
import com.oppzippy.openscq30.features.soundcoredevice.impl.SoundcoreDeviceCallbackHandler
import com.oppzippy.openscq30.features.soundcoredevice.impl.SoundcoreDeviceConnectorImpl
import com.oppzippy.openscq30.lib.bindings.ManualConnection
import com.oppzippy.openscq30.lib.bindings.newSoundcoreDevice
import io.mockk.clearAllMocks
import io.mockk.coEvery
import io.mockk.every
import io.mockk.impl.annotations.MockK
import io.mockk.junit4.MockKRule
import io.mockk.just
import io.mockk.justRun
import io.mockk.mockk
import io.mockk.mockkConstructor
import io.mockk.mockkStatic
import io.mockk.runs
import io.mockk.slot
import io.mockk.verify
import java.io.InputStream
import java.io.OutputStream
import java.util.UUID
import java.util.concurrent.TimeoutException
import kotlin.reflect.jvm.kotlinFunction
import kotlin.time.Duration.Companion.hours
import kotlin.time.Duration.Companion.seconds
import kotlinx.coroutines.cancelAndJoin
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.launch
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import kotlinx.coroutines.test.runTest
import org.junit.After
import org.junit.Assert
import org.junit.Before
import org.junit.Rule
import org.junit.Test

class SoundcoreDeviceConnectorImplTest {
    @get:Rule
    val mockkRule = MockKRule(this)

    @MockK(relaxed = true)
    private lateinit var context: Context

    @MockK
    private lateinit var deviceFinder: BluetoothDeviceFinder

    @MockK
    private lateinit var bluetoothDevice: BluetoothDevice

    @MockK
    private lateinit var socket: BluetoothSocket

    @MockK(relaxed = true)
    private lateinit var gatt: BluetoothGatt

    @MockK
    private lateinit var manualConnection: ManualConnection

    private lateinit var connector: SoundcoreDeviceConnectorImpl

    private val macAddress = "00:11:22:33:44:55"
    private val inputStream = InputStream.nullInputStream()
    private val outputStream = OutputStream.nullOutputStream()

    @Before
    fun setUp() {
        mockkStatic(
            Log::class.java.getDeclaredMethod(
                "d",
                String::class.java,
                String::class.java,
            ).kotlinFunction!!,
        )
        mockkStatic(
            Log::class.java.getDeclaredMethod(
                "i",
                String::class.java,
                String::class.java,
            ).kotlinFunction!!,
        )
        every { Log.d(any(), any()) } returns 0
        val stringSlot = slot<String>()
        every { Log.i(any(), capture(stringSlot)) } answers {
            println(stringSlot.captured)
            0
        }

        mockkConstructor(ManualConnection::class)
        mockkConstructor(SoundcoreDeviceCallbackHandler::class)

        every { context.getSystemService(BluetoothManager::class.java) } answers {
            val manager = mockk<BluetoothManager>()
            every { manager.adapter } returns mockk()
            manager
        }
        coEvery { deviceFinder.findByMacAddress(macAddress) } returns bluetoothDevice

        connector = SoundcoreDeviceConnectorImpl(
            context = context,
            deviceFinder = deviceFinder,
            fallbackRfcommUuid = UUID(0, 0),
            isVendorRfcommUuid = { true },
            createManualConnection = { _, _, _, _ -> manualConnection },
        )
        coEvery {
            bluetoothDevice.connectGatt(
                context,
                false,
                any(),
                BluetoothDevice.TRANSPORT_LE,
            )
        } returns gatt
        every { bluetoothDevice.createRfcommSocketToServiceRecord(any()) } returns socket
        every { bluetoothDevice.address } returns macAddress
        every { bluetoothDevice.name } returns "Demo Device"
        val uuid = mockk<ParcelUuid>()
        every { uuid.uuid } returns UUID(0, 0)
        every { bluetoothDevice.uuids } returns arrayOf(uuid)

        justRun { socket.connect() }
        every { socket.inputStream } returns inputStream
        every { socket.outputStream } returns outputStream
    }

    @After
    fun tearDown() {
        inputStream.close()
        outputStream.close()
        clearAllMocks()
    }

    @Test
    fun shouldCleanUpResourcesOnCancellationDuringWaitForReady() = runTest {
        // Will unlock when the job starts
        val readyLock = Mutex(true)
        coEvery { anyConstructed<SoundcoreDeviceCallbackHandler>().waitUntilReady() } coAnswers {
            readyLock.unlock()
            delay(1.seconds)
        }

        val job = launch {
            connector.connectToSoundcoreDevice(macAddress, this)
        }

        // Wait for the job to start, then cancel it
        readyLock.withLock { }
        job.cancelAndJoin()

        verify(exactly = 1) { gatt.disconnect() }
        verify(exactly = 1) { gatt.close() }
    }

    @Test
    fun shouldCleanUpResourcesOnWaitForReadyTimeout() = runTest {
        coEvery { anyConstructed<SoundcoreDeviceCallbackHandler>().waitUntilReady() } coAnswers {
            // wait for a greater amount of time than the timeout
            delay(1.hours)
        }

        try {
            connector.connectToSoundcoreDevice(macAddress, this)
        } catch (_: TimeoutException) {
        }

        verify(exactly = 1) { gatt.disconnect() }
        verify(exactly = 1) { gatt.close() }
    }

    @Test
    fun shouldCleanUpResourcesOnNativeDeviceException() = runTest {
        coEvery { anyConstructed<SoundcoreDeviceCallbackHandler>().waitUntilReady() } just runs
        coEvery { anyConstructed<SoundcoreDeviceCallbackHandler>().serviceUuid } returns MutableStateFlow(
            UUID(0, 0),
        )

        val exception = Exception()
        mockkStatic(::newSoundcoreDevice)
        coEvery { newSoundcoreDevice(any()) } throws exception
        every { manualConnection.close() } just runs

        try {
            connector.connectToSoundcoreDevice(macAddress, this)
        } catch (ex: Exception) {
            // Allow our expected exception, fail on any other exception
            if (ex !== exception) {
                throw ex
            }
        }

        verify(exactly = 1) { gatt.disconnect() }
        verify(exactly = 1) { gatt.close() }
        verify(exactly = 1) { manualConnection.close() }
    }

    @Test
    fun shouldThrowIfRfcommConnectionFails() = runTest {
        every { socket.connect() } throws BluetoothSocketException(BluetoothSocketException.UNSPECIFIED)
        justRun { manualConnection.close() }

        try {
            connector.connectToSoundcoreDevice(macAddress, this)
            Assert.fail("exception not thrown")
        } catch (_: BluetoothSocketException) {
        }
    }

    @Test
    fun shouldNotThrowIfGattServiceUuidIsNotFound() = runTest {
        coEvery { anyConstructed<SoundcoreDeviceCallbackHandler>().waitUntilReady() } just runs
        coEvery { anyConstructed<SoundcoreDeviceCallbackHandler>().serviceUuid } returns MutableStateFlow(null)
        coEvery { anyConstructed<SoundcoreDeviceCallbackHandler>().isDisconnected } returns MutableStateFlow(false)
        mockkStatic(::newSoundcoreDevice)
        coEvery { newSoundcoreDevice(any()) } returns mockk(relaxed = true)

        val device = connector.connectToSoundcoreDevice(macAddress, this)
        Assert.assertNotNull(device)

        justRun { socket.close() }
        justRun { manualConnection.close() }
        device?.close()
    }
}
