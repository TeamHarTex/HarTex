//! # The `ansi` Module
//!
//! This module contains utilities for using ANSI Escape Sequences for use when printing to stdout.

/// # Constant `ANSI_ESC_CHAR`
///
/// The ANSI escape character used when creating an ANSI escape sequence.
pub const ANSI_ESC_CHAR: char = '\x1B';

/// # Enum `AnsiColour`
///
/// An enumerate representing the various colours or custom colours that ANSI supports.
pub enum AnsiColour {
    /// # Enum Variant `AnsiColour::Black`
    ///
    /// Black, converted to `30` when the `AnsiColour::into_i32` instance method is invoked on this
    /// variant with the parameter `foreground` set to `true`, `40` otherwise.
    Black,

    /// # Enum Variant `AnsiColour::Red`
    ///
    /// Red, converted to `31` when the `AnsiColour::into_i32` instance method is invoked on this
    /// variant with the parameter `foreground` set to `true`, `41` otherwise.
    Red,

    /// # Enum Variant `AnsiColour::Green`
    ///
    /// Green, converted to `32` when the `AnsiColour::into_i32` instance method is invoked on this
    /// variant with the parameter `foreground` set to `true`, `42` otherwise.
    Green,

    /// # Enum Variant `AnsiColour::Yellow`
    ///
    /// Yellow, converted to `33` when the `AnsiColour::into_i32` instance method is invoked on
    /// this variant with the parameter `foreground` set to `true`, `43` otherwise.
    Yellow,

    /// # Enum Variant `AnsiColour::Blue`
    ///
    /// Blue, converted to `34` when the `AnsiColour::into_i32` instance method is invoked on this
    /// variant with the parameter `foreground` set to `true`, `44` otherwise.
    Blue,

    /// # Enum Variant `AnsiColour::Mangenta`
    ///
    /// Magenta, converted to `35` when the `AnsiColour::into_i32` instance method is invoked on
    /// this variant with the parameter `foreground` set tp `true`, `45` otherwise.
    Magenta,

    /// # Enum Variant `AnsiColour::Cyan`
    ///
    /// Cyan, converted to `36` when the `AnsiColour::into_i32` instance method is invoked on this
    /// variant with the parameter `foreground` set to `true`, `46` otherwise.
    Cyan,

    /// # Enum Variant `AnsiColour::White`
    ///
    /// White, converted to `37` when the `AnsiColour::into_i32` instance method is invoked on this
    /// variant with the parameter `foreground` set to `true`, `47` otherwise.
    White,

    /// # Enum Variant `AnsiColour::CustomU8`
    ///
    /// Allows the user to set a custom `u8` value for the colour, converted to `38` when the
    /// `AnsiColour::into_i32` instance method is invoked on this variant with the parameter
    /// `foreground` set to `true`, `48` otherwise.
    CustomU8 {
        n: u8
    },

    /// # Enum Variant `AnsiColour::CustomRgb`
    ///
    /// Allow the user to set a custom red-green-blue values (all are `u8`s), converted to `38`
    /// when the `AnsiColour::into_i32` instance method is invoked on this variant with the
    /// parameter `foreground` is set to `true`, `48` otherwise.
    CustomRgb {
        r: u8,
        g: u8,
        b: u8
    }
}

/// # Enum `SgrParam`
///
/// An enumerate representing the SGR Parameters, also known as the Select Graphics Rendition
/// Parameters; which sets display attributes.
pub enum SgrParam {
    /// # Enum Variant `SgrParam::Reset`
    ///
    /// Reset or normal; converted to `0` when the `SgrParam::into_i32` instance method is invoked
    /// on this variant.
    Reset,

    /// # Enum Variant `SgrParam::BoldOrIncreasedIntensity`
    ///
    /// Changes the font weight to bold or increases the intensity of the glpyh; converted to `1`
    /// when the `SgrParam::into_i32` instance method is invoked on this variant.
    BoldOrIncreasedIntensity,

    /// # Enum Variant `SgrParam::LightOrDecreasedIntensity`
    ///
    /// Changes the font weight to light or decreases the intensity of the glyph; converted to `2`
    /// when the `SgrParam::into_i32` instance method is invoked on this variant.
    LightOrDecreasedIntensity,

    /// # Enum Variant `SgrParam::Italic`
    ///
    /// Changes the glyph to italics, however is not widely supported; converted to `3` when the
    /// `SgrParam::into_i32` instance method is invoked on this variant.
    Italic,

    /// # Enum Variant `SgrParam::Underline`
    ///
    /// Underlines the glpyh; converted to `4` when the `SgrParam::into_i32` instance method is
    /// invoked on this variant.
    Underline,

    /// # Enum Variant `SgrParam::SetColour`
    ///
    /// Sets the colour, either foreground or background.
    ///
    /// ## Fields
    /// - `colour`, type `AnsiColour`: the colour to set.
    /// - `foreground`, type `bool`:
    SetColour {
        colour: AnsiColour,
        foreground: bool
    }
}
