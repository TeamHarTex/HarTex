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

package com.github.teamhartex.hartex.buildsystem.kotlin.dsl.resolver

import com.github.teamhartex.hartex.buildsystem.kotlin.dsl.model.KotlinBuildScriptModel
import com.github.teamhartex.hartex.buildsystem.tooling.IModelBuilder
import java.io.File

internal typealias ModelBuilderCustomization_T = IModelBuilder<KotlinBuildScriptModel>.() -> Unit

data class KotlinBuildScriptModelRequest(
  val projectRoot: File,
  val scriptFile: File? = null,
  val javaHome: File? = null
)

private data class KotlinBuildScriptModelRequestFetchParameters(
  val projectRoot: File,
  val scriptFile: File?,
  val modelBuilderCustomization: ModelBuilderCustomization_T = {}
)

private
fun fetchKotlinBuildScriptModelFor(parameters: KotlinBuildScriptModelRequestFetchParameters): KotlinBuildScriptModel {
  TODO("to be implemented")
}


private
fun KotlinBuildScriptModelRequest.toFetchParametersWith(modelBuilderCustomization: ModelBuilderCustomization_T) =
  KotlinBuildScriptModelRequestFetchParameters(
    projectRoot,
    scriptFile,
    modelBuilderCustomization
  )

internal fun fetchKotlinBuildScriptModelFor(request: KotlinBuildScriptModelRequest): KotlinBuildScriptModel =
  fetchKotlinBuildScriptModelFor(
    request.toFetchParametersWith {
      request.javaHome?.let { setJavaHomeDirectory(it) }
    }
  )
