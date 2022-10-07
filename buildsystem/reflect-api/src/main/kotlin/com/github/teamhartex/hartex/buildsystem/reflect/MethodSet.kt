package com.github.teamhartex.hartex.buildsystem.reflect

import com.google.common.collect.Maps
import java.lang.reflect.Method
import kotlin.collections.Iterable as IIterable
import kotlin.collections.List as IList
import kotlin.collections.Map as IMap
import kotlin.reflect.KClass as IKClass

class MethodSet : IIterable<Method> {
  private val methods: IMap<MethodKey, Method> = Maps.newLinkedHashMap()

  class MethodKey {
    private val method: Method
    private val parameterTypes: IList<IKClass<Any>>

    constructor(method: Method) {
      this.method = method
      this.parameterTypes = method.parameterTypes.map { it.kotlin }
    }
  }
}
