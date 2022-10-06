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

import com.github.teamhartex.hartex.buildsystem.kotlin.dsl.concurrent.ConcurrentGroupQueue
import com.github.teamhartex.hartex.buildsystem.kotlin.dsl.concurrent.ResurrectingThread
import com.github.teamhartex.hartex.buildsystem.kotlin.dsl.model.KotlinBuildScriptModel
import kotlin.coroutines.suspendCoroutine
import kotlin.collections.List as IList
import kotlin.coroutines.Continuation as IContinuation

private typealias AsynchronousKotlinBuildScriptModelRequest_T = Pair<KotlinBuildScriptModelRequest, IContinuation<KotlinBuildScriptModel?>>

open class KotlinBuildScriptModelRepository {
  private val requestProcessor = ResurrectingThread("Kotlin Build Script Model Repository") {
    while (true) {
      val group = queue.nextGroup()
      if (group.isEmpty())
        break

      process(group)
    }
  }

  private val queue = ConcurrentGroupQueue<AsynchronousKotlinBuildScriptModelRequest_T> {
      first.scriptFile == it.first.scriptFile && first.projectRoot == it.first.projectRoot
  }

  open fun fetch(request: KotlinBuildScriptModelRequest): KotlinBuildScriptModel =
    fetchKotlinBuildScriptModelFor(request)

  open suspend fun requestScriptModel(request: KotlinBuildScriptModelRequest): KotlinBuildScriptModel? =
    suspendCoroutine {
      accept(request, it)
      requestProcessor.wake()
    }

  open fun accept(request: KotlinBuildScriptModelRequest, continuation: IContinuation<KotlinBuildScriptModel?>) {
    queue.push(request to continuation)
  }

  private fun process(group: IList<AsynchronousKotlinBuildScriptModelRequest_T>) {
    val (request, continuation) = group.first()
    val requestResult = runCatching {
      fetch(request)
    }

    resumeAll(supersededRequests(group), Result.success(null))
    resume(continuation, requestResult)
  }

  private fun resume(continuation: IContinuation<KotlinBuildScriptModel?>, result: Result<KotlinBuildScriptModel?>) {
    ignoreErrors { continuation.resumeWith(result) }
  }

  private fun resumeAll(continuations: Sequence<IContinuation<KotlinBuildScriptModel?>>, result: Result<KotlinBuildScriptModel?>) {
    for (continuation in continuations) {
      resume(continuation, result)
    }
  }

  private fun supersededRequests(group: IList<AsynchronousKotlinBuildScriptModelRequest_T>) =
    group.asReversed().asSequence().take(group.size - 1).map { (_, continuation) -> continuation }

  private inline fun ignoreErrors(block: () -> Unit) {
    try {
      block()
    } catch (throwable: Throwable) {
      throwable.printStackTrace()
    }
  }
}
