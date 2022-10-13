/* SPDX-License-Identifier: AGPL-3.0-only
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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

package com.github.teamhartex.hartex.buildsystem

import java.io.File
import kotlin.NoSuchElementException
import kotlin.reflect.KClass
import kotlin.reflect.full.createInstance
import kotlin.reflect.full.memberProperties


class ProjectBuilder {
  companion object {
    fun build(scriptClass: KClass<*>, vararg args: String) {
      if (args.size > 1) {
        val projectsField = scriptClass.memberProperties.find { field -> field.name == "projects" }!!
        val projects = projectsField.call(scriptClass.createInstance()) as Projects

        val projectToBuild = projects.projects[args[2]] ?: throw NoSuchElementException("no such project")

        when (args[1]) {
          "build" -> {
            val processBuilder = ProcessBuilder()
              .redirectOutput(ProcessBuilder.Redirect.PIPE)

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
              else -> {}
            }

            val process = processBuilder.directory(File(System.getProperty("user.dir") + """/${args[2]}"""))
              .start()
            val outputReader = process.errorStream.bufferedReader()
            var line = outputReader.readLine()
            while (line != null) {
              println(line)
              line = outputReader.readLine()
            }

            process.waitFor()

            if (process.exitValue() != 0)
              throw RuntimeException("abnormal termination of cargo process")
          }
          "clippy" -> {
            val processBuilder = ProcessBuilder()
              .redirectOutput(ProcessBuilder.Redirect.PIPE)

            when (projectToBuild.projectType to projectToBuild.buildTool) {
              ProjectType.RUST to ProjectBuildTool.CARGO -> {
                processBuilder.command("cargo", "clippy")
              }
              else -> {
                println("running clippy is not supported in non-Rust projects")
                return
              }
            }

            val process = processBuilder.directory(File(System.getProperty("user.dir") + """/${args[2]}"""))
              .start()
            val outputReader = process.errorStream.bufferedReader()
            var line = outputReader.readLine()
            while (line != null) {
              println(line)
              line = outputReader.readLine()
            }
          }
          "fmt" -> {
            val processBuilder = ProcessBuilder()
              .redirectOutput(ProcessBuilder.Redirect.PIPE)

            when (projectToBuild.projectType to projectToBuild.buildTool) {
              ProjectType.RUST to ProjectBuildTool.CARGO -> {
                processBuilder.command("cargo", "fmt", "--all", "--", "--check")
              }
              else -> {}
            }

            val process = processBuilder.directory(File(System.getProperty("user.dir") + """/${args[2]}"""))
              .start()
            val outputReader = process.errorStream.bufferedReader()
            var line = outputReader.readLine()
            while (line != null) {
              println(line)
              line = outputReader.readLine()
            }
          }
          "test" -> {
            val processBuilder = ProcessBuilder()
              .redirectOutput(ProcessBuilder.Redirect.PIPE)

            when (projectToBuild.projectType to projectToBuild.buildTool) {
              ProjectType.RUST to ProjectBuildTool.CARGO -> {
                processBuilder.command("cargo", "nextest", "run")
              }
              else -> {}
            }

            val process = processBuilder.directory(File(System.getProperty("user.dir") + """/${args[2]}"""))
              .start()
            val outputReader = process.errorStream.bufferedReader()
            var line = outputReader.readLine()
            while (line != null) {
              println(line)
              line = outputReader.readLine()
            }
          }
        }
      }
    }
  }
}
