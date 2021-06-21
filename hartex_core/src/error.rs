//! # The `error` Module
//!
//! This module defines several types for error handling in the HarTex Discord bot.

/// # Enum `HarTexError`
///
/// An enumeration representing the various error types used within HarTex.
pub enum HarTexError {
    /// # Enum Variant `HarTexError::Custom`
    ///
    /// Represents a custom error that cannot be represented with any other variants of this
    /// enumeration.
    ///
    /// ## Fields
    /// - `message`, type `String`: the error message.
    Custom {
        message: String
    }
}

/// # Type Alias `HarTexResult<T>`
///
/// A type alias for `Result<T, HarTexError>`, used for error-handling.
pub type HarTexResult<T> = Result<T, HarTexError>;
