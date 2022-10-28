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

import com.github.teamhartex.hartex.buildsystem.processes.common.CommonBuildProcess
import com.github.teamhartex.hartex.buildsystem.processes.common.CommonCleanProcess
import com.github.teamhartex.hartex.buildsystem.processes.cargo.CargoClippyProcess
import com.github.teamhartex.hartex.buildsystem.processes.cargo.CargoFormatCheckProcess
import com.github.teamhartex.hartex.buildsystem.processes.cargo.CargoFormatProcess
import com.github.teamhartex.hartex.buildsystem.processes.cargo.CargoTestProcess
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

        val process = when (args[1]) {
          "build" -> CommonBuildProcess.new(projectToBuild, args.asList())
          "clean" -> CommonCleanProcess.new(projectToBuild, args.asList())
          "clippy" -> CargoClippyProcess.new(projectToBuild, args.asList())
          "format" -> CargoFormatProcess.new(projectToBuild, args.asList())
          "formatck" -> CargoFormatCheckProcess.new(projectToBuild, args.asList())
          "test" -> CargoTestProcess.new(projectToBuild, args.asList())
          else -> throw IllegalArgumentException("invalid command")
        }

        process?.waitFor()

        if (process == null)
          return

        if (process.exitValue() != 0)
          throw RuntimeException("abnormal termination")
      }
    }
  }
}
