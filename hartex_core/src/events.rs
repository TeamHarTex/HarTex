//! # The `events` Module
//! 
//! The `events` module contains definitions for events emitted from the Discord API as well as
//! our custom events.

/// # Enum `EventType`
/// 
/// Represents the type of an event.
pub enum EventType {
    /// # Enum Variant `EventType::Twilight`
    /// 
    /// A twilight event.
    Twilight,
    
    /// # Enum Variant `EventType::Custom`
    /// 
    /// A custom event.
    Custom
}
