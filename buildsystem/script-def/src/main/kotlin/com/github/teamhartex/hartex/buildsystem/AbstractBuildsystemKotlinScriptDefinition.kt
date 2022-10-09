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

import kotlin.script.experimental.annotations.KotlinScript
import kotlin.script.experimental.api.ScriptCompilationConfiguration
import kotlin.script.experimental.api.baseClass
import kotlin.script.experimental.api.defaultIdentifier
import kotlin.script.experimental.api.defaultImports
import kotlin.script.experimental.api.displayName
import kotlin.script.experimental.api.fileExtension
import kotlin.script.experimental.jvm.dependenciesFromCurrentContext
import kotlin.script.experimental.jvm.jvm

@KotlinScript(
  compilationConfiguration = AbstractBuildsystemKotlinScriptDefinition.Configuration::class,
)
abstract class AbstractBuildsystemKotlinScriptDefinition {
  object Configuration : ScriptCompilationConfiguration({
    baseClass(Workspace::class)
    defaultIdentifier("BuildsystemKotlinScript")
    defaultImports(CargoBuildProfile::class, ProjectBuildTool::class, ProjectType::class)
    displayName("Buildsystem Kotlin DSL")
    fileExtension("buildsystem.kts")

    jvm {
      dependenciesFromCurrentContext(wholeClasspath = true)
    }
  })
}
