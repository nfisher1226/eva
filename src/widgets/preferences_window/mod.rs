mod imp;

use adw::gtk::{
    gio::Settings,
    glib::{self, Object},
    subclass::prelude::*,
};

glib::wrapper! {
    pub struct PreferencesWindow(ObjectSubclass<imp::PreferencesWindow>)
        @extends adw::PreferencesWindow, adw::Window, gtk::Window, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
            gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl PreferencesWindow {
    pub fn new(settings: &Settings) -> Self {
        let win: Self = Object::builder().build();
        win.imp().bind_settings(settings);
        win
    }
}
