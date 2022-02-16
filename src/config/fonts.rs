#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::needless_pass_by_value)]
use gtk::pango;
use serde::{Deserialize, Serialize};

use std::fmt;
use std::str::FromStr;

/// The style of the font
#[derive(Clone, Copy, Deserialize, Debug, PartialEq, Serialize)]
pub enum Style {
    Normal,
    Oblique,
    Italic,
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::Normal
    }
}

impl FromStr for Style {
    type Err = ParseFontError;

    #[allow(clippy::must_use_candidate)]
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "Style::Normal" | "Style::normal" => Ok(Self::Normal),
            "Style::Oblique" | "Style::oblique" => Ok(Self::Oblique),
            "Style::Italic" | "Style::italic" => Ok(Self::Italic),
            _ => Err(ParseFontError),
        }
    }
}

impl Style {
    pub fn to_pango(&self) -> pango::Style {
        match self {
            Self::Normal => pango::Style::Normal,
            Self::Oblique => pango::Style::Oblique,
            Self::Italic => pango::Style::Italic,
        }
    }

    pub fn from_pango(style: pango::Style) -> Self {
        match style {
            pango::Style::Oblique => Self::Oblique,
            pango::Style::Italic => Self::Italic,
            _ => Self::Normal,
        }
    }
}

/// The weight of the font
#[derive(Clone, Copy, Deserialize, Debug, PartialEq, Serialize)]
pub enum Weight {
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

impl fmt::Display for Weight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for Weight {
    fn default() -> Self {
        Self::Normal
    }
}

impl FromStr for Weight {
    type Err = ParseFontError;

    #[allow(clippy::must_use_candidate)]
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "Weight::Thin" | "Weight::thin" => Ok(Self::Thin),
            "Weight::Ultralight" | "Weight::ultralight" => Ok(Self::Ultralight),
            "Weight::Light" | "Weight::light" => Ok(Self::Light),
            "Weight::Semilight" | "Weight::semilight" => Ok(Self::Semilight),
            "Weight::Book" | "Weight::book" => Ok(Self::Book),
            "Weight::Normal" | "Weight::normal" | "Weight::Regular" | "Weight::regular" => {
                Ok(Self::Normal)
            }
            "Weight::Medium" | "Weight::medium" => Ok(Self::Medium),
            "Weight::Semibold" | "Weight::semibold" => Ok(Self::Semibold),
            "Weight::Bold" | "Weight::bold" => Ok(Self::Bold),
            "Weight::Ultrabold" | "Weight::ultrabold" => Ok(Self::Ultrabold),
            "Weight::Heavy" | "Weight::heavy" => Ok(Self::Heavy),
            "Weight::Ultraheavy" | "Weight::ultraheavy" => Ok(Self::Ultraheavy),
            _ => Err(ParseFontError),
        }
    }
}

impl Weight {
    pub fn to_pango(&self) -> pango::Weight {
        match self {
            Self::Thin => pango::Weight::Thin,
            Self::Ultralight => pango::Weight::Ultralight,
            Self::Light => pango::Weight::Light,
            Self::Semilight => pango::Weight::Semilight,
            Self::Book => pango::Weight::Book,
            Self::Normal => pango::Weight::Normal,
            Self::Medium => pango::Weight::Medium,
            Self::Semibold => pango::Weight::Semibold,
            Self::Bold => pango::Weight::Bold,
            Self::Ultrabold => pango::Weight::Ultrabold,
            Self::Heavy => pango::Weight::Heavy,
            Self::Ultraheavy => pango::Weight::Ultraheavy,
        }
    }

    pub fn from_pango(weight: pango::Weight) -> Self {
        match weight {
            pango::Weight::Thin => Self::Thin,
            pango::Weight::Ultralight => Self::Ultralight,
            pango::Weight::Light => Self::Light,
            pango::Weight::Semilight => Self::Semilight,
            pango::Weight::Book => Self::Book,
            pango::Weight::Medium => Self::Medium,
            pango::Weight::Semibold => Self::Semibold,
            pango::Weight::Bold => Self::Bold,
            pango::Weight::Ultrabold => Self::Ultrabold,
            pango::Weight::Heavy => Self::Heavy,
            pango::Weight::Ultraheavy => Self::Ultraheavy,
            _ => Self::Normal,
        }
    }
}

/// The stretch of the font
#[derive(Clone, Copy, Deserialize, Debug, PartialEq, Serialize)]
pub enum Stretch {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

impl fmt::Display for Stretch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for Stretch {
    fn default() -> Self {
        Self::Normal
    }
}

impl FromStr for Stretch {
    type Err = ParseFontError;

    #[allow(clippy::must_use_candidate)]
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "Stretch::UltraCondensed" => Ok(Self::UltraCondensed),
            "Stretch::ExtraCondensed" => Ok(Self::ExtraCondensed),
            "Stretch::Condensed" => Ok(Self::Condensed),
            "Stretch::SemiCondensed" => Ok(Self::SemiCondensed),
            "Stretch::Normal" => Ok(Self::Normal),
            "Stretch::SemiExpanded" => Ok(Self::SemiExpanded),
            "Stretch::Expanded" => Ok(Self::Expanded),
            "Stretch::ExtraExpanded" => Ok(Self::ExtraExpanded),
            "Stretch::UltraExpanded" => Ok(Self::UltraExpanded),
            _ => Err(ParseFontError),
        }
    }
}

