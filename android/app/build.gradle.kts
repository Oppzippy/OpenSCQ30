@file:Suppress("UnstableApiUsage")

import com.android.build.gradle.internal.tasks.factory.dependsOn
import com.google.protobuf.gradle.id
import java.io.ByteArrayOutputStream
import java.io.FileInputStream
import java.util.Properties

plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("org.jetbrains.kotlin.plugin.compose")
    id("org.jetbrains.kotlin.kapt")
    id("com.google.devtools.ksp")
    id("com.google.dagger.hilt.android")
    id("org.jlleitschuh.gradle.ktlint")
    id("com.google.protobuf")
    kotlin("plugin.serialization")
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
    compileSdk = 34
    buildToolsVersion = "34.0.0"

    defaultConfig {
        applicationId = "com.oppzippy.openscq30"
        minSdk = 26
        targetSdk = 34
        versionCode = 34
        versionName = "1.18.1"

        testInstrumentationRunner = "com.oppzippy.openscq30.HiltTestRunner"
        vectorDrawables {
            useSupportLibrary = true
        }
    }

    sourceSets {
        getByName("main") {
            java.srcDir("${layout.buildDirectory.get()}/generated/source/uniffi/java")
            jniLibs.srcDir("src/main/libs")
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
    flavorDimensions += "mode"
    productFlavors {
        create("bluetooth") {
            isDefault = true
            dimension = "mode"
            buildConfigField("boolean", "IS_DEMO_MODE", "false")
        }
        create("demo") {
            isDefault = false
            dimension = "mode"
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
    ksp {
        arg("room.schemaLocation", "$projectDir/schemas")
    }
    ndkVersion = "27.2.12479018"
    packaging {
        resources {
            excludes += "/META-INF/*"
        }
    }
}

dependencies {
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.8.0")

    val lifecycleVersion = "2.8.7"
    implementation("androidx.lifecycle:lifecycle-runtime-ktx:$lifecycleVersion")
    implementation("androidx.lifecycle:lifecycle-service:$lifecycleVersion")

    // Compose
    val composeBomVersion = "2024.12.01"
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
    implementation("androidx.activity:activity-compose:1.9.3")
    // Optional - Integration with ViewModels
    implementation("androidx.lifecycle:lifecycle-viewmodel-compose:$lifecycleVersion")
    // Compose navigation
    implementation("androidx.navigation:navigation-compose:2.8.5")

    val accompanistVersion = "0.37.0"
    implementation("com.google.accompanist:accompanist-permissions:$accompanistVersion")

    val roomVersion = "2.6.1"
    implementation("androidx.room:room-runtime:$roomVersion")
    implementation("androidx.room:room-ktx:$roomVersion")
    annotationProcessor("androidx.room:room-compiler:$roomVersion")
    ksp("androidx.room:room-compiler:$roomVersion")
    androidTestImplementation("androidx.room:room-testing:$roomVersion")

    val protobufVersion = "4.29.2"
    implementation("com.google.protobuf:protobuf-java:$protobufVersion")
    implementation("com.google.protobuf:protobuf-kotlin:$protobufVersion")
    protobuf(files("../../lib_protobuf/protobuf/"))

    implementation("net.java.dev.jna:jna:5.16.0@aar")

    val hiltVersion = "2.54"
    implementation("com.google.dagger:hilt-android:$hiltVersion")
    kapt("com.google.dagger:hilt-android-compiler:$hiltVersion")
    implementation("androidx.hilt:hilt-navigation-compose:1.2.0")

    // For instrumentation tests
    androidTestImplementation("com.google.dagger:hilt-android-testing:$hiltVersion")
    kaptAndroidTest("com.google.dagger:hilt-compiler:$hiltVersion")

    // For local unit tests
    testImplementation("com.google.dagger:hilt-android-testing:$hiltVersion")
    kaptTest("com.google.dagger:hilt-compiler:$hiltVersion")

    testImplementation("junit:junit:4.13.2")

    val mockkVersion = "1.13.14"
    testImplementation("io.mockk:mockk:$mockkVersion")
    androidTestImplementation("io.mockk:mockk-android:$mockkVersion")
    androidTestImplementation("androidx.test:runner:1.6.2")
    androidTestImplementation("androidx.test:core-ktx:1.6.1")
    androidTestImplementation("androidx.test:rules:1.6.1")
    androidTestImplementation("androidx.test.ext:junit:1.2.1")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.6.1")
    androidTestImplementation("androidx.test.uiautomator:uiautomator:2.3.0")

    testImplementation("org.jetbrains.kotlinx:kotlinx-coroutines-test:1.10.1")
    testImplementation(kotlin("reflect"))
    androidTestImplementation(kotlin("reflect"))
}

kapt {
    correctErrorTypes = true
}

protobuf {
    generateProtoTasks {
        all().forEach { task ->
            task.builtins {
                id("java")
                id("kotlin")
            }
        }
    }
    protoc {
        artifact = "com.google.protobuf:protoc:3.25.1"
    }
}

val rustProjectDir: File = layout.projectDirectory.asFile.parentFile
val rustWorkspaceDir: File = rustProjectDir.parentFile
val archTriplets = mapOf(
    "armeabi-v7a" to "armv7-linux-androideabi",
    "arm64-v8a" to "aarch64-linux-android",
    "x86" to "i686-linux-android",
    "x86_64" to "x86_64-linux-android",
)

tasks.create<Exec>("generate-uniffi-bindings") {
    dependsOn("cargo-build-arm64-v8a")
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
        "./target/aarch64-linux-android/release-debuginfo/libopenscq30_android.so",
        "--language",
        "kotlin",
        "--out-dir",
        "${layout.buildDirectory.get()}/generated/source/uniffi/java",
        "--config",
        "${layout.projectDirectory.asFile.parentFile.path}/uniffi.toml",
    )
}
tasks.withType<JavaCompile> {
    dependsOn("generate-uniffi-bindings")
}

interface InjectedExecOps {
    @get:Inject
    val execOps: ExecOperations
}

archTriplets.forEach { (arch, target) ->
    // execute cargo metadata and get path to target directory
    tasks.create("cargo-output-dir-$arch") {
        val injectedExecOps = project.objects.newInstance<InjectedExecOps>()
        description = "Get cargo metadata"
        val output = ByteArrayOutputStream()
        injectedExecOps.execOps.exec {
            commandLine("cargo", "metadata", "--format-version", "1")
            workingDir = rustProjectDir
            standardOutput = output
        }
        val outputAsString = output.toString()
        val json = groovy.json.JsonSlurper().parseText(outputAsString) as Map<*, *>
        val targetDirectory = json["target_directory"] as String

        logger.info("cargo target directory: $targetDirectory")
        project.extensions.extraProperties.set("cargo_target_directory", targetDirectory)
    }
    // Build with cargo
    tasks.create<Exec>("cargo-build-$arch") {
        description = "Building core for $arch"
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
            "release-debuginfo",
        )
    }
    // Sync shared native dependencies
    tasks.create<Sync>("sync-rust-deps-$arch") {
        dependsOn("cargo-build-$arch")
        from("${rustProjectDir.absolutePath}/src/libs/$arch") {
            include("*.so")
        }
        into("src/main/libs/$arch")
    }
    // Copy build libs into this app's libs directory
    tasks.create<Copy>("rust-deploy-$arch") {
        dependsOn("sync-rust-deps-$arch")
        description = "Copy rust libs for ($arch) to jniLibs"
        val cargoTargetDirectory = project.extensions.extraProperties.get("cargo_target_directory")
        from(
            "$cargoTargetDirectory/$target/release-debuginfo",
        ) {
            include("*.so")
        }
        into("src/main/libs/$arch")
    }

    // Hook up tasks to execute before building java
    tasks.withType<JavaCompile> {
        dependsOn("rust-deploy-$arch")
    }
    tasks.preBuild.dependsOn("rust-deploy-$arch")

    // Hook up clean tasks
    tasks.create<Delete>("clean-$arch") {
        dependsOn("cargo-output-dir-$arch")
        description = "Deleting built libs for $arch"
        delete(
            fileTree(
                "${project.extensions.extraProperties.get("cargo_target_directory")}/$target/release-debuginfo",
            ) {
                include("*.so")
            },
        )
    }
    tasks.clean.dependsOn("clean-$arch")
}
configure<org.jlleitschuh.gradle.ktlint.KtlintExtension> {
    version.set("1.3.1")
}
