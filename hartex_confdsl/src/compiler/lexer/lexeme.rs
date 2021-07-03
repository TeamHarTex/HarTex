//! # The `lexeme` Module
//!
//! This module defines several models for lexical analysis, typically lexemes.

/// # Struct `Lexeme`
/// 
/// Represents a lexeme in the lexeme stream.
pub struct Lexeme {
    ltype: LexemeType,
    len: usize
}

/// # Enum `LexemeType`
///
/// An enumeration of lexeme types that are present in the configuration DSL.
pub enum LexemeType {
    /// # Enum Variant `LexemeType::Identifier`
    ///
    /// An identifer. Keywords from the point of view of the lexical analyzer are still identifiers.
    Identifier,

    /// # Enum Variant `LexemeType::Eof`
    ///
    /// Represents an end of file.
    Eof
}
