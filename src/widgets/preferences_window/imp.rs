use adw::{
    gtk::{
        self,
        gdk::RGBA,
        gio::Settings,
        glib::{self, BindingFlags, prelude::ObjectExt, subclass::InitializingObject},
        prelude::SettingsExtManual,
        CompositeTemplate,
    },
    subclass::prelude::*,
};

#[derive(CompositeTemplate, Default)]
#[template(file = "preferences_window.ui")]
pub struct PreferencesWindow {
    #[template_child]
    pub homepage: TemplateChild<adw::EntryRow>,
    #[template_child]
    pub new_page_type: TemplateChild<adw::ComboRow>,
    #[template_child]
    pub downloads: TemplateChild<adw::ComboRow>,
    #[template_child]
    pub download_location_row: TemplateChild<adw::ActionRow>,
    #[template_child]
    pub download_location: TemplateChild<gtk::Button>,
    #[template_child]
    pub pg_font: TemplateChild<gtk::FontButton>,
    #[template_child]
    pub h1_font: TemplateChild<gtk::FontButton>,
    #[template_child]
    pub h2_font: TemplateChild<gtk::FontButton>,
    #[template_child]
    pub h3_font: TemplateChild<gtk::FontButton>,
    #[template_child]
    pub pre_font: TemplateChild<gtk::FontButton>,
    #[template_child]
    pub quote_font: TemplateChild<gtk::FontButton>,
    #[template_child]
    pub fg_color: TemplateChild<gtk::ColorButton>,
    #[template_child]
    pub bg_color: TemplateChild<gtk::ColorButton>,
    #[template_child]
    pub pre_fg_color: TemplateChild<gtk::ColorButton>,
    #[template_child]
    pub pre_bg_color: TemplateChild<gtk::ColorButton>,
    #[template_child]
    pub quote_fg_color: TemplateChild<gtk::ColorButton>,
    #[template_child]
    pub quote_bg_color: TemplateChild<gtk::ColorButton>,
    #[template_child]
    pub link_color: TemplateChild<gtk::ColorButton>,
    #[template_child]
    pub hover_color: TemplateChild<gtk::ColorButton>,
}

