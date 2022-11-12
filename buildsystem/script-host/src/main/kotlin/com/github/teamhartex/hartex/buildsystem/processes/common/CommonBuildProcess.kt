/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2022 HarTex Project Developers
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

import com.github.teamhartex.hartex.buildsystem.CargoBuildProfile
import com.github.teamhartex.hartex.buildsystem.Project
import com.github.teamhartex.hartex.buildsystem.ProjectBuildTool
import com.github.teamhartex.hartex.buildsystem.ProjectType
import com.github.teamhartex.hartex.buildsystem.processes.BuildsystemProcess
import java.io.File

class CommonBuildProcess {
  companion object : BuildsystemProcess {
    override fun new(projectToBuild: Project, args: List<String>): Process {
      val processBuilder = ProcessBuilder()

      when (projectToBuild.projectType to projectToBuild.buildTool) {
        ProjectType.RUST to ProjectBuildTool.CARGO -> {
          processBuilder.command("cargo", "build")

          when (projectToBuild.cargoBuildProfile) {
            CargoBuildProfile.RELEASE -> {
              processBuilder.command().add("--release")
            }
            else -> {}
          }
        }
        ProjectType.TYPESCRIPT to ProjectBuildTool.YARN -> processBuilder.command("yarn", "build")
      }

      return processBuilder.directory(File(System.getProperty("user.dir") + """/${args[2]}"""))
        .redirectError(ProcessBuilder.Redirect.INHERIT)
        .start()
    }
  }
}
