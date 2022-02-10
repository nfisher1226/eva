#![warn(clippy::all, clippy::pedantic)]
use gtk::pango::FontDescription;
use gtk::prelude::*;
use gtk::ResponseType;
use rgba_simple::{Color, ColorError, Convert};

use crate::CONFIG;
use crate::config::{Colors, Config, Font, Fonts, General, NewPage, ShowTabs, TabPosition};

use std::env;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone)]
pub struct PrefWidgets {
    window: gtk::Dialog,
    homepage: gtk::Entry,
    new_page: gtk::ComboBoxText,
    show_tabs: gtk::ComboBoxText,
    tab_position: gtk::ComboBoxText,
    fg_color: gtk::ColorButton,
    bg_color: gtk::ColorButton,
    quote_fg_color: gtk::ColorButton,
    quote_bg_color: gtk::ColorButton,
    link_color: gtk::ColorButton,
    pg_font: gtk::FontButton,
    pre_font: gtk::FontButton,
    h1_font: gtk::FontButton,
    h2_font: gtk::FontButton,
    h3_font: gtk::FontButton,
}

#[derive(Clone)]
pub struct Dialogs {
    pub about: gtk::AboutDialog,
    pub preferences: PrefWidgets,
}

impl Dialogs {
    pub fn init(window: &gtk::ApplicationWindow, builder: &gtk::Builder) -> Self {
        let prefs = Self::init_preferences(window, builder);

        let dialogs = Self {
            about: Self::init_about(window),
            preferences: prefs,
        };
        dialogs
    }

    fn init_about(window: &gtk::ApplicationWindow) -> gtk::AboutDialog {
        let dlg = gtk::AboutDialog::builder()
            .program_name("Eva")
            .authors(vec!("Nathan Fisher".to_string()))
            .version(env!("CARGO_PKG_VERSION"))
            .license(include_str!(r"../../../LICENSE.md"))
            .wrap_license(true)
            .comments("A browser for the SmolWeb\nBuilt using Rust and Gtk+")
            .logo_icon_name("eva")
            .copyright("©2020 by Nathan Fisher (the JeanG3nie)")
            .website("https://codeberg.org/jeang3nie/eva")
            .transient_for(window)
            .build();
        dlg
    }

    fn init_preferences(window: &gtk::ApplicationWindow, builder: &gtk::Builder) -> PrefWidgets {
        let dlg = PrefWidgets::init(builder);
        match dlg.load_config() {
            Ok(_) => {},
            Err(e) => eprintln!("Error loading config: {}", e),
        }
        dlg.window.set_transient_for(Some(window));
        let dialog = dlg.clone();
        dlg.window.connect_response(move |dlg,res| {
            if res == ResponseType::Accept {
                if let Some(cfg) = dialog.config() {
                    *CONFIG.lock().unwrap() = cfg;
                } else {
                    match dialog.load_config() {
                        Ok(_) => {},
                        Err(e) => eprintln!("Error loading config: {}", e),
                    }
                }
            }
            dlg.hide();
        });

        dlg
    }
}

impl PrefWidgets {
    /// Returns a struct of pointers to the widgets that contain state
    fn init(builder: &gtk::Builder) -> Self {
        let ui_src = include_str!("prefs.ui");
        builder.add_from_string(ui_src).unwrap();
        PrefWidgets {
            window: builder
                .object("prefs_window")
                .expect("Error getting 'prefs_window'"),
            homepage: builder
                .object("homepage")
                .expect("Error getting 'homepage'"),
            new_page: builder
                .object("new_page")
                .expect("Error getting 'new_page'"),
            show_tabs: builder
                .object("show_tabs")
                .expect("Error getting 'show_tabs'"),
            tab_position: builder
                .object("tab_position")
                .expect("Error getting 'tab_position'"),
            fg_color: builder
                .object("fg_color")
                .expect("Error getting 'fg_color'"),
            bg_color: builder
                .object("bg_color")
                .expect("Error getting 'bg_color'"),
            quote_fg_color: builder
                .object("quote_fg_color")
                .expect("Error getting 'quote_fg_color'"),
            quote_bg_color: builder
                .object("quote_bg_color")
                .expect("Error getting 'quote_bg_color'"),
            link_color: builder
                .object("link_color")
                .expect("Error getting 'link_color'"),
            pg_font: builder
                .object("pg_font")
                .expect("Error getting 'pg_font'"),
            pre_font: builder
                .object("pre_font")
                .expect("Error getting 'pre_font'"),
            h1_font: builder
                .object("h1_font")
                .expect("Error getting 'h1_font'"),
            h2_font: builder
                .object("h2_font")
                .expect("Error getting 'h2_font'"),
            h3_font: builder
                .object("h3_font")
                .expect("Error getting 'h3_font'"),
        }
    }

