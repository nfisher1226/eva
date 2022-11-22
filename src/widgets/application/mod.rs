mod imp;

use adw::gtk::{
    gio,
    glib::{self, Object},
};

glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>)
        @extends adw::Application, gtk::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Application {
    pub fn new() -> Self {
        Object::new(&[])
    }
}
