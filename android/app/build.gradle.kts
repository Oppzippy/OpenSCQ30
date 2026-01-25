import com.oppzippy.openscq30.gradle.CopyNativeLibTask
import com.oppzippy.openscq30.gradle.GenerateUniffiBindingsTask
import dagger.hilt.android.plugin.util.capitalize
import java.io.FileInputStream
import java.util.Properties
import org.jetbrains.kotlin.gradle.dsl.KotlinVersion

plugins {
    alias(libs.plugins.android.application)
    alias(libs.plugins.kotlin.plugin.compose)
    alias(libs.plugins.google.devtools.ksp)
    alias(libs.plugins.dagger.hilt.android)
    alias(libs.plugins.kotlin.serialization)
    id("kotlin-parcelize")
}

val keystorePropertiesFile: File = rootProject.file("keystore.properties")
val keystoreProperties = Properties()
if (keystorePropertiesFile.exists()) {
    keystoreProperties.load(FileInputStream(keystorePropertiesFile))
}

data class ABI(val android: String, val rust: String)

val abis = listOf(
    ABI(android = "armeabi-v7a", rust = "armv7-linux-androideabi"),
    ABI(android = "arm64-v8a", rust = "aarch64-linux-android"),
    ABI(android = "x86", rust = "i686-linux-android"),
    ABI(android = "x86_64", rust = "x86_64-linux-android"),
)
val gradleToCargoProfiles = mapOf(
    "debug" to "debug",
    "release" to "release-android",
)