    pub fn window(&self) -> gtk::Dialog {
        self.window.clone()
    }

    pub fn homepage(&self) -> String {
        self.homepage.buffer().text()
    }

    pub fn set_homepage(&self, page: &str) {
        self.homepage.buffer().set_text(page);
    }

    pub fn new_page(&self) -> Option<NewPage> {
        if let Some(id) = self.new_page.active_id() {
            match id.as_str() {
                "home" => Some(NewPage::Home),
                "blank" => Some(NewPage::Blank),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn set_new_page(&self, page: NewPage) {
        self.new_page.set_active_id(match page {
            NewPage::Home => Some("home"),
            NewPage::Blank => Some("blank"),
        });
    }

    pub fn show_tabs(&self) -> Option<ShowTabs> {
        if let Some(id) = self.show_tabs.active_id() {
            match id.as_str() {
                "always" => Some(ShowTabs::Always),
                "multiple" => Some(ShowTabs::Multiple),
                "never" => Some(ShowTabs::Never),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn set_show_tabs(&self, show: ShowTabs) {
        self.show_tabs.set_active_id(match show {
            ShowTabs::Always => Some("always"),
            ShowTabs::Multiple => Some("multiple"),
            ShowTabs::Never => Some("never"),
        });
    }

    pub fn tab_position(&self) -> Option<TabPosition> {
        if let Some(pos) = self.tab_position.active_id() {
            match pos.as_str() {
                "top" => Some(TabPosition::Top),
                "bottom" => Some(TabPosition::Bottom),
                "left" => Some(TabPosition::Left),
                "right" => Some(TabPosition::Right),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn set_tab_position(&self, pos: TabPosition) {
        self.tab_position.set_active_id(match pos {
            TabPosition::Top => Some("top"),
            TabPosition::Bottom => Some("bottom"),
            TabPosition::Left => Some("left"),
            TabPosition::Right => Some("right"),
        });
    }

    pub fn general(&self) -> Option<General> {
        Some(General {
            homepage: self.homepage(),
            new_page: match self.new_page() {
                Some(np) => np,
                None => return None,
            },
            show_tabs: match self.show_tabs() {
                Some(st) => st,
                None => return None,
            },
            tab_position: match self.tab_position() {
                Some(tp) => tp,
                None => return None,
            },
        })
    }

    pub fn set_general(&self, gen: General) {
        self.set_homepage(&gen.homepage);
        self.set_new_page(gen.new_page);
        self.set_show_tabs(gen.show_tabs);
        self.set_tab_position(gen.tab_position);
    }

    pub fn fg_color(&self) -> Result<Color, ColorError> {
        match self.fg_color.rgba().to_reduced_rgba() {
            Ok(c) => Ok(Color::Reduced(c)),
            Err(e) => Err(e),
        }
    }

    pub fn set_fg_color(&self, color: Color) -> Result<(), ColorError> {
        match color.to_gdk() {
            Ok(c) => {
                self.fg_color.set_rgba(&c);
                Ok(())
            },
            Err(e) => Err(e),
        }
    }

    pub fn bg_color(&self) -> Result<Color, ColorError> {
        match self.bg_color.rgba().to_reduced_rgba() {
            Ok(c) => Ok(Color::Reduced(c)),
            Err(e) => Err(e),
        }
    }

    pub fn set_bg_color(&self, color: Color) -> Result<(), ColorError> {
        match color.to_gdk() {
            Ok(c) => {
                self.bg_color.set_rgba(&c);
                Ok(())
            },
            Err(e) => Err(e),
        }
    }

    pub fn quote_fg_color(&self) -> Result<Color, ColorError> {
        match self.quote_fg_color.rgba().to_reduced_rgba() {
            Ok(c) => Ok(Color::Reduced(c)),
            Err(e) => Err(e),
        }
    }

    pub fn set_quote_fg_color(&self, color: Color) -> Result<(), ColorError> {
        match color.to_gdk() {
            Ok(c) => {
                self.quote_fg_color.set_rgba(&c);
                Ok(())
            },
            Err(e) => Err(e),
        }
    }

    pub fn quote_bg_color(&self) -> Result<Color, ColorError> {
        match self.quote_bg_color.rgba().to_reduced_rgba() {
            Ok(c) => Ok(Color::Reduced(c)),
            Err(e) => Err(e),
        }
    }

    pub fn set_quote_bg_color(&self, color: Color) -> Result<(), ColorError> {
        match color.to_gdk() {
            Ok(c) => {
                self.quote_bg_color.set_rgba(&c);
                Ok(())
            },
            Err(e) => Err(e),
        }
    }

    pub fn link_color(&self) -> Result<Color, ColorError> {
        match self.link_color.rgba().to_reduced_rgba() {
            Ok(c) => Ok(Color::Reduced(c)),
            Err(e) => Err(e),
        }
    }

    pub fn set_link_color(&self, color: Color) -> Result<(), ColorError> {
        match color.to_gdk() {
            Ok(c) => {
                self.link_color.set_rgba(&c);
                Ok(())
            },
            Err(e) => Err(e),
        }
    }

    pub fn colors(&self) -> Result<Colors, ColorError> {
        Ok(Colors {
            fg: self.fg_color()?,
            bg: self.bg_color()?,
            quote_fg: self.quote_fg_color()?,
            quote_bg: self.quote_bg_color()?,
            link: self.link_color()?,
        })
    }

    pub fn set_colors(&self, colors: Colors) -> Result<(), ColorError> {
        self.set_fg_color(colors.fg)?;
        self.set_bg_color(colors.bg)?;
        self.set_quote_fg_color(colors.quote_fg)?;
        self.set_quote_bg_color(colors.quote_bg)?;
        self.set_link_color(colors.link)?;
        Ok(())
    }

    pub fn pg_font(&self) -> Option<Font> {
        match self.pg_font.font_desc() {
            Some(font) => Some(Font::from_pango(font)),
            None => None,
        }
    }

    pub fn set_pg_font(&self, font: Font) {
        self.pg_font.set_font_desc(&font.to_pango());
    }

    pub fn pre_font(&self) -> Option<Font> {
        match self.pre_font.font_desc() {
            Some(font) => Some(Font::from_pango(font)),
            None => None,
        }
    }

    pub fn set_pre_font(&self, font: Font) {
        self.pre_font.set_font_desc(&font.to_pango());
    }

    pub fn h1_font(&self) -> Option<Font> {
        match self.h1_font.font_desc() {
            Some(font) => Some(Font::from_pango(font)),
            None => None,
        }
    }

    pub fn set_h1_font(&self, font: Font) {
        self.h1_font.set_font_desc(&font.to_pango());
    }

    pub fn h2_font(&self) -> Option<Font> {
        match self.h2_font.font_desc() {
            Some(font) => Some(Font::from_pango(font)),
            None => None,
        }
    }

    pub fn set_h2_font(&self, font: Font) {
        self.h2_font.set_font_desc(&font.to_pango());
    }

    pub fn h3_font(&self) -> Option<Font> {
        match self.h3_font.font_desc() {
            Some(font) => Some(Font::from_pango(font)),
            None => None,
        }
    }

    pub fn set_h3_font(&self, font: Font) {
        self.h3_font.set_font_desc(&font.to_pango());
    }

    pub fn fonts(&self) -> Option<Fonts> {
        Some(Fonts {
            pg: match self.pg_font() {
                Some(f) => f,
                None => return None,
            },
            pre: match self.pre_font() {
                Some(f) => f,
                None => return None,
            },
            h1: match self.h1_font() {
                Some(f) => f,
                None => return None,
            },
            h2: match self.h2_font() {
                Some(f) => f,
                None => return None,
            },
            h3: match self.h3_font() {
                Some(f) => f,
                None => return None,
            },
        })
    }

    pub fn set_fonts(&self, fonts: Fonts) {
        self.set_pg_font(fonts.pg);
        self.set_pre_font(fonts.pre);
        self.set_h1_font(fonts.h1);
        self.set_h1_font(fonts.h2);
        self.set_h1_font(fonts.h3);
    }

    pub fn config(&self) -> Option<Config> {
        Some(Config {
            general: match self.general() {
                Some(g) => g,
                None => return None,
            },
            colors: match self.colors() {
                Ok(c) => c,
                Err(_) => return None,
            },
            fonts: match self.fonts() {
                Some(f) => f,
                None => return None,
            },
        })
    }

    fn load_config(&self) -> Result<(), ColorError> {
        let cfg = CONFIG.lock().unwrap();
        self.set_general(cfg.general.clone());
        self.set_colors(cfg.colors.clone())?;
        self.set_fonts(cfg.fonts.clone());
        Ok(())
    }

    pub fn show(&self) {
        self.window.show();
    }
}
