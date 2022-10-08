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

package com.github.teamhartex.hartex.buildsystem.reflect

import java.lang.reflect.Field
import java.lang.reflect.Method
import java.util.TreeMap
import kotlin.collections.List as IList
import kotlin.collections.Map as IMap
import kotlin.collections.Set as ISet
import kotlin.reflect.KClass as IKClass

class MutableClassDetails(private val type: IKClass<*>) : IClassDetails {
  private val instanceMethods = MethodSet()
  private val properties: IMap<String, MutablePropertyDetails> = TreeMap()
  private val methods: IList<Method> = ArrayList()
  private val instanceFields: IList<Field> = ArrayList()
  private val superTypes: ISet<IKClass<Any>> = LinkedHashSet()
}