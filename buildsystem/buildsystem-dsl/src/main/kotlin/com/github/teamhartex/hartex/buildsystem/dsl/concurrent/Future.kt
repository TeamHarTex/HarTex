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

package com.github.teamhartex.hartex.buildsystem.dsl.concurrent

import java.util.concurrent.CountDownLatch
import java.util.concurrent.Future
import java.util.concurrent.TimeUnit
import java.util.concurrent.TimeoutException
import kotlin.coroutines.Continuation
import kotlin.coroutines.CoroutineContext
import kotlin.coroutines.EmptyCoroutineContext
import kotlin.coroutines.startCoroutine

fun <T> future(context: CoroutineContext = EmptyCoroutineContext, computation: suspend () -> T) =
  ConcurrentFutureContinuation<T>(context).also {
    computation.startCoroutine(it)
  }

class ConcurrentFutureContinuation<T>(override val context: CoroutineContext) : Continuation<T>, Future<T> {
  private var result: Result<T>? = null
  private val outcomeLatch = CountDownLatch(1)

  override fun resumeWith(result: Result<T>) {
    this.result = result
    outcomeLatch.countDown()
  }

  override fun isCancelled(): Boolean = false

  override fun cancel(mayInterruptIfRunning: Boolean): Boolean = false

  override fun isDone(): Boolean = result != null

  override fun get(): T {
    outcomeLatch.await()
    return getOrThrow()
  }

  override fun get(timeout: Long, unit: TimeUnit): T =
    if (outcomeLatch.await(timeout, unit))
      getOrThrow()
    else
      throw TimeoutException()

  private fun getOrThrow() = (result as Result<T>).getOrThrow()
}
