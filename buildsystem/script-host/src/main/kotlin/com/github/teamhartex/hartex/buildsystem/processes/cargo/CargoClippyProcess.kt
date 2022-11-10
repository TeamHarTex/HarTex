package com.github.teamhartex.hartex.buildsystem.processes.cargo

import com.github.teamhartex.hartex.buildsystem.Project
import com.github.teamhartex.hartex.buildsystem.ProjectBuildTool
import com.github.teamhartex.hartex.buildsystem.ProjectType
import java.io.File

class CargoClippyProcess {
  companion object {
    fun new(projectToBuild: Project, args: List<String>): Process {
      val processBuilder = ProcessBuilder()

      when (projectToBuild.projectType to projectToBuild.buildTool) {
        ProjectType.RUST to ProjectBuildTool.CARGO -> {
          processBuilder.command("cargo", "clippy")
        }
        else -> throw IllegalStateException("running cargo clippy is not supported in non-Rust projects")
      }

      return processBuilder.directory(File(System.getProperty("user.dir") + """/${args[2]}"""))
        .redirectError(ProcessBuilder.Redirect.INHERIT)
        .start()
    }
  }
}
