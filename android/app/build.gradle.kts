import com.android.build.api.dsl.ApplicationBuildType
import com.android.build.gradle.internal.tasks.factory.dependsOn
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
        versionCode = 1015
        versionName = "2.5.0"

        testInstrumentationRunner = "com.oppzippy.openscq30.HiltTestRunner"
        vectorDrawables {
            useSupportLibrary = true
        }
    }

    sourceSets {
        buildProfiles.forEach { buildProfile ->
            getByName(buildProfile.gradle) {
                jniLibs.directories += "src/main/${buildProfile.gradle}/jniLibs"
            }
            abis.forEach { abi ->
                create("${buildProfile.gradle}-${abi.android}") {
                    jniLibs.directories += "src/main/${buildProfile.gradle}-${abi.android}/jniLibs"
                }
            }
        }
        getByName("main") {
            kotlin.directories += "${layout.buildDirectory.get()}/generated/source/uniffi/java"
        }
        getByName("androidTest") {
            assets.directories += "$projectDir/schemas"
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
            isShrinkResources = true
            isCrunchPngs = false
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
    // Optional - Included automatically by material, only add when you need
    // the icons but not the material library (e.g. when using Material3 or a
    // custom design system based on Foundation)
    implementation(libs.androidx.compose.material.icons.core)
    // Optional - Add full set of material icons
    implementation(libs.androidx.compose.material.icons.extended)
    // Optional - Integration with activities
    implementation(libs.androidx.activity.compose)
    // Optional - Integration with ViewModels
    implementation(libs.androidx.lifecycle.viewmodel.compose)
    // Compose navigation
    implementation(libs.androidx.navigation.compose)

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
