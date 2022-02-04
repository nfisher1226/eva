#![warn(clippy::all, clippy::pedantic)]
use gemview::GemView;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::gio::{Cancellable, File, MemoryInputStream, SimpleAction};
use gtk::glib::char::Char;
use gtk::glib;
use glib::closure_local;
use gtk::glib::{clone, OptionArg, OptionFlags};
use gtk::prelude::*;
use gtk::{Application, ResponseType};

use std::path::PathBuf;
use std::process::Command;
use std::rc::Rc;

mod tab;

use crate::CONFIG;

struct Actions {
    new_tab: SimpleAction,
    close_tab: SimpleAction,
    new_window: SimpleAction,
    open_bookmarks: SimpleAction,
    bookmark_page: SimpleAction,
    open_prefs: SimpleAction,
    open_about: SimpleAction,
    quit: SimpleAction,
}

impl Default for Actions {
    fn default() -> Self {
        Self {
            new_tab: SimpleAction::new("new_tab", None),
            close_tab: SimpleAction::new("close_tab", None),
            new_window: SimpleAction::new("new_window", None),
            open_bookmarks: SimpleAction::new("open_bookmarks", None),
            bookmark_page: SimpleAction::new("bookmark_page", None),
            open_prefs: SimpleAction::new("open_prefs", None),
            open_about: SimpleAction::new("open_about", None),
            quit: SimpleAction::new("quit", None),
        }
    }
}

impl Actions {
    fn connect(&self, gui: &Rc<Gui>, app: &Application) {
        self.new_tab.connect_activate(clone!(@strong gui => move |_, _| {
            gui.new_tab(None);
        }));

        self.close_tab.connect_activate(clone!(@weak gui => move |_,_| {
            gui.close_current();
        }));

        self.new_window
            .connect_activate(clone!(@weak gui, @strong app => move |_,_| {
                let new_gui = build_ui(&app);
                new_gui.new_tab(None);
            }));

        self.open_bookmarks.connect_activate(clone!(@weak gui => move |_,_| {
            println!("Open bookmarks");
        }));

        self.bookmark_page.connect_activate(clone!(@weak gui => move |_,_| {
            println!("Bookmark page");
        }));

        self.open_prefs.connect_activate(clone!(@weak gui => move |_,_| {
            println!("Open prefs");
        }));

        self.open_about.connect_activate(clone!(@weak gui => move |_,_| {
            println!("Open about");
        }));

        self.quit.connect_activate(clone!(@weak gui => move |_,_| {
            gui.window.close();
        }));
    }
}

struct Gui {
    window: gtk::ApplicationWindow,
    notebook: gtk::Notebook,
}

impl Default for Gui {
    fn default() -> Self {
        let builder = gtk::Builder::from_string(include_str!("main.ui"));
        let window: gtk::ApplicationWindow = builder.object("mainWindow").unwrap();
        let notebook: gtk::Notebook = builder.object("mainNotebook").unwrap();

        Self {
            window,
            notebook,
        }
    }
}

impl Gui {
    fn add_actions(&self, app: &gtk::Application) -> Actions {
        let actions = Actions::default();

        app.set_accels_for_action("win.new_tab", &["<primary>T"]);
        app.set_accels_for_action("win.close_tab", &["<primary>W"]);
        app.set_accels_for_action("win.new_window", &["<primary>N"]);
        app.set_accels_for_action("win.open_bookmarks", &["<primary><Shift>O"]);
        app.set_accels_for_action("win.bookmark_page", &["<primary>D"]);
        app.set_accels_for_action("win.open_prefs", &["<primary><Shift>P"]);
        app.set_accels_for_action("win.open_about", &["<primary><Shift>A"]);
        app.set_accels_for_action("win.quit", &["<primary>Q"]);

        self.window.add_action(&actions.new_tab);
        self.window.add_action(&actions.close_tab);
        self.window.add_action(&actions.new_window);
        self.window.add_action(&actions.open_bookmarks);
        self.window.add_action(&actions.bookmark_page);
        self.window.add_action(&actions.open_prefs);
        self.window.add_action(&actions.open_about);
        self.window.add_action(&actions.quit);
        actions
    }

    fn new_tab(&self, uri: Option<&str>) {
        let newtab = tab::Tab::default();
        if let Some(uri) = uri {
            if let Ok(u) = gmi::url::Url::try_from(uri) {
                let host = u.authority.host;
                newtab.label().label().set_label(&host);
            }
            newtab.addr_bar().set_text(uri);
            match newtab.viewer().visit(uri) {
                Ok(_) => {},
                Err(e) => eprintln!("{:?}", e),
            }
        }
        self.notebook.append_page(&newtab.tab(), Some(&newtab.label().handle()));
        self.notebook.set_tab_reorderable(&newtab.tab(), true);
        let notebook = self.notebook.clone();
        let t = newtab.clone();
        newtab.label().close_button().connect_clicked(move |_| {
            notebook.detach_tab(&t.tab())
        });
        let t = newtab.clone();
        newtab.addr_bar().connect_activate(move |bar| {
            let uri = String::from(bar.text());
            match t.viewer().visit(&uri) {
                Ok(_) => {},
                Err(e) => eprintln!("{:?}", e),
            }
        });
        let t = newtab.clone();
        newtab.viewer().connect_page_load_started(move |_,uri| {
            t.addr_bar().set_text(&uri);
        });
        let t = newtab.clone();
        newtab.viewer().connect_page_load_redirect(move |_,uri| {
            t.addr_bar().set_text(&uri);
        });
        let t = newtab.clone();
        newtab.viewer().connect_page_loaded(move |_,uri| {
            t.addr_bar().set_text(&uri);
        });
    }

    fn current_page(&self) -> Option<u32> {
        self.notebook.current_page()
    }

    fn close_current(&self) {
        if let Some(page) = self.current_page() {
            self.notebook.remove_page(Some(page));
        }
    }
}

pub fn run(addresses: Option<Vec<String>>) {
    let application = Rc::new(gtk::Application::new(
        Some("org.hitchhiker-linux.eva"),
        gtk::gio::ApplicationFlags::HANDLES_OPEN,
    ));

    application.add_main_option(
        "private",
        Char::from(b'p'),
        OptionFlags::NONE,
        OptionArg::None,
        "",
        None,
    );

    match application.register(Some(&Cancellable::new())) {
        Ok(_) => {},
        Err(e) => eprintln!("{}", e),
    };

    application.connect_open(move |app,addr,_| {
        let gui = build_ui(&app);
        for uri in addr {
            gui.new_tab(Some(&uri.uri()));
        }
    });
    application.connect_activate(|app| {
        let gui = build_ui(&app);
        gui.new_tab(None);
    });
    application.run();
}

fn build_ui(app: &Application) -> Rc<Gui> {
    let gui = Rc::new(Gui::default());
    gui.add_actions(app).connect(&gui, app);
    let cfg = CONFIG.lock().unwrap().clone();
    gui.window.set_application(Some(app));
    gui.notebook.connect_page_removed(clone!(@weak gui => move |nb,page,_| {
        if nb.n_pages() == 0 {
            gui.window.close();
        }
    }));

    gui.window.show();
    gui
}
