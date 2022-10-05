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

package com.github.teamhartex.hartex.buildsystem.kotlin.dsl.concurrent

import java.util.concurrent.TimeUnit
import java.util.concurrent.locks.ReentrantLock
import kotlin.concurrent.withLock
import kotlin.collections.List as IList

class ConcurrentGroupQueue<T>(private val supersedesFun: T.(T) -> Boolean) {
  private val lock = ReentrantLock()
  private val queue = ArrayDeque<T>()

  private val notEmpty = lock.newCondition()

  fun nextGroup(timeoutMilliseconds: Long = 5000L): IList<T> {
    lock.withLock {
      if (queue.isNotEmpty())
        takeNextGroup()

      if (notEmpty.await(timeoutMilliseconds, TimeUnit.MILLISECONDS))
        return takeNextGroup()
    }

    return emptyList()
  }

  fun push(element: T) {
    lock.withLock {
      queue.addFirst(element)

      if (queue.size == 1)
        notEmpty.signal()
    }
  }

  private fun takeNextGroup(): IList<T> {
    require(queue.isNotEmpty())

    val group = mutableListOf<T>()
    queue.iterator().run {
      val nextItem = next()
      group.add(nextItem)
      remove()

      for (next in this) {
        if (next.supersedesFun(next)) {
          group.add(next)
          remove()
        }
      }
    }

    return group
  }
}
