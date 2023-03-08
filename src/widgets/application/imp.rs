use {
    crate::prelude::Window,
    adw::{
        gtk::{
            gio::{PropertyAction, Settings, SettingsBindFlags},
            glib,
        },
        prelude::*,
        subclass::prelude::*,
    },
};

pub struct Application {
    pub settings: Settings,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            settings: Settings::new("org.codeberg.jeang3nie.eva"),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for Application {
    const NAME: &'static str = "Application";
    type Type = super::Application;
    type ParentType = adw::Application;
}

impl ObjectImpl for Application {
    fn constructed(&self) {
        self.parent_constructed();
        let instance = self.obj();
        let set_property_action =
            PropertyAction::new("set-theme", &instance.style_manager(), "color-scheme");
        instance.add_action(&set_property_action);
        self.settings
            .bind("theme", &instance.style_manager(), "color-scheme")
            .flags(SettingsBindFlags::DEFAULT)
            .build();
    }
}

impl ApplicationImpl for Application {
    fn activate(&self) {
        let instance = self.obj();
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
