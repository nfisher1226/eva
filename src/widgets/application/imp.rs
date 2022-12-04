use {
    adw::{
        gtk::glib,
        prelude::*,
        subclass::prelude::*,
    },
    crate::prelude::Window,
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

impl ApplicationImpl for Application {
    fn activate(&self) {
        let instance = self.instance();
        if instance.windows().is_empty() {
            let window = Window::new(&instance);
            instance.add_actions(&window);
            let mut addr = "gemini://gemini.circumlunar.space".to_string();
            window.open_tab(Some(&mut addr));
            window.present();
        }
    }
}

impl AdwApplicationImpl for Application {}
impl GtkApplicationImpl for Application {}
