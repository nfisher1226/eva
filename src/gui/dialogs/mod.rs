#![warn(clippy::all, clippy::pedantic)]
use gtk::prelude::*;
use rgba_simple::{FromGdk, ToGdk, RGBA};

use crate::config;
use crate::CONFIG;
use config::{
    Colors, Config, DownloadScheme, Font, Fonts, General, NewPage, ShowTabs, TabPosition,
};

use std::env;

#[derive(Clone)]
pub struct PrefWidgets {
    window: gtk::Dialog,
    homepage: gtk::Entry,
    new_page: gtk::ComboBoxText,
    show_tabs: gtk::ComboBoxText,
    tab_position: gtk::ComboBoxText,
    download_scheme: gtk::ComboBoxText,
    download_location_label: gtk::Label,
    download_location: gtk::Button,
    fg_color: gtk::ColorButton,
    bg_color: gtk::ColorButton,
    pre_fg_color: gtk::ColorButton,
    pre_bg_color: gtk::ColorButton,
    quote_fg_color: gtk::ColorButton,
    quote_bg_color: gtk::ColorButton,
    link_color: gtk::ColorButton,
    hover_color: gtk::ColorButton,
    pg_font: gtk::FontButton,
    pre_font: gtk::FontButton,
    quote_font: gtk::FontButton,
    h1_font: gtk::FontButton,
    h2_font: gtk::FontButton,
    h3_font: gtk::FontButton,
}

#[derive(Clone)]
pub struct Dialogs {
    pub about: gtk::AboutDialog,
    pub save: gtk::FileChooserDialog,
    pub preferences: PrefWidgets,
}

impl Dialogs {
    pub fn init(window: &gtk::ApplicationWindow, builder: &gtk::Builder) -> Self {
        let preferences = Self::init_preferences(window, builder);
        let dl_location = Self::init_dl_location(&preferences.window);
        dl_location.add_button("Accept", gtk::ResponseType::Accept);
        let dlg = dl_location.clone();
        preferences.download_location.connect_clicked(move |_| {
            dlg.show();
        });
        let button = preferences.download_location.clone();
        dl_location.connect_response(move |dlg, res| {
            if res == gtk::ResponseType::Accept {
                if let Some(file) = dlg.file() {
                    if let Some(path) = file.path() {
                        if let Some(path) = path.to_str() {
                            button.set_label(path);
                        }
                    }
                }
            }
            dlg.hide();
        });
        Self {
            about: Self::init_about(window),
            save: Self::init_save(window),
            preferences,
        }
    }

    fn init_about(window: &gtk::ApplicationWindow) -> gtk::AboutDialog {
        gtk::AboutDialog::builder()
            .program_name("Eva")
            .authors(vec!["Nathan Fisher".to_string()])
            .version(env!("CARGO_PKG_VERSION"))
            .license(include_str!(r"../../../LICENSE.md"))
            .wrap_license(true)
            .comments("A browser for the SmolWeb\nBuilt using Rust and Gtk+")
            .logo_icon_name("eva")
            .copyright("©2020 by Nathan Fisher (the JeanG3nie)")
            .website("https://codeberg.org/jeang3nie/eva")
            .transient_for(window)
            .build()
    }

    fn init_save(window: &gtk::ApplicationWindow) -> gtk::FileChooserDialog {
        let dlg = gtk::FileChooserDialog::builder()
            .use_header_bar(1)
            .destroy_with_parent(true)
            .modal(true)
            .transient_for(window)
            .title("Choose location to save file")
            .action(gtk::FileChooserAction::Save)
            .create_folders(true)
            .build();
        dlg.add_button("Accept", gtk::ResponseType::Accept);
        dlg.add_button("Cancel", gtk::ResponseType::Cancel);
        dlg
    }

    fn init_preferences(window: &gtk::ApplicationWindow, builder: &gtk::Builder) -> PrefWidgets {
        let dlg = PrefWidgets::init(builder);
        dlg.load_config();
        dlg.window.set_transient_for(Some(window));
        let dialog = dlg.clone();
        dlg.download_scheme.connect_changed(move |_| {
            if let Some(scheme) = dialog.download_scheme() {
                dialog.toggle_download_location(&scheme);
            }
        });
        dlg
    }

