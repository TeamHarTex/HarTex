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

package com.github.teamhartex.hartex.buildsystem

import java.io.File
import kotlin.script.experimental.api.ResultWithDiagnostics
import kotlin.script.experimental.api.ScriptDiagnostic
import kotlin.script.experimental.api.onSuccess
import org.fusesource.jansi.AnsiConsole

fun main(vararg args: String) {
  AnsiConsole.systemInstall()

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
      ProjectBuilder.build(it.returnValue.scriptClass!!, *args)

      ResultWithDiagnostics.Success(it)
    }
  }

  AnsiConsole.systemUninstall()
}
