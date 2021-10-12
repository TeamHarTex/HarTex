//! # The `dashacc` Module
//!
//! This module contains configuration models specifically for dashboard access configuration.

use serde::Deserialize;

/// # Struct `DashboardAccess`
///
/// Represents the dashboard access of a user.
#[derive(Deserialize)]
pub struct DashboardAccess {
    pub userId: u64,
    pub accessLevel: u8
}

#[cfg(test)]
mod tests {
    use serde_test::Token;
    use super::DashboardAccess;

    #[test]
    fn test_dashacc_de() {
        serde_test::assert_de_tokens(
            &[
                DashboardAccess {
                    userId: 1234567887654321,
                    accessLevel: 0
                }
            ],
            &[]
        );
    }
}