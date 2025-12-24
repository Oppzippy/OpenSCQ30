import com.android.build.api.dsl.ApplicationBuildType
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

val abis = listOf(
    ABI(android = "armeabi-v7a", rust = "armv7-linux-androideabi"),
    ABI(android = "arm64-v8a", rust = "aarch64-linux-android"),
    ABI(android = "x86", rust = "i686-linux-android"),
    ABI(android = "x86_64", rust = "x86_64-linux-android"),
)
val buildProfiles = listOf(
    BuildProfile(gradle = "debug", cargo = "debug", isDebug = true),
    BuildProfile(gradle = "release", cargo = "release-android", isDebug = false),
)

data class ABI(val android: String, val rust: String)

data class BuildProfile(val gradle: String, val cargo: String, val isDebug: Boolean)

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
        versionCode = 1008
        versionName = "2.0.1"

        testInstrumentationRunner = "com.oppzippy.openscq30.HiltTestRunner"
        vectorDrawables {
            useSupportLibrary = true
        }
    }

    sourceSets {
        buildProfiles.forEach { buildProfile ->
            getByName(buildProfile.gradle) {
                jniLibs.srcDir("src/main/${buildProfile.gradle}/jniLibs")
            }
            abis.forEach { abi ->
                create("${buildProfile.gradle}-${abi.android}") {
                    jniLibs.srcDir("src/main/${buildProfile.gradle}-${abi.android}/jniLibs")
                }
            }
        }
        getByName("main") {
            java.srcDir("${layout.buildDirectory.get()}/generated/source/uniffi/java")
        }
        getByName("androidTest") {
            assets.srcDir("$projectDir/schemas")
        }
    }

    buildTypes {
        fun filterAbi(buildType: ApplicationBuildType, abi: String?) {
            buildType.ndk {
                abiFilters.clear()
                if (abi != null) {
                    abiFilters.add(abi)
                } else {
                    abiFilters.addAll(listOf("armeabi-v7a", "arm64-v8a", "x86", "x86_64"))
                }
            }
        }

        named("debug") {
            applicationIdSuffix = ".debug"
            isDebuggable = true
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro",
            )
            filterAbi(this, null)
        }
        abis.forEach { abi ->
            create("debug-${abi.android}") {
                initWith(buildTypes["debug"])
                filterAbi(this, abi.android)
            }
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
            filterAbi(this, null)
        }
        abis.forEach { abi ->
            create("release-${abi.android}") {
                initWith(buildTypes["release"])
                filterAbi(this, abi.android)
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
    val composeBomVersion = "2025.12.01"
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
    implementation("androidx.activity:activity-compose:1.12.2")
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

    val mockkVersion = "1.14.7"
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

buildProfiles.forEach { buildProfile ->
    abis.forEach { abi ->
        // Build with cargo
        tasks.register<Exec>("cargo-build-${buildProfile.gradle}-${abi.android}") {
            description = "Building core for ${buildProfile.gradle}-${abi.android}"
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
                if (buildProfile.cargo == "debug") "dev" else buildProfile.cargo,
            )
        }
        // Copy build libs into this app's libs directory
        tasks.register<Copy>("rust-deploy-${buildProfile.gradle}-${abi.android}") {
            dependsOn("cargo-build-${buildProfile.gradle}-${abi.android}")
            description = "Copy rust libs for (${buildProfile.gradle}-${abi.android}) to jniLibs"
            from("$cargoTargetDirectory/${abi.rust}/${buildProfile.cargo}/libopenscq30_android.so")
            into("src/main/${buildProfile.gradle}-${abi.android}/jniLibs/${abi.android}")
        }

        // Hook up clean tasks
        tasks.register<Delete>("clean-${buildProfile.gradle}-${abi.android}") {
            description = "Deleting built libs for ${buildProfile.gradle}-${abi.android}"
            delete(
                file("src/main/${buildProfile.gradle}-${abi.android}/jniLibs/${abi.android}/libopenscq30_android.so"),
            )
        }
        tasks.clean.dependsOn("clean-${buildProfile.gradle}-${abi.android}")

        tasks.register<Exec>("generate-uniffi-bindings-${buildProfile.gradle}-${abi.android}") {
            dependsOn("cargo-build-${buildProfile.gradle}-${abi.android}")
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
                "./target/${abi.rust}/${buildProfile.cargo}/libopenscq30_android.so",
                "--language",
                "kotlin",
                "--out-dir",
                "${layout.buildDirectory.get()}/generated/source/uniffi/java",
                "--config",
                "${layout.projectDirectory.asFile.parentFile.path}/uniffi.toml",
            )
        }
    }

    abis.forEach { abi ->
        tasks.register<Copy>("rust-deploy-${buildProfile.gradle}-universal-${abi.android}") {
            dependsOn("cargo-build-${buildProfile.gradle}-${abi.android}")
            description = "Copy rust libs for (${buildProfile.gradle}-${abi.android}) to jniLibs"
            from("$cargoTargetDirectory/${abi.rust}/${buildProfile.cargo}/libopenscq30_android.so")
            into("src/main/${buildProfile.gradle}/jniLibs/${abi.android}")
        }
        tasks.register<Delete>("clean-${buildProfile.gradle}-universal-${abi.android}") {
            description = "Deleting built libs for ${buildProfile.gradle}-${abi.android}"
            delete(file("src/main/${buildProfile.gradle}/jniLibs/${abi.android}/libopenscq30_android.so"))
        }
        tasks.clean.dependsOn("clean-${buildProfile.gradle}-universal-${abi.android}")
    }
}

afterEvaluate {
    android.applicationVariants.forEach { variant ->
        variant.preBuildProvider.configure {
            val variantParts = variant.name.split('-', limit = 2)
            val profile = buildProfiles.find { variantParts[0] == it.gradle }!!
            if (variantParts.size == 2) {
                // specific abi
                val abi = abis.find { variantParts[1] == it.android }!!
                dependsOn("rust-deploy-${profile.gradle}-${abi.android}")
                dependsOn("generate-uniffi-bindings-${profile.gradle}-${abi.android}")
            } else {
                // all abis
                abis.forEach { abi ->
                    dependsOn("rust-deploy-${profile.gradle}-universal-${abi.android}")
                }
                dependsOn("generate-uniffi-bindings-${profile.gradle}-${abis[0].android}")
            }
        }
    }
}
