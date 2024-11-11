import org.jlleitschuh.gradle.ktlint.KtlintExtension

// Top-level build file where you can add configuration options common to all sub-projects/modules.
plugins {
    id("com.android.application") version "8.7.2" apply false
    id("com.android.library") version "8.7.2" apply false

    id("org.jetbrains.kotlin.android") version "2.0.21" apply false
    id("org.jetbrains.kotlin.plugin.compose") version "2.0.21" apply false
    id("com.google.devtools.ksp") version "2.0.21-1.0.26" apply false
    kotlin("plugin.serialization") version "2.0.10"

    id("com.google.dagger.hilt.android") version "2.52" apply false
    id("org.jlleitschuh.gradle.ktlint") version "12.1.1"
    id("com.google.protobuf") version "0.9.4" apply false
}

configure<KtlintExtension> {
    android.set(true)
}
