package com.github.teamhartex.hartex.buildsystem

import java.io.File
import kotlin.script.experimental.api.EvaluationResult
import kotlin.script.experimental.api.ResultWithDiagnostics
import kotlin.script.experimental.host.toScriptSource
import kotlin.script.experimental.jvmhost.BasicJvmScriptingHost
import kotlin.script.experimental.jvmhost.createJvmCompilationConfigurationFromTemplate

class BuildsystemKotlinScriptHost {
  companion object {
    fun executeScript(scriptFile: File): ResultWithDiagnostics<EvaluationResult> {
      val compilationConfiguration = createJvmCompilationConfigurationFromTemplate<AbstractBuildsystemKotlinScriptDefinition>()
      return BasicJvmScriptingHost().eval(scriptFile.toScriptSource(), compilationConfiguration, null)
    }
  }
}
