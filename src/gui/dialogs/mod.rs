#![warn(clippy::all, clippy::pedantic)]
use gtk::pango::FontDescription;
use gtk::prelude::*;
use gtk::ResponseType;
use rgba_simple::{Color, ColorError, Convert};

use crate::CONFIG;
use crate::config::{Colors, Config, General, NewPage, ShowTabs, TabPosition};

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
        dlg.window.set_transient_for(Some(window));
        dlg.new_page.set_active_id(Some("home"));
        dlg.show_tabs.set_active_id(Some("always"));
        dlg.tab_position.set_active_id(Some("top"));
        dlg.window.connect_response(move |dlg,res| {
            if res == ResponseType::Accept {
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

    fn load_config(&self) -> Result<(), ColorError> {
        let cfg = CONFIG.lock().unwrap();
        self.set_general(cfg.general.clone());
        self.set_colors(cfg.colors.clone())?;
        Ok(())
    }

    pub fn show(&self) {
        self.window.show();
    }
}
