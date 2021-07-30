//! # The `isglobadmin` Module
//!
//! This module implements a check for whether the message author is the global administrator himself.

use std::env;

use hartex_core::{
    discord::model::id::UserId,
    error::{
        HarTexError,
        HarTexResult
    }
};

use hartex_logging::Logger;

use hartex_utils::FutureRetType;

use crate::{
    checks::{
        Check,
        CheckParams,
    },
    context::CommandContext
};

/// # Struct `IsGlobAdmin`
///
/// The check for whether the message author is the global administrator himself.
pub struct IsGlobAdmin;

impl Check for IsGlobAdmin {
    type CheckRetType = ();

    fn execute<'asynchronous_trait>(_: CommandContext, params: CheckParams) -> FutureRetType<'asynchronous_trait, Self::CheckRetType> {
        Box::pin(exec_check(params))
    }
}

async fn exec_check(params: CheckParams) -> HarTexResult<<IsGlobAdmin as Check>::CheckRetType> {
    let user_id = match env::var("GLOBAL_ADMINISTRATOR_UID") {
        Ok(token) => UserId(token.parse().unwrap_or(0u64)),
        Err(var_error) => {
            Logger::error(
                format!(
                    "could not obtain the global administrator user id environment variable: {}",
                    var_error
                ),
                Some(module_path!()),
                file!(),
                line!(),
                column!()
            );

            return Err(HarTexError::Custom {
                message: format!(
                    "could not obtain the global administrator user id environment variable: {}",
                    var_error
                )
            });
        }
    };

    if params.user_id().is_none() {
        return Err(HarTexError::Custom {
            message: String::from("user id should never be `None`")
        });
    }

    if params.user_id().unwrap() != user_id {
        return Err(HarTexError::Custom {
            message: String::from("message author is not the global administrator")
        });
    }

    Ok(())
}