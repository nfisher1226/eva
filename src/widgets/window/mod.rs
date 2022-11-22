mod imp;

use {
    crate::Tab,
    adw::gtk::{
        gio,
        glib::{self, Object},
    },
};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, adw::Window,
            gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible,
            gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root,
            gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &crate::Application) -> Self {
        Object::new(&[("application", app)])
    }

    pub fn add_tab(&self, address: Option<&mut str>) {
        let tab = Tab::new();
    }
}
