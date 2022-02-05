#![warn(clippy::all, clippy::pedantic)]
use serde::{Deserialize, Serialize};

use std::fmt;
use std::str::FromStr;

/// The style of the font
#[derive(Clone, Copy, Deserialize, Debug, PartialEq, Serialize)]
pub enum FontStyle {
    Normal,
    Oblique,
    Italic,
}

/// The weight of the font
#[derive(Clone, Copy, Deserialize, Debug, PartialEq, Serialize)]
pub enum FontWeight {
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
    pub family: String,
    pub style: FontStyle,
    pub weight: FontWeight,
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

impl fmt::Display for FontWeight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for FontStyle {
    fn default() -> Self {
        Self::Normal
    }
}

impl Default for FontWeight {
    fn default() -> Self {
        Self::Normal
    }
}

impl FromStr for FontStyle {
    type Err = ParseFontError;

    #[allow(clippy::must_use_candidate)]
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "Style::Normal" | "Style::normal" => Ok(FontStyle::Normal),
            "Style::Oblique" | "Style::oblique" => Ok(FontStyle::Oblique),
            "Style::Italic" | "Style::italic" => Ok(FontStyle::Italic),
            _ => Err(ParseFontError),
        }
    }
}

impl FromStr for FontWeight {
    type Err = ParseFontError;

    #[allow(clippy::must_use_candidate)]
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "Weight::Thin" | "Weight::thin" => Ok(FontWeight::Thin),
            "Weight::Ultralight" | "Weight::ultralight" => Ok(FontWeight::Ultralight),
            "Weight::Light" | "Weight::light" => Ok(FontWeight::Light),
            "Weight::Semilight" | "Weight::semilight" => Ok(FontWeight::Semilight),
            "Weight::Book" | "Weight::book" => Ok(FontWeight::Book),
            "Weight::Normal" | "Weight::normal" | "Weight::Regular" | "Weight::regular" => {
                Ok(FontWeight::Normal)
            }
            "Weight::Medium" | "Weight::medium" => Ok(FontWeight::Medium),
            "Weight::Semibold" | "Weight::semibold" => Ok(FontWeight::Semibold),
            "Weight::Bold" | "Weight::bold" => Ok(FontWeight::Bold),
            "Weight::Ultrabold" | "Weight::ultrabold" => Ok(FontWeight::Ultrabold),
            "Weight::Heavy" | "Weight::heavy" => Ok(FontWeight::Heavy),
            "Weight::Ultraheavy" | "Weight::ultraheavy" => Ok(FontWeight::Ultraheavy),
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
            weight: FontWeight::default(),
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

    /// Get the *style of the font
    pub fn style(&self) -> FontStyle {
        self.style
    }

    /// Set the *style* or *style* of the font
    pub fn set_style(&mut self, style: FontStyle) {
        self.style = style;
    }

    pub fn weight(&self) -> FontWeight {
        self.weight
    }

    pub fn set_weight(&mut self, weight: FontWeight) {
        self.weight = weight;
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
                weight: FontWeight::default(),
                size: 13,
            },
            heading: Font {
                family: String::from("sans-serif"),
                style: FontStyle::default(),
                weight: FontWeight::Bold,
                size: 18,
            },
            quote: Font::default(),
        }
    }
}
