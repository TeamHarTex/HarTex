//! # The `discord` Module
//!
//! This module contains re-exports of *most* of the `twilight` ecosystem of crates to reduce the
//! need to add the `twilight_*` dependencies to the `Cargo.toml`s of the individual separated
//! HarTex crates.

/// Re-export `twilight_cache_inmemory`
///
/// An in-process memory cache for the `twilight-rs` ecosystem. It’s responsible for processing
/// events and caching things like guilds, channels, users, and voice states.
pub use twilight_cache_inmemory as cache_inmemory;

/// Re-export `twilight_embed_builder`
///
/// A set of builders for the `twilight-rs` ecosystem for creating message embeds and are useful
/// when creating or updating messages.
pub use twilight_embed_builder as embed_builder;

/// Re-export `twilight_gateway`
///
/// An implementation of Discord’s sharding gateway sessions for the `twilight-rs` ecoststem.
/// This is responsible for receiving stateful events in real-time from Discord and sending some
/// stateful information.
pub use twilight_gateway as gateway;

/// Re-export `twilight_http`
///
/// HTTP (HyperText Transfer Protocol) support for the `twilight-rs` ecosystem.
pub use twilight_http as http;

/// Re-export `twilight_mention`
///
/// A utility crate for the `twilight-rs` ecosystem to mention its model types and parse such
/// mentions.
pub use twilight_mention as mention;

/// Re-export `twilight_model`
///
/// A crate of `serde` models defining the Discord APIs with a few convenience methods implemented
/// on top of them for the `twilight-rs` ecosystem.
pub use twilight_model as model;

/// Re-export `twilight_standby`
///
/// Standby is a utility crate for the `twilight-rs` ecossytem to wait for an event to happen based
/// on a predicate check. For example, you may have a command that has a reaction menu of ✅ and ❌.
/// If you want to handle a reaction to these, using something like an application-level state or
/// event stream may not suit your use case. It may be cleaner to wait for a reaction inline to
/// your function.
pub use twilight_standby as standby;

/// Re-export `twilight_util`
///
/// A set of utility types and functions for the `twilight-rs` ecosystem to augment or enhance
/// default functionality.
pub use twilight_util as util;
