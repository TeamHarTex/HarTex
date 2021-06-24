//! # The `ansi` Module
//!
//! This module contains utilities for using ANSI Escape Sequences for use when printing to stdout.

use std::fmt::Display;

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
    /// Black, converted to `30` when the `AnsiColour::into_i32s` instance method is invoked on this
    /// variant with the parameter `foreground` set to `true`, `40` otherwise.
    Black,

    /// # Enum Variant `AnsiColour::Red`
    ///
    /// Red, converted to `31` when the `AnsiColour::into_i32s` instance method is invoked on this
    /// variant with the parameter `foreground` set to `true`, `41` otherwise.
    Red,

    /// # Enum Variant `AnsiColour::Green`
    ///
    /// Green, converted to `32` when the `AnsiColour::into_i32s` instance method is invoked on this
    /// variant with the parameter `foreground` set to `true`, `42` otherwise.
    Green,

    /// # Enum Variant `AnsiColour::Yellow`
    ///
    /// Yellow, converted to `33` when the `AnsiColour::into_i32s` instance method is invoked on
    /// this variant with the parameter `foreground` set to `true`, `43` otherwise.
    Yellow,

    /// # Enum Variant `AnsiColour::Blue`
    ///
    /// Blue, converted to `34` when the `AnsiColour::into_i32s` instance method is invoked on this
    /// variant with the parameter `foreground` set to `true`, `44` otherwise.
    Blue,

    /// # Enum Variant `AnsiColour::Mangenta`
    ///
    /// Magenta, converted to `35` when the `AnsiColour::into_i32s` instance method is invoked on
    /// this variant with the parameter `foreground` set tp `true`, `45` otherwise.
    Magenta,

    /// # Enum Variant `AnsiColour::Cyan`
    ///
    /// Cyan, converted to `36` when the `AnsiColour::into_i32s` instance method is invoked on this
    /// variant with the parameter `foreground` set to `true`, `46` otherwise.
    Cyan,

    /// # Enum Variant `AnsiColour::White`
    ///
    /// White, converted to `37` when the `AnsiColour::into_i32s` instance method is invoked on this
    /// variant with the parameter `foreground` set to `true`, `47` otherwise.
    White,

    /// # Enum Variant `AnsiColour::CustomU8`
    ///
    /// Allows the user to set a custom `u8` value for the colour, converted to `38` when the
    /// `AnsiColour::into_i32s` instance method is invoked on this variant with the parameter
    /// `foreground` set to `true`, `48` otherwise.
    CustomU8 {
        n: u8
    },

    /// # Enum Variant `AnsiColour::CustomRgb`
    ///
    /// Allow the user to set a custom red-green-blue values (all are `u8`s), converted to `38`
    /// when the `AnsiColour::into_i32s` instance method is invoked on this variant with the
    /// parameter `foreground` is set to `true`, `48` otherwise.
    CustomRgb {
        r: u8,
        g: u8,
        b: u8
    }
}

impl AnsiColour {
    /// # Instance Method `AnsiColour::into_i32s`
    ///
    /// Converts the current `AnsiColour` instance to an `i32` for terminal usage.
    ///
    /// ## Parameters
    /// - `foreground`, type `bool`: whether the colour specified is foreground
    ///
    /// ## Return Type
    /// `Vec<i32>`
    pub fn into_i32s(self, foreground: bool) -> Vec<i32> {
        match self {
            Self::Black => vec![ if foreground { 30 } else { 40 } ],
            Self::Red => vec![ if foreground { 31 } else { 41 } ],
            Self::Green => vec![ if foreground { 32 } else { 42 } ],
            Self::Yellow => vec![ if foreground { 33 } else { 43 } ],
            Self::Blue => vec![ if foreground { 34 } else { 44 } ],
            Self::Magenta => vec![ if foreground { 35 } else { 45 } ],
            Self::Cyan => vec![ if foreground { 36 } else { 46} ],
            Self::White => vec![ if foreground { 37 } else { 47 } ],
            Self::CustomU8 { n } => vec![ if foreground { 38 } else { 48 }, n ],
            Self::CustomRgb { r, g, b} => vec![ if foreground { 38 } else { 48 }, r, g, b]
        }
    }
}

/// # Enum `SgrParam`
///
/// An enumerate representing the SGR Parameters, also known as the Select Graphics Rendition
/// Parameters; which sets display attributes.
pub enum SgrParam {
    /// # Enum Variant `SgrParam::Reset`
    ///
    /// Reset or normal; converted to `0` when the `SgrParam::into_i32s`s` instance method is invoked
    /// on this variant.
    Reset,

    /// # Enum Variant `SgrParam::BoldOrIncreasedIntensity`
    ///
    /// Changes the font weight to bold or increases the intensity of the glpyh; converted to `1`
    /// when the `SgrParam::into_i32s` instance method is invoked on this variant.
    BoldOrIncreasedIntensity,

    /// # Enum Variant `SgrParam::LightOrDecreasedIntensity`
    ///
    /// Changes the font weight to light or decreases the intensity of the glyph; converted to `2`
    /// when the `SgrParam::into_i32s` instance method is invoked on this variant.
    LightOrDecreasedIntensity,

    /// # Enum Variant `SgrParam::Italic`
    ///
    /// Changes the glyph to italics, however is not widely supported; converted to `3` when the
    /// `SgrParam::into_i32s` instance method is invoked on this variant.
    Italic,

    /// # Enum Variant `SgrParam::Underline`
    ///
    /// Underlines the glpyh; converted to `4` when the `SgrParam::into_i32s` instance method is
    /// invoked on this variant.
    Underline,

    /// # Enum Variant `SgrParam::SetColour`
    ///
    /// Sets the colour, either foreground or background; converted to `5` and the following
    /// parameters when the `SgrParam::into_i32s` instance method is invoked on this variant.
    ///
    /// ## Fields
    /// - `colour`, type `AnsiColour`: the colour to set.
    /// - `foreground`, type `bool`: whether the colour set is used for foreground; `false` when
    ///                              the colour is used for background.
    SetColour {
        colour: AnsiColour,
        foreground: bool
    }
}

impl SgrParam {
    /// # Instance Method `SgrParam::into_i32s`
    ///
    /// Converts the current `SgrParam` instance to an `i32` for terminal usage.
    ///
    /// ## Return Type
    /// `Vec<i32>`
    pub fn into_i32s(self) -> Vec<i32> {
        match self {
            Self::Reset => vec![0],
            Self::BoldOrIncreasedIntensity => vec![1],
            Self::LightOrDecreasedIntensity => vec![2],
            Self::Italic => vec![3],
            Self::Underline => vec![4],
            Self::SetColour { colour, foreground } => {
                let mut vec = vec![5];
                vec.append(&mut colour.into_i32s(foreground));

                vec
            }
        }
    }
}

/// # Function `ansi_display`
/// 
/// Converts the provided parameters to a string that is `Display`able.
/// 
/// ## Parameters
/// - `params`, type `Vec<i32>`: the parameters to convert
pub fn ansi_display(params: Vec<i32>) -> impl Display {
    format!("{}[{}m", ANSI_ESC_CHAR, params.join(";"))
}
