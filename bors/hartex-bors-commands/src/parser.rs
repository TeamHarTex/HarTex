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

//! # Command Parser

use std::iter::Peekable;
use std::str::SplitWhitespace;

use itertools::Itertools;

use crate::BorsCommand;

pub(crate) const PREFIX: &str = "bors";

/// An error occurred during parsing.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub enum ParserError<'a> {
    /// An invalid parameter is provided.
    InvalidParameter,
    /// Command is missing.
    MissingCommand,
    /// No parameter value was provided.
    NoParameterValueProvided,
    /// Unexpected end of command.
    UnexpectedEndOfCommand,
    /// Unexpected parameter was provided.
    UnexpectedParameter(String),
    /// Unexpected parameters were provided.
    UnexpectedParameters,
    /// Unknown command.
    UnknownCommand(&'a str),
}

pub(crate) struct Parser<'a> {
    iterator: Peekable<SplitWhitespace<'a>>,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            iterator: input.split_whitespace().peekable(),
        }
    }

    fn next(&mut self) {
        self.iterator.next();
    }

    fn peek(&mut self) -> Option<&'a str> {
        self.iterator.peek().copied()
    }

    fn remaining(&mut self) -> String {
        self.iterator.join(" ")
    }
}

pub(crate) type ParserResult<'a> = Option<Result<BorsCommand, ParserError<'a>>>;

pub(crate) fn parse_approve(parser: Parser) -> ParserResult {
    parse_exact("r+", BorsCommand::Approve, parser)
}

pub(crate) fn parse_approve_eq(parser: Parser) -> ParserResult {
    parse_prefix("r=", parse_approve_eq_inner, parser)
}

pub(crate) fn parse_ping(parser: Parser) -> ParserResult {
    parse_exact("ping", BorsCommand::Ping, parser)
}

pub(crate) fn parse_try(parser: Parser) -> ParserResult {
    parse_with_params("try", parse_try_inner, parser)
}

pub(crate) fn parse_try_cancel(parser: Parser) -> ParserResult {
    parse_exact("try-", BorsCommand::TryCancel, parser)
}

fn parse_exact<'a>(
    exact: &'static str,
    expected: BorsCommand,
    mut parser: Parser<'a>,
) -> ParserResult<'a> {
    match parser.peek() {
        Some(word) if word == exact => Some(Ok(expected)),
        _ => None,
    }
}

fn parse_prefix<'a>(
    prefix: &'static str,
    expected_fn: fn(Option<&str>) -> ParserResult<'_>,
    mut parser: Parser<'a>,
) -> ParserResult<'a> {
    match parser.peek() {
        Some(word) if word.starts_with(prefix) => expected_fn(word.strip_prefix(prefix)),
        _ => None,
    }
}

fn parse_with_params<'a>(
    cmd: &'static str,
    mut f: impl FnMut(Parser<'a>) -> ParserResult<'_>,
    mut parser: Parser<'a>,
) -> ParserResult<'a> {
    match parser.peek() {
        Some(word) if word == cmd => {
            parser.next();
            f(parser)
        }
        _ => None,
    }
}

#[allow(clippy::unnecessary_wraps)]
fn parse_approve_eq_inner(remaining: Option<&str>) -> ParserResult<'_> {
    match remaining {
        Some(arg) if !arg.is_empty() => Some(Ok(BorsCommand::ApproveEq {
            reviewer: arg.to_string(),
        })),
        _ => Some(Err(ParserError::UnexpectedEndOfCommand)),
    }
}

#[allow(clippy::unnecessary_wraps)]
fn parse_try_inner(mut parser: Parser<'_>) -> ParserResult<'_> {
    let remaining = parser.remaining();

    if remaining.is_empty() {
        return Some(Ok(BorsCommand::Try { parent: None }));
    }

    let split = remaining
        .split_whitespace()
        .map(String::from)
        .collect::<Vec<String>>();
    if split.len() > 1 {
        return Some(Err(ParserError::UnexpectedParameters));
    }

    let param_segments = split[0]
        .split('=')
        .map(String::from)
        .collect::<Vec<String>>();
    if param_segments.len() < 2 {
        return Some(Err(ParserError::NoParameterValueProvided));
    }

    if param_segments[0] != "parent" {
        return Some(Err(ParserError::UnexpectedParameter(
            param_segments[0].clone(),
        )));
    }

    Some(Ok(BorsCommand::Try {
        parent: Some(param_segments[1].to_string()),
    }))
}

#[allow(clippy::unnecessary_wraps)]
pub(crate) fn parse_remaining(mut parser: Parser) -> ParserResult {
    let result = match parser.peek() {
        Some(arg) => Err(ParserError::UnknownCommand(arg)),
        None => Err(ParserError::MissingCommand),
    };

    Some(result)
}
