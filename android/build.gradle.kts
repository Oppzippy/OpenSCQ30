// Top-level build file where you can add configuration options common to all sub-projects/modules.
plugins {
    id("com.android.application") version "8.13.2" apply false
    id("com.android.library") version "8.13.2" apply false

    val kotlinVersion = "2.2.21"
    id("org.jetbrains.kotlin.android") version kotlinVersion apply false
    id("org.jetbrains.kotlin.plugin.compose") version kotlinVersion apply false
    id("com.google.devtools.ksp") version "$kotlinVersion-2.0.4" apply false
    kotlin("plugin.serialization") version kotlinVersion

    id("com.google.dagger.hilt.android") version "2.57.2" apply false
}
