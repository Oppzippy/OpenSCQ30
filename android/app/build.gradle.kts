import com.android.build.gradle.internal.tasks.factory.dependsOn
import java.io.FileInputStream
import java.util.Properties
import org.gradle.kotlin.dsl.kotlin
import org.jetbrains.kotlin.gradle.dsl.JvmTarget

plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("org.jetbrains.kotlin.plugin.compose")
    id("org.jetbrains.kotlin.kapt")
    id("com.google.devtools.ksp")
    id("com.google.dagger.hilt.android")
    kotlin("plugin.serialization")
    id("kotlin-parcelize")
}

val keystorePropertiesFile: File = rootProject.file("keystore.properties")
val keystoreProperties = Properties()
if (keystorePropertiesFile.exists()) {
    keystoreProperties.load(FileInputStream(keystorePropertiesFile))
}

android {
    signingConfigs {
        if (keystorePropertiesFile.exists()) {
            create("release") {
                storeFile = file(keystoreProperties["storeFile"] as String)
                storePassword = keystoreProperties["storePassword"] as String
                keyPassword = keystoreProperties["keyPassword"] as String
                keyAlias = keystoreProperties["keyAlias"] as String
            }
        }
    }
    namespace = "com.oppzippy.openscq30"
    compileSdk = 36
    buildToolsVersion = "36.1.0"

    defaultConfig {
        applicationId = "com.oppzippy.openscq30"
        minSdk = 26
        targetSdk = 36
        versionCode = 1006
        versionName = "2.0.0-beta7"

        testInstrumentationRunner = "com.oppzippy.openscq30.HiltTestRunner"
        vectorDrawables {
            useSupportLibrary = true
        }

        ndk {
            this.abiFilters.clear()
            this.abiFilters.addAll(listOf("armeabi-v7a", "arm64-v8a", "x86", "x86_64"))
        }
    }

    sourceSets {
        getByName("debug") {
            jniLibs.srcDir("src/main/debug/jniLibs")
        }
        getByName("release") {
            jniLibs.srcDir("src/main/release/jniLibs")
        }
        getByName("main") {
            java.srcDir("${layout.buildDirectory.get()}/generated/source/uniffi/java")
        }
        getByName("androidTest") {
            assets.srcDir("$projectDir/schemas")
        }
    }

    buildTypes {
        named("debug") {
            applicationIdSuffix = ".debug"
            isDebuggable = true
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro",
            )
        }
        named("release") {
            isDebuggable = false
            isMinifyEnabled = true
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro",
            )
            if (keystorePropertiesFile.exists()) {
                signingConfig = signingConfigs["release"]
            }
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    kotlin {
        compilerOptions {
            jvmTarget = JvmTarget.JVM_17
        }
    }
    buildFeatures {
        compose = true
        buildConfig = true
    }
    ksp {
        arg("room.schemaLocation", "$projectDir/schemas")
    }
    ndkVersion = "29.0.14206865"
    packaging {
        resources {
            excludes += "/META-INF/*"
        }
    }
    splits {
        abi {
            isEnable = true
            reset()
            include("arm64-v8a", "armeabi-v7a", "x86", "x86_64")
            isUniversalApk = true
        }
    }
    androidResources {
        @Suppress("UnstableApiUsage")
        generateLocaleConfig = true
    }
}

