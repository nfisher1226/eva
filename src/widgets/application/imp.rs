use adw::{
    gtk::{
        glib::{self, subclass::InitializingObject},
        subclass::prelude::*,
    },
    subclass::prelude::*,
};

#[derive(Default)]
pub struct Application {}

#[glib::object_subclass]
impl ObjectSubclass for Application {
    const NAME: &'static str = "Application";
    type Type = super::Application;
    type ParentType = adw::Application;

    //fn class_init(klass: &mut Self::Class) {}

    //fn instance_init(obj: &InitializingObject<Self>) {}
}

impl ObjectImpl for Application {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl AdwApplicationImpl for Application {}
impl GtkApplicationImpl for Application {}
impl ApplicationImpl for Application {}
