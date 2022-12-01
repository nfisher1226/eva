use adw::{
    gtk::{
        self,
        glib::{self, subclass::InitializingObject},
        prelude::*,
        subclass::prelude::*,
        CompositeTemplate,
    },
    subclass::prelude::*,
};

#[derive(CompositeTemplate, Default)]
#[template(file = "window.ui")]
pub struct Window {
    #[template_child]
    pub header_bar: TemplateChild<adw::HeaderBar>,
    #[template_child]
    pub tab_bar: TemplateChild<adw::TabBar>,
    #[template_child]
    pub new_tab: TemplateChild<gtk::Button>,
    #[template_child]
    pub overlay: TemplateChild<adw::ToastOverlay>,
    #[template_child]
    pub tabview: TemplateChild<adw::TabView>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "Window";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();
        let instance = self.instance();
        if let Some(app) = instance.application() {
            if let Some(app) = app.downcast_ref::<crate::prelude::Application>() {
                app.add_actions(&instance);
            }
        }
    }
}

impl AdwApplicationWindowImpl for Window {}
impl ApplicationWindowImpl for Window {}
impl AdwWindowImpl for Window {}
impl WindowImpl for Window {}
impl WidgetImpl for Window {}
