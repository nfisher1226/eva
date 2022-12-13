use {
    crate::{bookmarks::Bookmark, BOOKMARKS},
    adw::gtk::{
        glib::{self, clone, subclass::InitializingObject},
        prelude::*,
        subclass::prelude::*,
        CompositeTemplate,
    },
};

#[derive(CompositeTemplate, Default)]
#[template(file = "bookmark_editor.ui")]
pub struct BookmarkEditor {
    #[template_child]
    pub label: TemplateChild<gtk::Label>,
    #[template_child]
    pub name: TemplateChild<gtk::Entry>,
    #[template_child]
    pub description: TemplateChild<gtk::Entry>,
    #[template_child]
    pub url: TemplateChild<gtk::Entry>,
    #[template_child]
    pub tags: TemplateChild<gtk::Entry>,
    #[template_child]
    pub cancel: TemplateChild<gtk::Button>,
    #[template_child]
    pub accept: TemplateChild<gtk::Button>,
}

#[glib::object_subclass]
impl ObjectSubclass for BookmarkEditor {
    const NAME: &'static str = "BookmarkEditor";
    type Type = super::BookmarkEditor;
    type ParentType = gtk::Popover;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for BookmarkEditor {
    fn constructed(&self) {
        self.parent_constructed();
        self.cancel
            .get()
            .connect_clicked(clone!(@weak self as s => move |_| {
                s.instance().popdown();
            }));
        self.accept
            .get()
            .connect_clicked(clone!(@weak self as s => move |_| {
                let bm = Bookmark::from(&s.instance().clone());
                if let Ok(mut bmarks) = BOOKMARKS.try_lock() {
                    bmarks.update(&bm);
                    if let Err(e) = bmarks.save() {
                        eprintln!("Error: {e}");
                    }
                    s.instance().popdown();
                }
            }));
    }
}

impl WidgetImpl for BookmarkEditor {}
impl PopoverImpl for BookmarkEditor {}
