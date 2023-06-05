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

use crate::BorsCommand;

pub(crate) const PREFIX: &str = "bors";

/// An error occurred during parsing.
#[derive(Debug)]
pub enum ParserError<'a> {
    /// Command is missing.
    MissingCommand,
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

    fn peek(&mut self) -> Option<&'a str> {
        self.iterator.peek().copied()
    }
}

pub(crate) type ParserResult<'a> = Option<Result<BorsCommand, ParserError<'a>>>;

pub(crate) fn parse_approve(parser: Parser) -> ParserResult {
    parse_exact("r+", BorsCommand::Approve, parser)
}

pub(crate) fn parse_ping(parser: Parser) -> ParserResult {
    parse_exact("ping", BorsCommand::Ping, parser)
}

pub(crate) fn parse_try(parser: Parser) -> ParserResult {
    parse_exact("try", BorsCommand::Try, parser)
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

pub(crate) fn parse_remaining(mut parser: Parser) -> ParserResult {
    let result = match parser.peek() {
        Some(arg) => Err(ParserError::UnknownCommand(arg)),
        None => Err(ParserError::MissingCommand),
    };

    Some(result)
}
