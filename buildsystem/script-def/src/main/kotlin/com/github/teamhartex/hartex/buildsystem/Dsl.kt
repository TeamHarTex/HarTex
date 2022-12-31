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

package com.github.teamhartex.hartex.buildsystem

import kotlin.properties.Delegates

enum class CargoBuildProfile {
  DEBUG,
  RELEASE
}

open class Project(val projectType: ProjectType) {
  lateinit var cargoBuildProfile: CargoBuildProfile
  lateinit var buildTool: ProjectBuildTool
  var includeDebugInformation by Delegates.notNull<Boolean>()
  lateinit var linker: String
  lateinit var outputDirectory: String
}

open class Projects {
  val projects: MutableMap<String, Project> = HashMap()

  fun project(nameAndPath: String, projectType: ProjectType): Projects {
    projects[nameAndPath] = Project(projectType)
    return this
  }
}

enum class ProjectBuildTool {
  CARGO,
  GRADLE,
  YARN
}

open class ProjectConfigurationForProjectScope(private val forProject: Project) {
  fun buildTool(buildTool: ProjectBuildTool) {
    forProject.buildTool = buildTool
  }

  fun cargoBuildProfile(cargoBuildProfile: CargoBuildProfile) {
    forProject.cargoBuildProfile = cargoBuildProfile
  }

  fun includeDebugInformation(includeDebugInformation: Boolean) {
    forProject.includeDebugInformation = includeDebugInformation
  }

  fun linker(linker: String) {
    forProject.linker = linker
  }

  fun outputDirectory(buildDirectory: String) {
    forProject.outputDirectory = buildDirectory
  }
}

enum class ProjectType {
  JVM,
  TYPESCRIPT,
  RUST
}

open class Workspace {
  open val projects: Projects = Projects()

  fun buildConfigurationForProject(forProject: String, block: ProjectConfigurationForProjectScope.() -> Unit) {
    val project = projects.projects[forProject] ?: throw NullPointerException()
    block(ProjectConfigurationForProjectScope(project))
  }

  fun projects(block: Projects.() -> Projects) {
    block(projects)
  }
}
