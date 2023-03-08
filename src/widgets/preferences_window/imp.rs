use adw::{
    gtk::{
        self,
        gio::Settings,
        glib::{self, BindingFlags, prelude::ObjectExt, subclass::InitializingObject},
        pango::FontDescription,
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
    pub download_location_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub pg_font: TemplateChild<gtk::FontDialogButton>,
    #[template_child]
    pub h1_font: TemplateChild<gtk::FontDialogButton>,
    #[template_child]
    pub h2_font: TemplateChild<gtk::FontDialogButton>,
    #[template_child]
    pub h3_font: TemplateChild<gtk::FontDialogButton>,
    #[template_child]
    pub pre_font: TemplateChild<gtk::FontDialogButton>,
    #[template_child]
    pub quote_font: TemplateChild<gtk::FontDialogButton>,
    #[template_child]
    pub fg_color: TemplateChild<gtk::ColorDialogButton>,
    #[template_child]
    pub bg_color: TemplateChild<gtk::ColorDialogButton>,
    #[template_child]
    pub pre_fg_color: TemplateChild<gtk::ColorDialogButton>,
    #[template_child]
    pub pre_bg_color: TemplateChild<gtk::ColorDialogButton>,
    #[template_child]
    pub quote_fg_color: TemplateChild<gtk::ColorDialogButton>,
    #[template_child]
    pub quote_bg_color: TemplateChild<gtk::ColorDialogButton>,
    #[template_child]
    pub link_color: TemplateChild<gtk::ColorDialogButton>,
    #[template_child]
    pub hover_color: TemplateChild<gtk::ColorDialogButton>,
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
            .bind("paragraph-font", &self.pg_font.get(), "font-desc")
            .set_mapping(|_, font| {
                Some(font.as_str().into())
            })
            .mapping(|font,_| {
                let font = font.get::<String>().unwrap();
                Some(FontDescription::from_string(&font).into())
            })
            .build();
        settings
            .bind("h1-font", &self.h1_font.get(), "font-desc")
            .set_mapping(|_, font| {
                Some(font.as_str().into())
            })
            .mapping(|font,_| {
                let font = font.get::<String>().unwrap();
                Some(FontDescription::from_string(&font).into())
            })
            .build();
        settings
            .bind("h2-font", &self.h2_font.get(), "font-desc")
            .set_mapping(|_, font| {
                Some(font.as_str().into())
            })
            .mapping(|font,_| {
                let font = font.get::<String>().unwrap();
                Some(FontDescription::from_string(&font).into())
            })
            .build();
        settings
            .bind("h3-font", &self.h3_font.get(), "font-desc")
            .set_mapping(|_, font| {
                Some(font.as_str().into())
            })
            .mapping(|font,_| {
                let font = font.get::<String>().unwrap();
                Some(FontDescription::from_string(&font).into())
            })
            .build();
        settings
            .bind("preformatted-font", &self.pre_font.get(), "font-desc")
            .set_mapping(|_, font| {
                Some(font.as_str().into())
            })
            .mapping(|font,_| {
                let font = font.get::<String>().unwrap();
                Some(FontDescription::from_string(&font).into())
            })
            .build();
        settings
            .bind("quote-font", &self.quote_font.get(), "font-desc")
            .set_mapping(|_, font| {
                Some(font.as_str().into())
            })
            .mapping(|font,_| {
                let font = font.get::<String>().unwrap();
                Some(FontDescription::from_string(&font).into())
            })
            .build();
    }
}

impl PreferencesWindowImpl for PreferencesWindow {}
impl AdwWindowImpl for PreferencesWindow {}
impl WindowImpl for PreferencesWindow {}
impl WidgetImpl for PreferencesWindow {}
