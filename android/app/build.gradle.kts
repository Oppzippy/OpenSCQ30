import com.android.build.gradle.internal.tasks.factory.dependsOn
import java.util.Properties
import java.io.FileInputStream
import java.io.ByteArrayOutputStream

plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("org.jetbrains.kotlin.kapt")
    id("com.google.devtools.ksp")
    id("com.google.dagger.hilt.android")
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
    compileSdk = 33

    defaultConfig {
        applicationId = "com.oppzippy.openscq30"
        minSdk = 24
        targetSdk = 33
        versionCode = 4
        versionName = "1.2.0"
        buildConfigField("boolean", "IS_DEMO_MODE", "false")

        testInstrumentationRunner = "com.oppzippy.openscq30.HiltTestRunner"
        vectorDrawables {
            useSupportLibrary = true
        }
    }

    sourceSets {
        getByName("main") {
            jniLibs.srcDir("src/main/libs")
        }
    }

    buildTypes {
        named("debug") {
            applicationIdSuffix = ".debug"
            isDebuggable = true
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"), "proguard-rules.pro"
            )
        }
        create("debugDemo") {
            initWith(buildTypes["debug"])
            buildConfigField("boolean", "IS_DEMO_MODE", "true")
        }
        named("release") {
            isDebuggable = false
            isMinifyEnabled = true
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"), "proguard-rules.pro"
            )
            if (keystorePropertiesFile.exists()) {
                signingConfig = signingConfigs["release"]
            }
        }
        create("releaseDemo") {
            initWith(buildTypes["release"])
            buildConfigField("boolean", "IS_DEMO_MODE", "true")
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    kotlinOptions {
        jvmTarget = "17"
    }
    buildFeatures {
        compose = true
        buildConfig = true
    }
    composeOptions {
        kotlinCompilerExtensionVersion = "1.4.7"
    }
    ksp {
        arg("room.schemaLocation", "$projectDir/schemas")
    }
    ndkVersion = "25.2.9519653"
    packaging {
        resources {
            excludes += "/META-INF/*"
        }
    }
}

dependencies {
    implementation("androidx.lifecycle:lifecycle-runtime-ktx:2.6.1")

    // Compose
    val composeBomVersion = "2023.06.00"
    implementation(platform("androidx.compose:compose-bom:$composeBomVersion"))
    androidTestImplementation(platform("androidx.compose:compose-bom:$composeBomVersion"))
    implementation("androidx.compose.ui:ui")
    // Material Design 3
    implementation("androidx.compose.material3:material3")
    // Material Design 2
    implementation("androidx.compose.material:material")
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
    implementation("androidx.activity:activity-compose:1.7.2")
    // Optional - Integration with ViewModels
    implementation("androidx.lifecycle:lifecycle-viewmodel-compose:2.6.1")
    // Compose navigation
    implementation("androidx.navigation:navigation-compose:2.5.3")

    val accompanistVersion = "0.30.1"
    implementation("com.google.accompanist:accompanist-permissions:$accompanistVersion")

    val roomVersion = "2.5.1"

    implementation("androidx.room:room-runtime:$roomVersion")
    implementation("androidx.room:room-ktx:$roomVersion")
    annotationProcessor("androidx.room:room-compiler:$roomVersion")
    ksp("androidx.room:room-compiler:$roomVersion")
    testImplementation("androidx.room:room-testing:$roomVersion")

    val hiltVersion = "2.46.1"
    implementation("com.google.dagger:hilt-android:$hiltVersion")
    kapt("com.google.dagger:hilt-android-compiler:$hiltVersion")
    implementation("androidx.hilt:hilt-navigation-compose:1.0.0")

    // For instrumentation tests
    androidTestImplementation("com.google.dagger:hilt-android-testing:$hiltVersion")
    kaptAndroidTest("com.google.dagger:hilt-compiler:$hiltVersion")

    // For local unit tests
    testImplementation("com.google.dagger:hilt-android-testing:$hiltVersion")
    kaptTest("com.google.dagger:hilt-compiler:$hiltVersion")

    testImplementation("junit:junit:4.13.2")

    val mockkVersion = "1.13.3"
    testImplementation("io.mockk:mockk:$mockkVersion")
    androidTestImplementation("io.mockk:mockk-android:$mockkVersion")
    androidTestImplementation("androidx.test:runner:1.5.2")
    androidTestImplementation("androidx.test:core-ktx:1.5.0")
    androidTestImplementation("androidx.test:rules:1.5.0")
    androidTestImplementation("androidx.test.ext:junit:1.1.5")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.5.1")
}

kapt {
    correctErrorTypes = true
}

val rustBasePath = ".."
val archTriplets = mapOf(
    "armeabi-v7a" to "armv7-linux-androideabi",
    "arm64-v8a" to "aarch64-linux-android",
    "x86" to "i686-linux-android",
    "x86_64" to "x86_64-linux-android",
)

archTriplets.forEach { (arch, target) ->
    // execute cargo metadata and get path to target directory
    tasks.create("cargo-output-dir-${arch}") {
        description = "Get cargo metadata"
        val output = ByteArrayOutputStream()
        exec {
            commandLine("cargo", "metadata", "--format-version", "1")
            workingDir = File(rustBasePath)
            standardOutput = output
        }
        val outputAsString = output.toString()
        val json = groovy.json.JsonSlurper().parseText(outputAsString) as Map<*, *>
        val targetDirectory = json["target_directory"] as String

        logger.info("cargo target directory: $targetDirectory")
        project.extensions.extraProperties.set("cargo_target_directory", targetDirectory)
    }
    // Build with cargo
    tasks.create<Exec>("cargo-build-${arch}") {
        description = "Building core for $arch"
        workingDir = File(rustBasePath)
        commandLine(
            "cargo",
            "ndk",
            "--target",
            arch,
            "--platform",
            "24",
            "build",
            "--profile",
            "release-debuginfo"
        )
    }
    // Sync shared native dependencies
    tasks.create<Sync>("sync-rust-deps-${arch}") {
        dependsOn("cargo-build-${arch}")
        from("${rustBasePath}/src/libs/${arch}") {
            include("*.so")
        }
        into("src/main/libs/${arch}")
    }
    // Copy build libs into this app's libs directory
    tasks.create<Copy>("rust-deploy-${arch}") {
        dependsOn("sync-rust-deps-${arch}")
        description = "Copy rust libs for ($arch) to jniLibs"
        from("${project.extensions.extraProperties.get("cargo_target_directory")}/${target}/release-debuginfo") {
            include("*.so")
        }
        into("src/main/libs/${arch}")
    }

    // Hook up tasks to execute before building java
    tasks.withType<JavaCompile> {
        dependsOn("rust-deploy-${arch}")
    }
    tasks.preBuild.dependsOn("rust-deploy-${arch}")

    // Hook up clean tasks
    tasks.create<Delete>("clean-${arch}") {
        dependsOn("cargo-output-dir-${arch}")
        description = "Deleting built libs for $arch"
        delete(fileTree("${project.extensions.extraProperties.get("cargo_target_directory")}/${target}/release-debuginfo") {
            include("*.so")
        })
    }
    tasks.clean.dependsOn("clean-${arch}")
}
