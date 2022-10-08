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
import kotlin.collections.Collection
import kotlin.collections.Collection as ICollection

class MutablePropertyDetails(private val name: String) : IPropertyDetails {
  private val getters = MethodSet()
  private val setters = MethodSet()

  private lateinit var field: Field

  override fun getName(): String = name

  override fun getGetters(): ICollection<Method> = getters.values()

  override fun getSetters(): Collection<Method> = setters.values()

  override fun getBackingField(): Field = field

  fun addGetter(getter: Method) {
    getters.add(getter)
  }

  fun addSetter(setter: Method) {
    setters.add(setter)
  }

  fun setBackingField(field: Field) {
    this.field = field
  }
}
