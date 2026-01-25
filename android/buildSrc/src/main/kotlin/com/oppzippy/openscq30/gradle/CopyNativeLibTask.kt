package com.oppzippy.openscq30.gradle

import java.io.File
import javax.inject.Inject
import org.gradle.api.DefaultTask
import org.gradle.api.file.DirectoryProperty
import org.gradle.api.file.FileSystemOperations
import org.gradle.api.file.ProjectLayout
import org.gradle.api.provider.Property
import org.gradle.api.tasks.Input
import org.gradle.api.tasks.OutputDirectory
import org.gradle.api.tasks.TaskAction

abstract class CopyNativeLibTask @Inject constructor(
    private val fsOps: FileSystemOperations,
    private val layout: ProjectLayout,
) : DefaultTask() {
    @get:Input
    abstract val cargoProfile: Property<String>

    @get:Input
    abstract val gradleBuildProfile: Property<String>

    @get:Input
    abstract val androidAbi: Property<String>

    @get:Input
    abstract val inputFile: Property<File>

    @get:OutputDirectory
    abstract val outputDirectory: DirectoryProperty

    @TaskAction
    fun copy() {
        val gradleBuildProfile = this.gradleBuildProfile.get()
        val cargoProfile = this.cargoProfile.get()
        fsOps.copy {
            from(inputFile.get())
            into(outputDirectory.get().asFile.resolve(androidAbi.get()))
        }
    }
}