kotlin {
    compilerOptions {
        languageVersion = KotlinVersion.KOTLIN_2_3
    }
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
        versionCode = 1023
        versionName = "2.8.2"

        testInstrumentationRunner = "com.oppzippy.openscq30.HiltTestRunner"
        vectorDrawables {
            useSupportLibrary = true
        }
    }

    flavorDimensions += "abi"
    productFlavors {
        abis.forEach { abi ->
            create(abi.android) {
                dimension = "abi"
                ndk {
                    abiFilters.clear()
                    abiFilters.add(abi.android)
                }
            }
        }
    }

    sourceSets {
        getByName("main") {}
        getByName("androidTest") {
            assets.directories += "$projectDir/schemas"
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
            isShrinkResources = true
            isCrunchPngs = false
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
    buildFeatures {
        compose = true
        buildConfig = true
    }
    ndkVersion = "29.0.14206865"
    packaging {
        resources {
            excludes += "/META-INF/*"
        }
    }
    androidResources {
        @Suppress("UnstableApiUsage")
        generateLocaleConfig = true
    }
}

ksp {
    arg("room.schemaLocation", "$projectDir/schemas")
}

dependencies {
    implementation(libs.kotlinx.serialization.json)

    implementation(libs.androidx.lifecycle.runtime.ktx)
    implementation(libs.androidx.lifecycle.service)

    // Compose
    implementation(platform(libs.androidx.compose.bom))
    implementation(libs.androidx.appcompat)
    androidTestImplementation(platform(libs.androidx.compose.bom))
    implementation(libs.androidx.compose.ui)
    // Material Design 3
    implementation(libs.androidx.compose.material3)
    // Android Studio Preview support
    implementation(libs.androidx.compose.ui.tooling.preview)
    debugImplementation(libs.androidx.compose.ui.tooling)
    // UI Tests
    androidTestImplementation(libs.androidx.compose.ui.test.junit4)
    debugImplementation(libs.androidx.compose.ui.test.manifest)
    // Optional - Integration with activities
    implementation(libs.androidx.activity.compose)
    // Optional - Integration with ViewModels
    implementation(libs.androidx.lifecycle.viewmodel.compose)
    // Compose navigation
    implementation(libs.androidx.navigation.compose)

    implementation(libs.androidx.work)

    implementation(libs.androidx.glance.appwidget)
    implementation(libs.androidx.glance.material3)

    implementation(libs.accompanist.permissions)

    implementation(libs.androidx.room.runtime)
    implementation(libs.androidx.room.ktx)
    annotationProcessor(libs.androidx.room.compiler)
    ksp(libs.androidx.room.compiler)
    androidTestImplementation(libs.androidx.room.testing)

    implementation(libs.jna) {
        artifact {
            type = "aar"
        }
    }

    implementation(libs.dagger.hilt.android)
    ksp(libs.dagger.hilt.android.compiler)
    implementation(libs.androidx.hilt.navigationCompose)

    // For instrumentation tests
    androidTestImplementation(libs.dagger.hilt.android.testing)
    kspTest(libs.dagger.hilt.android.compiler)

    // For local unit tests
    testImplementation(libs.dagger.hilt.android.testing)
    kspTest(libs.dagger.hilt.android.compiler)

    testImplementation(libs.junit)

    testImplementation(libs.mockk)
    androidTestImplementation(libs.mockk.android)
    androidTestImplementation(libs.androidx.runner)
    androidTestImplementation(libs.androidx.core.ktx)
    androidTestImplementation(libs.androidx.rules)
    androidTestImplementation(libs.androidx.junit)
    androidTestImplementation(libs.androidx.espresso.core)
    androidTestImplementation(libs.androidx.uiautomator)

    testImplementation(libs.kotlinx.coroutines.test)
    testImplementation(kotlin("reflect"))
    androidTestImplementation(kotlin("reflect"))
}

val rustProjectDir: File = layout.projectDirectory.asFile.parentFile
val rustWorkspaceDir: File = rustProjectDir.parentFile
val cargoTargetDirectory: File = rustWorkspaceDir.resolve("target")

gradleToCargoProfiles.forEach { (gradleBuildProfile, cargoProfile) ->
    abis.forEach { abi ->
        // Build with cargo
        tasks.register<Exec>("cargo-build-$gradleBuildProfile-${abi.android}") {
            description = "Building core for $gradleBuildProfile-${abi.android}"
            workingDir = rustProjectDir
            commandLine(
                "cargo",
                "ndk",
                "--target",
                abi.rust,
                "--platform",
                "26",
                "build",
                "--profile",
                if (cargoProfile == "debug") "dev" else cargoProfile,
            )
        }

        val copyNativeLibTask = tasks.register<CopyNativeLibTask>("rust-deploy-$gradleBuildProfile-${abi.android}") {
            dependsOn("cargo-build-$gradleBuildProfile-${abi.android}")
            description = "Copy rust libs for ($gradleBuildProfile-${abi.android}) to jniLibs"
            this.gradleBuildProfile = gradleBuildProfile
            this.cargoProfile = cargoProfile
            this.inputFile = File("$cargoTargetDirectory/${abi.rust}/$cargoProfile/libopenscq30_android.so")
            this.androidAbi = abi.android
            this.outputDirectory =
                layout.buildDirectory.get().asFile.resolve(
                    "generated/native/$gradleBuildProfile-${abi.android}/jniLibs",
                )
        }

        val generateTask =
            tasks.register<GenerateUniffiBindingsTask>("generate-uniffi-bindings-$gradleBuildProfile-${abi.android}") {
                dependsOn("cargo-build-$gradleBuildProfile-${abi.android}")
                description = "Generate kotlin bindings using uniffi-bindgen"
                this.rustAbi = abi.rust
                this.cargoProfile = cargoProfile
                this.rustWorkspaceDirectory = rustWorkspaceDir
                this.rustProjectDirectory = rustProjectDir
                this.outputDirectory = layout.buildDirectory.get().asFile
                    .resolve("generated/source/uniffi/${abi.android}${gradleBuildProfile.capitalize()}/java")
            }

        androidComponents {
            onVariants(
                selector()
                    .withBuildType(gradleBuildProfile)
                    .withFlavor("abi" to abi.android),
            ) { variant ->
                // TODO switch to variant.sources.kotlin when this issue is fixed:
                // https://github.com/google/ksp/issues/2494
                variant.sources.java!!.addGeneratedSourceDirectory(
                    generateTask,
                    GenerateUniffiBindingsTask::outputDirectory,
                )
                variant.sources.jniLibs!!.addGeneratedSourceDirectory(
                    copyNativeLibTask,
                    CopyNativeLibTask::outputDirectory,
                )
            }
        }
    }
}
