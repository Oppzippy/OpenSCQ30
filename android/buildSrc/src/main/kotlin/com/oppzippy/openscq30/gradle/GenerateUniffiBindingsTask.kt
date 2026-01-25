package com.oppzippy.openscq30.gradle

import java.io.File
import javax.inject.Inject
import org.gradle.api.DefaultTask
import org.gradle.api.file.DirectoryProperty
import org.gradle.api.file.ProjectLayout
import org.gradle.api.provider.Property
import org.gradle.api.tasks.Input
import org.gradle.api.tasks.OutputDirectory
import org.gradle.api.tasks.TaskAction
import org.gradle.process.ExecOperations

abstract class GenerateUniffiBindingsTask @Inject constructor(
    private val execOps: ExecOperations,
    private val layout: ProjectLayout,
) : DefaultTask() {
    @get:Input
    abstract val rustAbi: Property<String>

    @get:Input
    abstract val cargoProfile: Property<String>

    @get:Input
    abstract val rustProjectDirectory: Property<File>

    @get:Input
    abstract val rustWorkspaceDirectory: Property<File>

    @get:OutputDirectory
    abstract val outputDirectory: DirectoryProperty

    @TaskAction
    fun generate() {
        execOps.exec {
            workingDir = rustWorkspaceDirectory.get()
            commandLine(
                "cargo",
                "run",
                "--bin",
                "uniffi-bindgen",
                "--",
                "generate",
                "--library",
                "./target/${rustAbi.get()}/${cargoProfile.get()}/libopenscq30_android.so",
                "--language",
                "kotlin",
                "--out-dir",
                outputDirectory.get().asFile.absolutePath,
                "--config",
                "${rustProjectDirectory.get().absolutePath}/uniffi.toml",
            )
        }
    }
}
