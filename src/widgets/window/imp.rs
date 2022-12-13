use {
    crate::prelude::ThemeSwitcher,
    adw::{
        gtk::{
            self,
            glib::{self, clone, subclass::InitializingObject},
            CompositeTemplate,
        },
        prelude::*,
        subclass::prelude::*,
    },
};

#[derive(CompositeTemplate, Default)]
#[template(file = "window.ui")]
pub struct Window {
    #[template_child]
    pub header_bar: TemplateChild<gtk::Box>,
    #[template_child]
    pub tab_bar: TemplateChild<adw::TabBar>,
    #[template_child]
    pub new_tab: TemplateChild<gtk::Button>,
    #[template_child]
    pub menu_button: TemplateChild<gtk::MenuButton>,
    #[template_child]
    pub overlay: TemplateChild<adw::ToastOverlay>,
    #[template_child]
    pub tab_view: TemplateChild<adw::TabView>,
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
        if let Some(pop) = self
            .menu_button
            .popover()
            .map(|x| x.downcast::<gtk::PopoverMenu>().ok())
            .flatten()
        {
            let switcher = ThemeSwitcher::new();
            pop.add_child(&switcher, "theme");
        }
        instance.set_css();
        self.connect_signals();
    }
}

impl Window {
    fn connect_signals(&self) {
        let win = self.instance();
        self.tab_view
            .get()
            .connect_n_pages_notify(clone!(@weak win => move |view| {
                if view.n_pages() == 0 {
                    win.close();
                }
            }));
    }
}

impl AdwApplicationWindowImpl for Window {}
impl ApplicationWindowImpl for Window {}
impl AdwWindowImpl for Window {}
impl WindowImpl for Window {}
impl WidgetImpl for Window {}
