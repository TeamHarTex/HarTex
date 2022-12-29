/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2023 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

package com.github.teamhartex.hartex.buildsystem.processes.common

import com.github.ajalt.mordant.rendering.TextColors.*
import com.github.ajalt.mordant.rendering.TextStyles.*
import com.github.ajalt.mordant.terminal.Terminal
import com.github.teamhartex.hartex.buildsystem.Project
import com.github.teamhartex.hartex.buildsystem.ProjectBuildTool
import com.github.teamhartex.hartex.buildsystem.ProjectType
import com.github.teamhartex.hartex.buildsystem.processes.BuildsystemProcess
import java.io.File
import java.nio.file.Path

class CommonCleanProcess {
  companion object : BuildsystemProcess {
    override fun new(projectToBuild: Project, args: List<String>, terminal: Terminal): Process? {
      val processBuilder = ProcessBuilder()

      when (projectToBuild.projectType to projectToBuild.buildTool) {
        ProjectType.RUST to ProjectBuildTool.CARGO -> {
          terminal.println("${bold(green("Running"))} cargo clean")
          processBuilder.command("cargo", "clean")
        }
        ProjectType.TYPESCRIPT to ProjectBuildTool.YARN -> {
          try {
            terminal.println("${bold(green("Deleting"))} built artifacts")

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