#[glib::object_subclass]
impl ObjectSubclass for PreferencesWindow {
    const NAME: &'static str = "PreferencesWindow";
    type Type = super::PreferencesWindow;
    type ParentType = adw::PreferencesWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for PreferencesWindow {
    fn constructed(&self) {
        self.parent_constructed();
        self.downloads
            .get()
            .bind_property("selected", &self.download_location_row.get(), "visible")
            .flags(BindingFlags::SYNC_CREATE)
            .transform_to(|_, num: u32| match num {
                0 => Some(false),
                1 => Some(true),
                _ => None,
            })
            .build();
    }
}

impl PreferencesWindow {
    pub fn bind_settings(&self, settings: &Settings) {
        settings
            .bind("homepage", &self.homepage.get(), "text")
            .build();
        settings
            .bind("new-page", &self.new_page_type.get(), "selected")
            .build();
        settings
            .bind("paragraph-font", &self.pg_font.get(), "font")
            .build();
        settings
            .bind("h1-font", &self.h1_font.get(), "font")
            .build();
        settings
            .bind("h2-font", &self.h2_font.get(), "font")
            .build();
        settings
            .bind("h3-font", &self.h3_font.get(), "font")
            .build();
        settings
            .bind("preformatted-font", &self.pre_font.get(), "font")
            .build();
        settings
            .bind("quote-font", &self.quote_font.get(), "font")
            .build();
        settings
            .bind("fg-color", &self.fg_color.get(), "rgba")
            .mapping(|variant, _vtype| {
                variant
                    .get::<String>()
                    .expect("The value needs to be of type `String`")
                    .parse::<RGBA>()
                    .map(|x| x.into())
                    .ok()
            })
            .set_mapping(|value, _vtype| {
                let color = value
                    .get::<RGBA>()
                    .expect("The value needs to be of type gdk::RGBA")
                    .to_string();
                Some(color.into())
            })
            .build();
        settings
            .bind("bg-color", &self.bg_color.get(), "rgba")
            .mapping(|variant, _vtype| {
                variant
                    .get::<String>()
                    .expect("The value needs to be of type `String`")
                    .parse::<RGBA>()
                    .map(|x| x.into())
                    .ok()
            })
            .set_mapping(|value, _vtype| {
                let color = value
                    .get::<RGBA>()
                    .expect("The value needs to be of type gdk::RGBA")
                    .to_string();
                Some(color.into())
            })
            .build();
        settings
            .bind("pre-fg-color", &self.pre_fg_color.get(), "rgba")
            .mapping(|variant, _vtype| {
                variant
                    .get::<String>()
                    .expect("The value needs to be of type `String`")
                    .parse::<RGBA>()
                    .map(|x| x.into())
                    .ok()
            })
            .set_mapping(|value, _vtype| {
                let color = value
                    .get::<RGBA>()
                    .expect("The value needs to be of type gdk::RGBA")
                    .to_string();
                Some(color.into())
            })
            .build();
        settings
            .bind("pre-bg-color", &self.pre_bg_color.get(), "rgba")
            .mapping(|variant, _vtype| {
                variant
                    .get::<String>()
                    .expect("The value needs to be of type `String`")
                    .parse::<RGBA>()
                    .map(|x| x.into())
                    .ok()
            })
            .set_mapping(|value, _vtype| {
                let color = value
                    .get::<RGBA>()
                    .expect("The value needs to be of type gdk::RGBA")
                    .to_string();
                Some(color.into())
            })
            .build();
        settings
            .bind("quote-fg-color", &self.quote_fg_color.get(), "rgba")
            .mapping(|variant, _vtype| {
                variant
                    .get::<String>()
                    .expect("The value needs to be of type `String`")
                    .parse::<RGBA>()
                    .map(|x| x.into())
                    .ok()
            })
            .set_mapping(|value, _vtype| {
                let color = value
                    .get::<RGBA>()
                    .expect("The value needs to be of type gdk::RGBA")
                    .to_string();
                Some(color.into())
            })
            .build();
        settings
            .bind("quote-bg-color", &self.quote_bg_color.get(), "rgba")
            .mapping(|variant, _vtype| {
                variant
                    .get::<String>()
                    .expect("The value needs to be of type `String`")
                    .parse::<RGBA>()
                    .map(|x| x.into())
                    .ok()
            })
            .set_mapping(|value, _vtype| {
                let color = value
                    .get::<RGBA>()
                    .expect("The value needs to be of type gdk::RGBA")
                    .to_string();
                Some(color.into())
            })
            .build();
        settings
            .bind("link-color", &self.link_color.get(), "rgba")
            .mapping(|variant, _vtype| {
                variant
                    .get::<String>()
                    .expect("The value needs to be of type `String`")
                    .parse::<RGBA>()
                    .map(|x| x.into())
                    .ok()
            })
            .set_mapping(|value, _vtype| {
                let color = value
                    .get::<RGBA>()
                    .expect("The value needs to be of type gdk::RGBA")
                    .to_string();
                Some(color.into())
            })
            .build();
        settings
            .bind("hover-color", &self.hover_color.get(), "rgba")
            .mapping(|variant, _vtype| {
                variant
                    .get::<String>()
                    .expect("The value needs to be of type `String`")
                    .parse::<RGBA>()
                    .map(|x| x.into())
                    .ok()
            })
            .set_mapping(|value, _vtype| {
                let color = value
                    .get::<RGBA>()
                    .expect("The value needs to be of type gdk::RGBA")
                    .to_string();
                Some(color.into())
            })
            .build();
    }
}

impl PreferencesWindowImpl for PreferencesWindow {}
impl AdwWindowImpl for PreferencesWindow {}
impl WindowImpl for PreferencesWindow {}
impl WidgetImpl for PreferencesWindow {}