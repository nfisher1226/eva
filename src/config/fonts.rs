#![warn(clippy::all, clippy::pedantic)]
use serde::{Deserialize, Serialize};

use std::{fmt, fs};
use std::str::FromStr;

/// The weight, or style, of the font
#[derive(Clone, Copy, Deserialize, Debug, PartialEq, Serialize)]
pub enum FontStyle {
    Thin,
    Ultralight,
    Light,
    Semilight,
    Book,
    Normal,
    Medium,
    Semibold,
    Bold,
    Ultrabold,
    Heavy,
    Ultraheavy,
}

/// The font used to print the description in the output file
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Font {
    /// The *family* , eg *Sans* or *ComicSans*
    pub family: String,
    /// The *style* of the given font
    pub style: FontStyle,
    /// The size of the font
    pub size: usize,
}

/// Error returned if unable to parse a font from a given `str`
#[derive(Debug, PartialEq)]
pub struct ParseFontError;

impl fmt::Display for ParseFontError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for FontStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for FontStyle {
    fn default() -> Self {
        Self::Normal
    }
}

impl FromStr for FontStyle {
    type Err = ParseFontError;

    #[allow(clippy::must_use_candidate)]
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "Style::Thin" | "Style::thin" => Ok(FontStyle::Thin),
            "Style::Ultralight" | "Style::ultralight" => Ok(FontStyle::Ultralight),
            "Style::Light" | "Style::light" => Ok(FontStyle::Light),
            "Style::Semilight" | "Style::semilight" => Ok(FontStyle::Semilight),
            "Style::Book" | "Style::book" => Ok(FontStyle::Book),
            "Style::Normal" | "Style::normal" | "Style::Regular" | "Style::regular" => {
                Ok(FontStyle::Normal)
            }
            "Style::Medium" | "Style::medium" => Ok(FontStyle::Medium),
            "Style::Semibold" | "Style::semibold" => Ok(FontStyle::Semibold),
            "Style::Bold" | "Style::bold" => Ok(FontStyle::Bold),
            "Style::Ultrabold" | "Style::ultrabold" => Ok(FontStyle::Ultrabold),
            "Style::Heavy" | "Style::heavy" => Ok(FontStyle::Heavy),
            "Style::Ultraheavy" | "Style::ultraheavy" => Ok(FontStyle::Ultraheavy),
            _ => Err(ParseFontError),
        }
    }
}

impl Default for Font {
    /// Returns "Sans Normal"
    fn default() -> Self {
        Self {
            family: String::from("Sans"),
            style: FontStyle::default(),
            size: 13,
        }
    }
}

impl Font {
    /// Get the *family* of the font
    pub fn family(&self) -> String {
        String::from(&self.family)
    }

    /// Set the *family* of the font
    pub fn set_family(&mut self, family: String) {
        self.family = family;
    }

    /// Get the *styleof the font
    pub fn style(&self) -> FontStyle {
        self.style
    }

    /// Set the *style* or *style* of the font
    pub fn set_style(&mut self, style: FontStyle) {
        self.style = style;
    }

    /// Get the *size* of the font
    pub fn size(&self) -> usize {
        self.size
    }

    /// Set the *size* of the font
    pub fn set_size(&mut self, size: usize) {
        self.size = size;
    }
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Fonts {
    paragraph: Font,
    pre: Font,
    heading: Font,
    quote: Font,
}

impl Default for Fonts {
    fn default() -> Self {
        Self {
            paragraph: Font::default(),
            pre: Font {
                family: String::from("monospace"),
                style: FontStyle::default(),
                size: 13,
            },
            heading: Font {
                family: String::from("sans-serif"),
                style: FontStyle::default(),
                size: 18,
            },
            quote: Font::default(),
        }
    }
}
