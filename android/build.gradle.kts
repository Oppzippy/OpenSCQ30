// Top-level build file where you can add configuration options common to all sub-projects/modules.
plugins {
    id("com.android.application") version "9.0.0" apply false
    id("com.android.library") version "9.0.0" apply false

    val kotlinVersion = "2.3.0"
    id("org.jetbrains.kotlin.plugin.compose") version kotlinVersion apply false
    id("com.google.devtools.ksp") version "2.3.4" apply false
    kotlin("plugin.serialization") version kotlinVersion

    id("com.google.dagger.hilt.android") version "2.57.2" apply false
}
