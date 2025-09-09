// Top-level build file where you can add configuration options common to all sub-projects/modules.
plugins {
    id("com.android.application") version "8.13.0" apply false
    id("com.android.library") version "8.13.0" apply false

    val kotlinVersion = "2.2.10"
    id("org.jetbrains.kotlin.android") version kotlinVersion apply false
    id("org.jetbrains.kotlin.plugin.compose") version kotlinVersion apply false
    id("com.google.devtools.ksp") version "$kotlinVersion-2.0.2" apply false
    kotlin("plugin.serialization") version kotlinVersion

    id("com.google.dagger.hilt.android") version "2.57.1" apply false
}
