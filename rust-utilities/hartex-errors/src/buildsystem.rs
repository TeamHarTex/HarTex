/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2023 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

use miette::Diagnostic;
use miette::SourceSpan;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[diagnostic(
    code("E0002: buildsystem::project_not_found"),
    help("check the spelling of the project name")
)]
#[error("could not find the specified project")]
pub struct ProjectNotFound {
    #[source_code]
    pub src: String,
    #[label("this project is not found")]
    pub err_span: SourceSpan,
}

#[derive(Debug, Diagnostic, Error)]
#[diagnostic(code("E0003: buildsystem::jsts_test_not_supported"))]
#[error("testing is currently not supported for jsts projects")]
pub struct JstsTestNotSupported {
    #[source_code]
    pub src: String,
    #[label("this is a jsts project")]
    pub err_span: SourceSpan,
}

#[derive(Debug, Diagnostic, Error)]
#[diagnostic(code("E0004: buildsystem::abnormal_termination"))]
#[error("the process terminated abnormally")]
pub struct AbnormalTermination;
