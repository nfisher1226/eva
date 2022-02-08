#![warn(clippy::all, clippy::pedantic)]
use gtk::pango::FontDescription;
use gtk::prelude::*;
use gtk::ResponseType;

use crate::CONFIG;

use std::env;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone)]
pub struct PrefWidgets {
    window: gtk::Dialog,
    homepage: gtk::Entry,
    new_tab_variant: gtk::ComboBoxText,
    show_tab_bar: gtk::ComboBoxText,
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
        dlg.new_tab_variant.set_active_id(Some("home"));
        dlg.show_tab_bar.set_active_id(Some("always"));
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
            new_tab_variant: builder
                .object("new_page_variant")
                .expect("Error getting 'new_page_variant'"),
            show_tab_bar: builder
                .object("show_tab_bar")
                .expect("Error getting 'show_tab_bar'"),
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

    pub fn show(&self) {
        self.window.show();
    }

    pub fn window(&self) -> gtk::Dialog {
        self.window.clone()
    }
}
