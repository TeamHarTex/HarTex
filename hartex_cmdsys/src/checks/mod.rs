//! # The `checks` Module
//!
//! This module contains implementations of several pre-command checks.

use hartex_core::{
    discord::model::id::UserId,
    error::HarTexResult
};

use crate::context::CommandContext;

/// # Trait `Check`
///
/// A pre-command check.
///
/// ## Trait Types
/// - `CheckRetType`: the return type that the check returns
pub trait Check {
    type CheckRetType;

    fn execute<'asynchronous_trait>(ctx: CommandContext, params: CheckParams) -> HarTexResult<Self::CheckRetType>;
}

/// # Struct `CheckParams`
///
/// The parameters to pass to a check.
pub struct CheckParams {
    // the user id of the message author
    user_id: Option<UserId>
}

impl CheckParams {
    /// # Static Method `CheckParams::builder`
    ///
    /// Constructs a new `CheckParamsBuilder`.
    pub fn builder() -> CheckParamsBuilder {
        CheckParamsBuilder::new()
    }

    /// # Instance Method `CheckParams::user_id`
    ///
    /// Returns the current user id parameter.
    pub fn user_id(&self) -> Option<UserId> {
        self.user_id
    }
}

/// # Struct `CheckParamsBuilder`
///
/// The builder for `CheckParams`
#[derive(Default)]
#[non_exhaustive]
pub struct CheckParamsBuilder {
    // the user id of the message author
    user_id: Option<UserId>
}

impl CheckParamsBuilder {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// # Instance Method `CheckParamsBuilder::user_id`
    ///
    /// Consumes and updates the user id of this builder.
    pub fn user_id(mut self, user_id: UserId) -> Self {
        self.user_id.replace(user_id);
        self
    }

    /// # Instance Method `CheckParamsBuilder::user_id`
    ///
    /// Consumes this builder and return a `CheckParams`.
    pub fn build(self) -> CheckParams {
        CheckParams {
            user_id: self.user_id
        }
    }
}