use {
    adw::gtk::{
        self,
        glib::{self, subclass::InitializingObject},
        prelude::*,
        subclass::prelude::*,
        CompositeTemplate,
    },
    gemview::GemView,
};

#[derive(CompositeTemplate, Default)]
#[template(file = "tab.ui")]
pub struct Tab {
    #[template_child]
    pub back_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub forward_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub reload_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub addr_bar: TemplateChild<gtk::Entry>,
    #[template_child]
    pub bookmark_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub scroller: TemplateChild<gtk::ScrolledWindow>,
    #[template_child]
    pub viewer: TemplateChild<GemView>,
}

#[glib::object_subclass]
impl ObjectSubclass for Tab {
    const NAME: &'static str = "Tab";
    type Type = super::Tab;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Tab {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl BoxImpl for Tab {}
impl WidgetImpl for Tab {}
