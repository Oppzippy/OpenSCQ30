import org.jlleitschuh.gradle.ktlint.KtlintExtension

// Top-level build file where you can add configuration options common to all sub-projects/modules.
plugins {
    id("com.android.application") version "8.7.3" apply false
    id("com.android.library") version "8.7.3" apply false

    val kotlinVersion = "2.1.0"
    id("org.jetbrains.kotlin.android") version kotlinVersion apply false
    id("org.jetbrains.kotlin.plugin.compose") version kotlinVersion apply false
    id("com.google.devtools.ksp") version "$kotlinVersion-1.0.29" apply false
    kotlin("plugin.serialization") version kotlinVersion

    id("com.google.dagger.hilt.android") version "2.54" apply false
    id("org.jlleitschuh.gradle.ktlint") version "12.1.1"
    id("com.google.protobuf") version "0.9.4" apply false
}

configure<KtlintExtension> {
    android.set(true)
}
