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
