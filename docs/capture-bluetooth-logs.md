# Capture Bluetooth Logs on Android

1. [Enable developer options](https://developer.android.com/studio/debug/dev-options)
2. Turn off bluetooth
3. In developer options, set "Enable Bluetooth HCI snoop log" to Enabled.
4. Turn on bluetooth and connect to your device. Don't have any audio playing to avoid unnecessary data in the log.
5. Open the app for your device and perform actions
6. When you're done, in developer options, generate a bug report and then turn off "Enable Bluetooth HCI snoop log"
7. In the bug report zip file, the log is located at one of these paths, depending on your Android version: `FS/data/misc/bluetooth/logs/btsnoop_hci.log`, `FS\data\log\bt\btsnoop_hci.log`
