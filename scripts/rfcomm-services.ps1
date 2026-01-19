Add-Type -AssemblyName System.Runtime.WindowsRuntime
[Windows.Devices.Bluetooth.BluetoothDevice, Windows.Devices.Bluetooth, ContentType=WindowsRuntime] | Out-Null
[Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceService, Windows.Devices.Bluetooth, ContentType=WindowsRuntime] | Out-Null
[Windows.Devices.Bluetooth.BluetoothCacheMode, Windows.Devices.Bluetooth, ContentType=WindowsRuntime] | Out-Null
[Windows.Devices.Enumeration.DeviceInformation, Windows.Devices.Enumeration, ContentType=WindowsRuntime] | Out-Null
[Windows.Devices.Enumeration.DeviceInformationCollection, Windows.Devices.Enumeration, ContentType=WindowsRuntime] | Out-Null

$asTaskGeneric = ([System.WindowsRuntimeSystemExtensions].GetMethods() | ? { $_.Name -eq 'AsTask' -and $_.GetParameters().Count -eq 1 -and $_.GetParameters()[0].ParameterType.Name -eq 'IAsyncOperation`1' })[0]
Function Await($WinRtTask, $ResultType) {
    $asTask = $asTaskGeneric.MakeGenericMethod($ResultType)
    $netTask = $asTask.Invoke($null, @($WinRtTask))
    $netTask.Wait(-1) | Out-Null
    $netTask.Result
}

Function PromptChoice {
    param (
        [string[]]$Choices
    )

    for ($i = 0; $i -lt $Choices.Count; $i++) {
        $choice = $Choices[$i];
        Write-Host "[$($i)] $choice"
    }

    $choiceString = Read-Host "Enter selection number"
    try {
        $choice = [int]::Parse($choiceString)
        if ($choice -ge 0 -and $choice -lt $Choices.Count) {
            $choice
        } else {
            Write-Host "Selection outside of valid range"
            exit 1
        }
    } catch {
        Write-Host "You must enter a number"
        exit 1
    }
}

$deviceInfoCollection = Await ([Windows.Devices.Enumeration.DeviceInformation]::FindAllAsync(
    [Windows.Devices.Bluetooth.BluetoothDevice]::GetDeviceSelectorFromConnectionStatus([Windows.Devices.Bluetooth.BluetoothConnectionStatus]::Connected)
)) ([Windows.Devices.Enumeration.DeviceInformationCollection])

if ($deviceInfoCollection.Count -eq 0) {
    Write-Host "No bluetooth devices are connected"
    exit 1
}

$selectionIndex = PromptChoice -Choices $($deviceInfoCollection | ForEach-Object Name)
$deviceInfo = $deviceInfoCollection[$selectionIndex]

$bluetoothDevice = Await (
    [Windows.Devices.Bluetooth.BluetoothDevice]::FromIdAsync($deviceInfo.Id)
) ([Windows.Devices.Bluetooth.BluetoothDevice])

$servicesResult = Await (
    $bluetoothDevice.GetRfcommServicesAsync()
) ([Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceServicesResult])

Write-Host "$($deviceInfo.Name) RFCOMM Services:"
$servicesResult.Services | ForEach-Object { Write-Host $_.ServiceId.Uuid }

pause
