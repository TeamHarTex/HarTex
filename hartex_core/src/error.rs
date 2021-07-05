//! # The `error` Module
//!
//! This module defines several types for error handling in the HarTex Discord bot.

use ctrlc::Error as CtrlcError;

use crate::discord::{
    gateway::cluster::ClusterStartError,
    http::error::Error as HttpError,
    model::gateway::payload::update_presence::UpdatePresenceError
};

/// # Enum `HarTexError`
///
/// An enumeration representing the various error types used within HarTex.
#[derive(Debug)]
pub enum HarTexError {
    /// # Enum Variant HarTexError::ClusterStartError
    ///
    /// A wrapper around `twilight_gateway::cluster::ClusterStartError`.
    ///
    /// ## Fields
    /// - `error`, type `ClusterStartError`: the cluster start error returned when building the
    ///                                      cluster.
    ClusterStartError {
        error: ClusterStartError
    },

    /// # Enum Variant `HarTexError::CtrlcError`
    ///
    /// A wrapper around `ctrlc::Error`.
    ///
    /// ## Fields
    /// - `error`, type `Error`: the ctrlc error returned when setting the ctrl-c handler.
    CtrlcError {
        error: CtrlcError
    },
    
    /// # Enum Variant `HHarTexError::TwilightHttpError`
    /// 
    /// A wrapper around `twilight_http::error::Error`.
    /// 
    /// ## Fields
    /// - `error`, type `Error`: the error returned when executing an HTTP request.
    TwilightHttpError {
        error: HttpError
    },

    /// # Enum Variant `HarTexError::UpdatePresenceError`
    ///
    /// A wrapper around `twilight_model::gateway::paylod::update_presence::UpdatePresenceError`.
    ///
    /// ## Fields
    /// - `error`, type `UpdatePresenceError`: the error presence update error returned when
    ///                                        attempting to update the bot's presence.
    UpdatePresenceError {
        error: UpdatePresenceError
    },

    /// # Enum Variant `HarTexError::Custom`
    ///
    /// Represents a custom error that cannot be represented with any other variants of this
    /// enumeration.
    ///
    /// ## Fields
    /// - `message`, type `&str`: the error message.
    Custom {
        message: String
    }
}

impl From<ClusterStartError> for HarTexError {
    fn from(error: ClusterStartError) -> Self {
        Self::ClusterStartError {
            error
        }
    }
}

impl From<CtrlcError> for HarTexError {
    fn from(error: CtrlcError) -> Self {
        Self::CtrlcError {
            error
        }
    }
}

impl From<HttpError> for HarTexError {
    fn from(error: HttpError) -> Self {
        Self::TwilightHttpError {
            error
        }
    }
}

impl From<UpdatePresenceError> for HarTexError {
    fn from(error: UpdatePresenceError) -> Self {
        Self::UpdatePresenceError {
            error
        }
    }
}

/// # Type Alias `HarTexResult<T>`
///
/// A type alias for `Result<T, HarTexError>`, used for error-handling.
pub type HarTexResult<T> = Result<T, HarTexError>;