impl Stretch {
    pub fn to_pango(&self) -> pango::Stretch {
        match self {
            Self::UltraCondensed => pango::Stretch::UltraCondensed,
            Self::ExtraCondensed => pango::Stretch::ExtraCondensed,
            Self::Condensed => pango::Stretch::Condensed,
            Self::SemiCondensed => pango::Stretch::SemiCondensed,
            Self::Normal => pango::Stretch::Normal,
            Self::SemiExpanded => pango::Stretch::SemiExpanded,
            Self::Expanded => pango::Stretch::Expanded,
            Self::ExtraExpanded => pango::Stretch::ExtraExpanded,
            Self::UltraExpanded => pango::Stretch::UltraExpanded,
        }
    }

    pub fn from_pango(stretch: pango::Stretch) -> Self {
        match stretch {
            pango::Stretch::UltraCondensed => Self::UltraCondensed,
            pango::Stretch::ExtraCondensed => Self::ExtraCondensed,
            pango::Stretch::Condensed => Self::Condensed,
            pango::Stretch::SemiCondensed => Self::SemiCondensed,
            pango::Stretch::SemiExpanded => Self::SemiExpanded,
            pango::Stretch::Expanded => Self::Expanded,
            pango::Stretch::ExtraExpanded => Self::ExtraExpanded,
            pango::Stretch::UltraExpanded => Self::UltraExpanded,
            _ => Self::Normal,
        }
    }
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Font {
    pub family: String,
    pub style: Style,
    pub weight: Weight,
    pub stretch: Stretch,
    pub size: i32,
}

/// Error returned if unable to parse a font from a given `str`
#[derive(Debug, PartialEq)]
pub struct ParseFontError;

impl fmt::Display for ParseFontError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for Font {
    /// Returns "Sans Normal"
    fn default() -> Self {
        Self {
            family: String::from("Sans"),
            style: Style::default(),
            weight: Weight::default(),
            stretch: Stretch::default(),
            size: 12288,
        }
    }
}

impl Font {
    /// Get the *family* of the font
    #[must_use]
    pub fn family(&self) -> String {
        String::from(&self.family)
    }

    /// Set the *family* of the font
    pub fn set_family(&mut self, family: String) {
        self.family = family;
    }

    /// Get the *style* of the font
    #[must_use]
    pub fn style(&self) -> Style {
        self.style
    }

    /// Set the *style* or *style* of the font
    pub fn set_style(&mut self, style: Style) {
        self.style = style;
    }

    /// Get the *weight* of the font
    #[must_use]
    pub fn weight(&self) -> Weight {
        self.weight
    }

    /// Set the *weight* of the font
    pub fn set_weight(&mut self, weight: Weight) {
        self.weight = weight;
    }

    /// Get the *size* of the font
    #[must_use]
    pub fn size(&self) -> i32 {
        self.size
    }

    /// Set the *size* of the font
    pub fn set_size(&mut self, size: i32) {
        self.size = size;
    }

    /// Convert to a [`pango::FontDescription`]
    #[must_use]
    pub fn to_pango(&self) -> pango::FontDescription {
        let mut font = pango::FontDescription::new();
        font.set_family(&self.family);
        font.set_style(self.style.to_pango());
        font.set_weight(self.weight.to_pango());
        font.set_stretch(self.stretch.to_pango());
        font.set_size(self.size);
        font
    }

    #[must_use]
    pub fn from_pango(font: pango::FontDescription) -> Self {
        Self {
            family: match font.family() {
                Some(f) => f.to_string(),
                None => "Sans".to_string(),
            },
            style: Style::from_pango(font.style()),
            weight: Weight::from_pango(font.weight()),
            stretch: Stretch::from_pango(font.stretch()),
            size: font.size(),
        }
    }
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Fonts {
    pub pg: Font,
    pub pre: Font,
    pub quote: Font,
    pub h1: Font,
    pub h2: Font,
    pub h3: Font,
}

impl Default for Fonts {
    fn default() -> Self {
        Self {
            pg: Font::default(),
            pre: Font {
                family: String::from("Monospace"),
                style: Style::default(),
                weight: Weight::default(),
                stretch: Stretch::default(),
                size: 12288,
            },
            quote: Font {
                family: String::from("Sans"),
                style: Style::Italic,
                weight: Weight::default(),
                stretch: Stretch::default(),
                size: 12288,
            },
            h1: Font {
                family: String::from("Sans"),
                style: Style::default(),
                weight: Weight::Bold,
                stretch: Stretch::default(),
                size: 18432,
            },
            h2: Font {
                family: String::from("Sans"),
                style: Style::default(),
                weight: Weight::Bold,
                stretch: Stretch::default(),
                size: 16384,
            },
            h3: Font {
                family: String::from("Sans"),
                style: Style::default(),
                weight: Weight::Bold,
                stretch: Stretch::default(),
                size: 14336,
            },
        }
    }
}