    fn init_dl_location(window: &gtk::Dialog) -> gtk::FileChooserDialog {
        gtk::FileChooserDialog::builder()
            .use_header_bar(1)
            .modal(true)
            .title("Set download location")
            .transient_for(window)
            .action(gtk::FileChooserAction::SelectFolder)
            .create_folders(true)
            .build()
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
            download_scheme: builder
                .object("download_scheme")
                .expect("Error getting 'download_scheme'"),
            download_location_label: builder
                .object("download_location_label")
                .expect("Error getting 'download_location_label'"),
            download_location: builder
                .object("download_location")
                .expect("Error getting 'download_location'"),
            tab_position: builder
                .object("tab_position")
                .expect("Error getting 'tab_position'"),
            fg_color: builder
                .object("fg_color")
                .expect("Error getting 'fg_color'"),
            bg_color: builder
                .object("bg_color")
                .expect("Error getting 'bg_color'"),
            pre_fg_color: builder
                .object("pre_fg_color")
                .expect("Error getting 'pre_fg_color'"),
            pre_bg_color: builder
                .object("pre_bg_color")
                .expect("Error getting 'pre_bg_color'"),
            quote_fg_color: builder
                .object("quote_fg_color")
                .expect("Error getting 'quote_fg_color'"),
            quote_bg_color: builder
                .object("quote_bg_color")
                .expect("Error getting 'quote_bg_color'"),
            link_color: builder
                .object("link_color")
                .expect("Error getting 'link_color'"),
            hover_color: builder
                .object("hover_color")
                .expect("Error getting 'hover_color'"),
            pg_font: builder.object("pg_font").expect("Error getting 'pg_font'"),
            pre_font: builder
                .object("pre_font")
                .expect("Error getting 'pre_font'"),
            quote_font: builder
                .object("quote_font")
                .expect("Error getting 'quote_font'"),
            h1_font: builder.object("h1_font").expect("Error getting 'h1_font'"),
            h2_font: builder.object("h2_font").expect("Error getting 'h2_font'"),
            h3_font: builder.object("h3_font").expect("Error getting 'h3_font'"),
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

    pub fn set_new_page(&self, page: &NewPage) {
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

    pub fn set_show_tabs(&self, show: &ShowTabs) {
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

    pub fn set_tab_position(&self, pos: &TabPosition) {
        self.tab_position.set_active_id(match pos {
            TabPosition::Top => Some("top"),
            TabPosition::Bottom => Some("bottom"),
            TabPosition::Left => Some("left"),
            TabPosition::Right => Some("right"),
        });
    }

    pub fn download_scheme(&self) -> Option<DownloadScheme> {
        if let Some(scm) = self.download_scheme.active_id() {
            match scm.as_str() {
                "auto" => Some(DownloadScheme::Auto),
                "ask" => Some(DownloadScheme::Ask),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn set_download_scheme(&self, scheme: &DownloadScheme) {
        match scheme {
            DownloadScheme::Ask => {
                self.download_scheme.set_active_id(Some("ask"));
            }
            DownloadScheme::Auto => {
                self.download_scheme.set_active_id(Some("auto"));
            }
        }
        self.toggle_download_location(scheme);
    }

    fn toggle_download_location(&self, scheme: &DownloadScheme) {
        match scheme {
            DownloadScheme::Ask => {
                self.download_location_label.hide();
                self.download_location.hide();
            }
            DownloadScheme::Auto => {
                self.download_location_label.show();
                self.download_location.show();
            }
        }
    }

    fn download_location(&self) -> Option<String> {
        if let Some(loc) = self.download_location.label() {
            Some(loc.to_string())
        } else {
            None
        }
    }

    fn set_download_location(&self, location: &str) {
        self.download_location.set_label(location);
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
            download_scheme: match self.download_scheme() {
                Some(sc) => sc,
                None => return None,
            },
            download_location: self.download_location(),
        })
    }

    pub fn set_general(&self, gen: &General) {
        self.set_homepage(&gen.homepage);
        self.set_new_page(&gen.new_page);
        self.set_show_tabs(&gen.show_tabs);
        self.set_tab_position(&gen.tab_position);
        self.set_download_scheme(&gen.download_scheme);
        self.set_download_location(match &gen.download_location {
            Some(loc) => &loc,
            None => "~/Downloads",
        });
    }

    pub fn fg_color(&self) -> RGBA<u8> {
        RGBA::from_gdk(self.fg_color.rgba())
    }

    pub fn set_fg_color(&self, color: &RGBA<u8>) {
        self.fg_color.set_rgba(&color.to_gdk());
    }

    pub fn bg_color(&self) -> RGBA<u8> {
        RGBA::from_gdk(self.bg_color.rgba())
    }

    pub fn set_bg_color(&self, color: &RGBA<u8>) {
        self.bg_color.set_rgba(&color.to_gdk());
    }

    pub fn pre_fg_color(&self) -> RGBA<u8> {
        RGBA::from_gdk(self.pre_fg_color.rgba())
    }

    pub fn set_pre_fg_color(&self, color: &RGBA<u8>) {
        self.pre_fg_color.set_rgba(&color.to_gdk());
    }

    pub fn pre_bg_color(&self) -> RGBA<u8> {
        RGBA::from_gdk(self.pre_fg_color.rgba())
    }

    pub fn set_pre_bg_color(&self, color: &RGBA<u8>) {
        self.pre_bg_color.set_rgba(&color.to_gdk());
    }

    pub fn quote_fg_color(&self) -> RGBA<u8> {
        RGBA::from_gdk(self.quote_fg_color.rgba())
    }

    pub fn set_quote_fg_color(&self, color: &RGBA<u8>) {
        self.quote_fg_color.set_rgba(&color.to_gdk())
    }

    pub fn quote_bg_color(&self) -> RGBA<u8> {
        RGBA::from_gdk(self.quote_bg_color.rgba())
    }

    pub fn set_quote_bg_color(&self, color: &RGBA<u8>) {
        self.quote_bg_color.set_rgba(&color.to_gdk());
    }

    pub fn link_color(&self) -> RGBA<u8> {
        RGBA::from_gdk(self.link_color.rgba())
    }

    pub fn set_link_color(&self, color: &RGBA<u8>) {
        self.link_color.set_rgba(&color.to_gdk());
    }

    pub fn hover_color(&self) -> RGBA<u8> {
        RGBA::from_gdk(self.hover_color.rgba())
    }

    pub fn set_hover_color(&self, color: &RGBA<u8>) {
        self.hover_color.set_rgba(&color.to_gdk());
    }

    pub fn colors(&self) -> Colors {
        Colors {
            fg: self.fg_color(),
            bg: self.bg_color(),
            pre_fg: self.pre_fg_color(),
            pre_bg: self.pre_bg_color(),
            quote_fg: self.quote_fg_color(),
            quote_bg: self.quote_bg_color(),
            link: self.link_color(),
            hover: self.hover_color(),
        }
    }

    pub fn set_colors(&self, colors: &Colors) {
        self.set_fg_color(&colors.fg);
        self.set_bg_color(&colors.bg);
        self.set_pre_fg_color(&colors.pre_fg);
        self.set_pre_bg_color(&colors.pre_bg);
        self.set_quote_fg_color(&colors.quote_fg);
        self.set_quote_bg_color(&colors.quote_bg);
        self.set_link_color(&colors.link);
        self.set_hover_color(&colors.hover);
    }

    pub fn pg_font(&self) -> Option<Font> {
        self.pg_font.font_desc().map(Font::from_pango)
    }

    pub fn set_pg_font(&self, font: &Font) {
        self.pg_font.set_font_desc(&font.to_pango());
    }

    pub fn pre_font(&self) -> Option<Font> {
        self.pre_font.font_desc().map(Font::from_pango)
    }

    pub fn set_pre_font(&self, font: &Font) {
        self.pre_font.set_font_desc(&font.to_pango());
    }

    pub fn quote_font(&self) -> Option<Font> {
        self.quote_font.font_desc().map(Font::from_pango)
    }

    pub fn set_quote_font(&self, font: &Font) {
        self.quote_font.set_font_desc(&font.to_pango());
    }

    pub fn h1_font(&self) -> Option<Font> {
        self.h1_font.font_desc().map(Font::from_pango)
    }

    pub fn set_h1_font(&self, font: &Font) {
        self.h1_font.set_font_desc(&font.to_pango());
    }

    pub fn h2_font(&self) -> Option<Font> {
        self.h2_font.font_desc().map(Font::from_pango)
    }

    pub fn set_h2_font(&self, font: &Font) {
        self.h2_font.set_font_desc(&font.to_pango());
    }

    pub fn h3_font(&self) -> Option<Font> {
        self.h3_font.font_desc().map(Font::from_pango)
    }

    pub fn set_h3_font(&self, font: &Font) {
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
            quote: match self.quote_font() {
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

    pub fn set_fonts(&self, fonts: &Fonts) {
        self.set_pg_font(&fonts.pg);
        self.set_pre_font(&fonts.pre);
        self.set_quote_font(&fonts.quote);
        self.set_h1_font(&fonts.h1);
        self.set_h2_font(&fonts.h2);
        self.set_h3_font(&fonts.h3);
    }

    pub fn config(&self) -> Option<Config> {
        Some(Config {
            general: match self.general() {
                Some(g) => g,
                None => return None,
            },
            colors: self.colors(),
            fonts: match self.fonts() {
                Some(f) => f,
                None => return None,
            },
        })
    }

    pub fn load_config(&self) {
        let cfg = CONFIG.lock().unwrap();
        self.set_general(&cfg.general);
        self.set_colors(&cfg.colors);
        self.set_fonts(&cfg.fonts);
    }

    pub fn show(&self) {
        self.window.show();
    }
}
