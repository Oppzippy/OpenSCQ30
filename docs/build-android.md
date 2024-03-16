## Building OpenSCQ30 on Android

### If you do not have Android Studio installed:

1. Install a JDK distribution of your choice
2. Download the [Android command line tools (scroll down a bit)](https://developer.android.com/studio), extract it, and set the `ANDROID_HOME` environment variable to the cmdline-tools directory (the one containing `NOTICE.txt`).
3. Accept licenses by `cd`ing to cmdline-tools/bin and run `./sdkmanager --licenses --sdk_root=..`.
4. Check the version of ndk listed in [`android/app/build.gradle`](https://github.com/Oppzippy/OpenSCQ30/blob/master/android/app/build.gradle) (Ctrl-F ndkVersion) and [download that version of the ndk](https://developer.android.com/ndk/downloads), extract it, and set the `ANDROID_NDK_HOME` environment variable to the directory inside containing `NOTICE`.

### If you do have Android Studio installed:

1. Ensure the version of ndk listed in [`android/app/build.gradle`](https://github.com/Oppzippy/OpenSCQ30/blob/master/android/app/build.gradle) (Ctrl-F ndkVersion) is installed (File -> Settings -> Appearance & Behavior -> System Settings -> Android SDK -> SDK Tools).

### Then:

1. Install rustup
2. Add all supported cpu architecture targets: `rustup target add armv7-linux-androideabi aarch64-linux-android i686-linux-android x86_64-linux-android`
3. Install [cargo-ndk](https://github.com/bbqsrc/cargo-ndk): `cargo install cargo-ndk`
4. In the `android` directory, run `./gradlew assembleRelease`
5. The apk can be found at `android/app/build/outputs/apk/release/app-release-unsigned.apk`
