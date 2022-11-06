package com.github.teamhartex.hartex.buildsystem.processes.common

import com.github.teamhartex.hartex.buildsystem.Project
import com.github.teamhartex.hartex.buildsystem.ProjectBuildTool
import com.github.teamhartex.hartex.buildsystem.ProjectType
import java.io.File
import java.nio.file.Path

class CommonCleanProcess {
  companion object {
    fun new(projectToBuild: Project, args: List<String>): Process? {
      val processBuilder = ProcessBuilder()

      when (projectToBuild.projectType to projectToBuild.buildTool) {
        ProjectType.RUST to ProjectBuildTool.CARGO -> processBuilder.command("cargo", "clean")
        ProjectType.TYPESCRIPT to ProjectBuildTool.YARN -> {
          try {
            if (!File(Path.of(System.getProperty("user.dir"), args[2], "dist").toUri()).deleteRecursively())
              throw RuntimeException("failed to delete dist directory")

            return null
          } catch (exception: Throwable) {
            throw exception
          }
        }
      }

      return processBuilder.directory(File(System.getProperty("user.dir") + """/${args[2]}"""))
        .redirectError(ProcessBuilder.Redirect.INHERIT)
        .start()
    }
  }
}
