package com.github.teamhartex.hartex.buildsystem.reflect

import com.google.common.collect.Maps
import java.lang.reflect.Method
import java.lang.reflect.Modifier
import java.util.Collections
import java.util.Objects
import kotlin.collections.Collection as ICollection
import kotlin.collections.Iterable as IIterable
import kotlin.collections.MutableMap as IMutableMap
import kotlin.reflect.KClass as IKClass

class MethodSet : IIterable<Method> {
  private var methods: IMutableMap<MethodKey, Method> = Maps.newLinkedHashMap()

  fun add(method: Method) {
    val key = MethodKey(method)
    val old = methods[key]

    if (old == null || shouldNewReplaceOld(old, method))
      methods[key] = method
  }

  fun isEmpty(): Boolean = methods.isEmpty()

  override fun iterator(): Iterator<Method> = values().iterator()

  private fun shouldNewReplaceOld(old: Method, new: Method): Boolean {
    val isOldAbstract = Modifier.isAbstract(old.modifiers)
    val isNewAbstract = Modifier.isAbstract(new.modifiers)

    if (isOldAbstract != isNewAbstract)
      return isOldAbstract

    return old.isBridge && !new.isBridge
  }

  private fun values(): ICollection<Method> = Collections.unmodifiableCollection(methods.values)

  class MethodKey(private val method: Method) {
    private val parameterTypes: Array<IKClass<out Any>> = method.parameterTypes.map { it.kotlin }.toTypedArray()

    override fun equals(other: Any?): Boolean {
      if (this === other)
        return true
      else if (other == null || this::class != other::class)
        return false

      val otherKey = other as MethodKey
      return Objects.equals(this.method.name, otherKey.method.name)
        && Objects.equals(this.method.returnType, otherKey.method.returnType)
        && this.parameterTypes.contentEquals(otherKey.parameterTypes)
    }

    override fun hashCode(): Int {
      return Objects.hash(method.name, parameterTypes.size)
    }
  }
}
