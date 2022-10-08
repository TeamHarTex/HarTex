package com.github.teamhartex.hartex.buildsystem

import java.io.File
import kotlin.script.experimental.api.ScriptDiagnostic

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
  }
}
