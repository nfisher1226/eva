#![warn(clippy::all, clippy::pedantic)]
mod prefs;
pub use prefs::Prefs;
use {gtk::prelude::*, std::env};

#[derive(Clone)]
pub struct Dialogs {
    pub about: gtk::AboutDialog,
    pub save: gtk::FileChooserDialog,
    pub preferences: Prefs,
}

impl Dialogs {
    pub fn init(window: &gtk::ApplicationWindow) -> Self {
        let preferences = Self::init_preferences(window);
        let about = Self::init_about(window);
        about.add_credit_section(
            "Gtk4-rs written by",
            &[
                "Guillaume Gomez",
                "Sebastian Dröge",
                "Bilal Elmoussaoui",
                "Xiang Fan",
                "and others",
            ],
        );
        about.add_credit_section("Serde written by", &["David Tolnay", "and others"]);
        about.add_credit_section("Chrono written by", &["Kang Seonghoon", "and others"]);
        about.add_credit_section("Url written by", &["The Servo project"]);
        about.add_credit_section("Lazy-static by", &["Marvin Löbel", "and others"]);
        about.add_credit_section("Fastrand by", &["Taiki Endo", "and others"]);
        about.add_credit_section("Urlencoding by", &["Kornel", "Bertram Truong"]);
        about.add_credit_section("Mime2ext by", &["Jan Verbeek", "and others"]);
        about.add_credit_section("Gemview by", &["Nathan Fisher"]);
        about.add_credit_section("Mime-open by", &["Nathan Fisher"]);
        about.add_credit_section("Rgba-simple by", &["Nathan Fisher"]);
        Self {
            about,
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

    fn init_preferences(window: &gtk::ApplicationWindow) -> Prefs {
        let dlg = Prefs::new();
        dlg.load_config();
        dlg.set_transient_for(Some(window));
        dlg
    }
}