dependencies {
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.9.0")

    val lifecycleVersion = "2.10.0"
    implementation("androidx.lifecycle:lifecycle-runtime-ktx:$lifecycleVersion")
    implementation("androidx.lifecycle:lifecycle-service:$lifecycleVersion")

    // Compose
    val composeBomVersion = "2025.11.01"
    implementation(platform("androidx.compose:compose-bom:$composeBomVersion"))
    androidTestImplementation(platform("androidx.compose:compose-bom:$composeBomVersion"))
    implementation("androidx.compose.ui:ui")
    // Material Design 3
    implementation("androidx.compose.material3:material3")
    // Android Studio Preview support
    implementation("androidx.compose.ui:ui-tooling-preview")
    debugImplementation("androidx.compose.ui:ui-tooling")
    // UI Tests
    androidTestImplementation("androidx.compose.ui:ui-test-junit4")
    debugImplementation("androidx.compose.ui:ui-test-manifest")
    // Optional - Included automatically by material, only add when you need
    // the icons but not the material library (e.g. when using Material3 or a
    // custom design system based on Foundation)
    implementation("androidx.compose.material:material-icons-core")
    // Optional - Add full set of material icons
    implementation("androidx.compose.material:material-icons-extended")
    // Optional - Integration with activities
    implementation("androidx.activity:activity-compose:1.12.0")
    // Optional - Integration with ViewModels
    implementation("androidx.lifecycle:lifecycle-viewmodel-compose:$lifecycleVersion")
    // Compose navigation
    implementation("androidx.navigation:navigation-compose:2.9.6")

    val accompanistVersion = "0.37.3"
    implementation("com.google.accompanist:accompanist-permissions:$accompanistVersion")

    val roomVersion = "2.8.4"
    implementation("androidx.room:room-runtime:$roomVersion")
    implementation("androidx.room:room-ktx:$roomVersion")
    annotationProcessor("androidx.room:room-compiler:$roomVersion")
    ksp("androidx.room:room-compiler:$roomVersion")
    androidTestImplementation("androidx.room:room-testing:$roomVersion")

    implementation("net.java.dev.jna:jna:5.18.1@aar")

    val hiltVersion = "2.57.2"
    implementation("com.google.dagger:hilt-android:$hiltVersion")
    kapt("com.google.dagger:hilt-android-compiler:$hiltVersion")
    implementation("androidx.hilt:hilt-navigation-compose:1.3.0")

    // For instrumentation tests
    androidTestImplementation("com.google.dagger:hilt-android-testing:$hiltVersion")
    kaptAndroidTest("com.google.dagger:hilt-compiler:$hiltVersion")

    // For local unit tests
    testImplementation("com.google.dagger:hilt-android-testing:$hiltVersion")
    kaptTest("com.google.dagger:hilt-compiler:$hiltVersion")

    testImplementation("junit:junit:4.13.2")

    val mockkVersion = "1.14.6"
    testImplementation("io.mockk:mockk:$mockkVersion")
    androidTestImplementation("io.mockk:mockk-android:$mockkVersion")
    androidTestImplementation("androidx.test:runner:1.7.0")
    androidTestImplementation("androidx.test:core-ktx:1.7.0")
    androidTestImplementation("androidx.test:rules:1.7.0")
    androidTestImplementation("androidx.test.ext:junit:1.3.0")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.7.0")
    androidTestImplementation("androidx.test.uiautomator:uiautomator:2.3.0")

    testImplementation("org.jetbrains.kotlinx:kotlinx-coroutines-test:1.10.2")
    testImplementation(kotlin("reflect"))
    androidTestImplementation(kotlin("reflect"))
}

kapt {
    correctErrorTypes = true
}

val rustProjectDir: File = layout.projectDirectory.asFile.parentFile
val rustWorkspaceDir: File = rustProjectDir.parentFile
val cargoTargetDirectory: File = rustWorkspaceDir.resolve("target")
val archTriplets = mapOf(
    "armeabi-v7a" to "armv7-linux-androideabi",
    "arm64-v8a" to "aarch64-linux-android",
    "x86" to "i686-linux-android",
    "x86_64" to "x86_64-linux-android",
)

listOf(Pair("debug", "debug"), Pair("release", "release-android")).forEach { (gradleProfile, cargoProfile) ->
    archTriplets.forEach { (arch, target) ->
        // Build with cargo
        tasks.register<Exec>("cargo-build-$gradleProfile-$arch") {
            description = "Building core for $gradleProfile-$arch"
            workingDir = rustProjectDir
            commandLine(
                "cargo",
                "ndk",
                "--target",
                arch,
                "--platform",
                "26",
                "build",
                "--profile",
                if (cargoProfile == "debug") "dev" else cargoProfile,
            )
        }
        // Copy build libs into this app's libs directory
        tasks.register<Copy>("rust-deploy-$gradleProfile-$arch") {
            dependsOn("cargo-build-$gradleProfile-$arch")
            description = "Copy rust libs for ($gradleProfile-$arch) to jniLibs"
            from("$cargoTargetDirectory/$target/$cargoProfile/libopenscq30_android.so")
            into("src/main/$gradleProfile/jniLibs/$arch")
        }

        // Hook up clean tasks
        tasks.register<Delete>("clean-$gradleProfile-$arch") {
            description = "Deleting built libs for $gradleProfile-$arch"
            delete(file("src/main/$gradleProfile/jniLibs/$arch/libopenscq30_android.so"))
        }
        tasks.clean.dependsOn("clean-$gradleProfile-$arch")
    }

    tasks.register<Exec>("generate-uniffi-bindings-$gradleProfile") {
        dependsOn("cargo-build-$gradleProfile-arm64-v8a")
        description = "Generate kotlin bindings using uniffi-bindgen"
        workingDir = rustWorkspaceDir
        // generate bindings
        commandLine(
            "cargo",
            "run",
            "--bin",
            "uniffi-bindgen",
            "--",
            "generate",
            "--library",
            "./target/aarch64-linux-android/$cargoProfile/libopenscq30_android.so",
            "--language",
            "kotlin",
            "--out-dir",
            "${layout.buildDirectory.get()}/generated/source/uniffi/java",
            "--config",
            "${layout.projectDirectory.asFile.parentFile.path}/uniffi.toml",
        )
    }
}

afterEvaluate {
    android.applicationVariants.forEach { variant ->
        val profile = if (variant.buildType.isDebuggable) "debug" else "release"
        // Hook up tasks to execute before building java
        variant.preBuildProvider.configure {
            archTriplets.forEach { (arch, _) ->
                dependsOn("rust-deploy-$profile-$arch")
            }
            dependsOn("generate-uniffi-bindings-$profile")
        }
    }
}
