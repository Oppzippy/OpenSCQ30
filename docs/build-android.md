## Building OpenSCQ30 on Android

### If you do not have Android Studio installed:

1. Install a JDK distribution of your choice
2. Download the [Android command line tools (scroll down a bit)](https://developer.android.com/studio), extract it to `~/Android/Sdk` (or somewhere else, this is just where Android Studio puts it), and set the `ANDROID_HOME` environment variable to `~/Android/Sdk`.
3. Accept licenses by running `$ANDROID_HOME/cmdline-tools/bin/sdkmanager --licenses --sdk_root=$ANDROID_HOME`.
4. Check the ndk version required for the commit you have checked out (it's listed in listed in [`android/app/build.gradle.kts`](https://github.com/Oppzippy/OpenSCQ30/blob/master/android/app/build.gradle.kts), Ctrl-F ndkVersion) and [download that version of the ndk](https://developer.android.com/ndk/downloads), extract it to `$ANDROID_HOME/ndk/your_ndk_version_number` (and ensure symlinks are preserved), and set the `ANDROID_NDK_HOME` environment variable to `$ANDROID_HOME/ndk/your_ndk_version_number`.

### If you do have Android Studio installed:

1. Ensure the version of ndk listed in [`android/app/build.gradle.kts`](https://github.com/Oppzippy/OpenSCQ30/blob/master/android/app/build.gradle.kts) (Ctrl-F ndkVersion) is installed (File -> Settings -> Appearance & Behavior -> System Settings -> Android SDK -> SDK Tools).

### Then:

If it's inconvenient to install the latest version of [just](https://github.com/casey/just), use the without just instructions. The catch is that the without just instructions are more likely to change in the future, so if you're packaging openscq30 and the latest version of just is easily available, prefer the with just instructions.

1. Install rustup
2. Add all supported cpu architecture targets that you plan to build: `rustup target add armv7-linux-androideabi aarch64-linux-android i686-linux-android x86_64-linux-android`
3. Install [cargo-ndk](https://github.com/bbqsrc/cargo-ndk): `cargo install cargo-ndk`

#### Without just:

4. In the `android` directory, run `./gradlew assembleRelease` for a universal apk, or one or more of these for a specific cpu architecture:

- `./gradlew assembleRelease-arm64-v8a`
- `./gradlew assembleRelease-armeabi-v7a`
- `./gradlew assembleRelease-x86_64`
- `./gradlew assembleRelease-x86`

5. The apk can be found at `android/app/build/outputs/apk/release/app-release-unsigned.apk` for a universal apk or `android/app/build/outputs/apk/release-arm64-v8a/app-release-arm64-v8a-unsigned.apk` for example otherwise.

#### With just:

4. In the project root directory, run one or more of these commands:

- `just build-android`: builds all of the following
- `just build-android-universal`: builds a single apk supporting all of the following architectures
- `just build-android-arm64-v8a`
- `just build-android-armeabi-v7a`
- `just build-android-x86_64`
- `just build-android-x86`

5. The apks can be found at `build-output/openscq30-android-*.apk`
