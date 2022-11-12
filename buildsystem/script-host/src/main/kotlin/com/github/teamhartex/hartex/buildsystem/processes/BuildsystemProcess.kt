package com.github.teamhartex.hartex.buildsystem.processes

import com.github.teamhartex.hartex.buildsystem.Project

interface BuildsystemProcess {
  fun new(projectToBuild: Project, args: List<String>): Process?
}
