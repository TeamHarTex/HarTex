package com.github.teamhartex.hartex.buildsystem

import java.io.File
import java.lang.IllegalArgumentException
import kotlin.reflect.full.createInstance
import kotlin.reflect.full.memberProperties
import kotlin.script.experimental.api.ResultWithDiagnostics
import kotlin.script.experimental.api.ScriptDiagnostic
import kotlin.script.experimental.api.onSuccess

fun main(vararg args: String) {
  if (args.isEmpty())
    println("no script file to execute")
  else {
    val scriptFile = File(args[0])
    val executionResult = BuildsystemKotlinScriptHost.executeScript(scriptFile)
    executionResult.reports.forEach {
      if (it.severity > ScriptDiagnostic.Severity.DEBUG)
        println(" : ${it.message}" + if (it.exception == null) "" else ": ${it.exception}")
    }
    executionResult.onSuccess {
      if (args.size > 1) {
        when (args[1]) {
          "build" -> {
            val scriptClass = it.returnValue.scriptClass!!
            val projectsField = scriptClass.memberProperties.find { field -> field.name == "projects" }!!
            val projects = projectsField.call(scriptClass.createInstance()) as Projects

            val projectToBuild = projects.projects[args[2]] ?: throw IllegalArgumentException()
            println(projectToBuild.buildTool)
          }
        }
      }

      ResultWithDiagnostics.Success(it)
    }
  }
}
