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

package com.github.teamhartex.hartex.buildsystem.service

import com.github.teamhartex.hartex.buildsystem.IFactory
import kotlin.collections.List
import kotlin.reflect.KClass
import java.lang.reflect.Type as IType
import kotlin.collections.List as IList
import kotlin.reflect.KClass as IKClass

interface IServiceRegistry : IServiceLookup {
  fun <T: Any> get(serviceType: IKClass<T>): T

  fun <T: Any> getAll(serviceType: IKClass<T>) : IList<T>

  fun <T: Any> getFactory(serviceType: IKClass<T>): IFactory<T>

  override fun get(serviceType: IType): Any

  override fun lookup(serviceType: IType): Any?

  fun <T: Any> newInstance(serviceType: IKClass<T>): T

  companion object {
    val EMPTY_REGISTRY = object : IServiceRegistry {
      private fun emptyServiceRegistryException(type: IType): UnknownServiceException =
        UnknownServiceException(type, "nothing is available in the empty service registry")

      override fun <T : Any> get(serviceType: KClass<T>): T =
        throw emptyServiceRegistryException(serviceType.java)

      override fun get(serviceType: IType): Any =
        throw emptyServiceRegistryException(serviceType)

      override fun get(serviceType: IType, annotatedWith: KClass<Annotation>) =
        throw emptyServiceRegistryException(serviceType)

      override fun <T : Any> getAll(serviceType: KClass<T>): List<T> = emptyList()

      override fun lookup(serviceType: IType): Any? = null

      override fun <T : Any> getFactory(serviceType: KClass<T>): IFactory<T> =
        throw emptyServiceRegistryException(serviceType.java)

      override fun <T : Any> newInstance(serviceType: KClass<T>): T =
        throw emptyServiceRegistryException(serviceType.java)
    }
  }
}
