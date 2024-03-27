import org.jlleitschuh.gradle.ktlint.KtlintExtension

// Top-level build file where you can add configuration options common to all sub-projects/modules.
plugins {
    id("com.android.application") version "8.3.1" apply false
    id("com.android.library") version "8.3.1" apply false
    id("org.jetbrains.kotlin.android") version "1.9.23" apply false
    id("com.google.dagger.hilt.android") version "2.51" apply false
    id("com.google.devtools.ksp") version "1.9.23-1.0.19" apply false
    id("org.jlleitschuh.gradle.ktlint") version "12.1.0"
    id("com.google.protobuf") version "0.9.4" apply false
}

configure<KtlintExtension> {
    android.set(true)
}
