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

package com.github.teamhartex.hartex.buildsystem.dsl.resolver

import com.github.teamhartex.hartex.buildsystem.dsl.concurrent.future
import java.io.File
import java.util.concurrent.Future
import kotlin.script.dependencies.Environment
import kotlin.script.dependencies.KotlinScriptExternalDependencies
import kotlin.script.dependencies.ScriptContents
import kotlin.script.dependencies.ScriptDependenciesResolver

private typealias ReportFun = (ScriptDependenciesResolver.ReportSeverity, String, ScriptContents.Position?) -> Unit

class BuildScriptDependenciesResolver : ScriptDependenciesResolver {
  override fun resolve(
    script: ScriptContents,
    environment: Environment?,
    report: ReportFun,
    previousDependencies: KotlinScriptExternalDependencies?): Future<KotlinScriptExternalDependencies?> = future {
      try {
        assembleDependenciesFrom(script.file, environment!!, report, previousDependencies)
      } catch (exception: Exception) {
        previousDependencies
      }
  }

  private suspend fun assembleDependenciesFrom(
    scriptFile: File?,
    environment: Environment,
    report: ReportFun,
    previousDependencies: KotlinScriptExternalDependencies?,
    classPathBlocksHash: ByteArray? = null
  ): KotlinScriptExternalDependencies {
    TODO("to be implemented")
  }
}
