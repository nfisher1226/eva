#![warn(clippy::all, clippy::pedantic)]
use gtk::gio::{Cancellable, SimpleAction};
use gtk::glib::char::Char;
use gtk::glib;
use gtk::glib::{clone, OptionArg, OptionFlags};
use gtk::prelude::*;
use gtk::Application;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

mod tab;
use tab::Tab;

mod dialogs;
use dialogs::Dialogs;

use crate::CONFIG;

struct Actions {
    new_tab: SimpleAction,
    close_tab: SimpleAction,
    next_tab: SimpleAction,
    prev_tab: SimpleAction,
    tab1: SimpleAction,
    tab2: SimpleAction,
    tab3: SimpleAction,
    tab4: SimpleAction,
    tab5: SimpleAction,
    tab6: SimpleAction,
    tab7: SimpleAction,
    tab8: SimpleAction,
    tab9: SimpleAction,
    reload: SimpleAction,
    new_window: SimpleAction,
    open_bookmarks: SimpleAction,
    bookmark_page: SimpleAction,
    open_history: SimpleAction,
    clear_history: SimpleAction,
    open_prefs: SimpleAction,
    open_about: SimpleAction,
    quit: SimpleAction,
}

impl Default for Actions {
    fn default() -> Self {
        Self {
            new_tab: SimpleAction::new("new_tab", None),
            close_tab: SimpleAction::new("close_tab", None),
            next_tab: SimpleAction::new("next_tab", None),
            prev_tab: SimpleAction::new("prev_tab", None),
            tab1: SimpleAction::new("tab1", None),
            tab2: SimpleAction::new("tab2", None),
            tab3: SimpleAction::new("tab3", None),
            tab4: SimpleAction::new("tab4", None),
            tab5: SimpleAction::new("tab5", None),
            tab6: SimpleAction::new("tab6", None),
            tab7: SimpleAction::new("tab7", None),
            tab8: SimpleAction::new("tab8", None),
            tab9: SimpleAction::new("tab9", None),
            reload: SimpleAction::new("reload", None),
            new_window: SimpleAction::new("new_window", None),
            open_bookmarks: SimpleAction::new("open_bookmarks", None),
            bookmark_page: SimpleAction::new("bookmark_page", None),
            open_history: SimpleAction::new("open_history", None),
            clear_history: SimpleAction::new("clear_history", None),
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
            gui.close_current_tab();
        }));

        self.next_tab.connect_activate(clone!(@weak gui => move |_,_| {
            gui.next_tab();
        }));

        self.prev_tab.connect_activate(clone!(@weak gui => move |_,_| {
            gui.prev_tab();
        }));

        self.tab1.connect_activate(clone!(@weak gui => move |_,_| {
            gui.notebook.set_page(0);
        }));

        self.tab2.connect_activate(clone!(@weak gui => move |_,_| {
            gui.notebook.set_page(1);
        }));

        self.tab3.connect_activate(clone!(@weak gui => move |_,_| {
            gui.notebook.set_page(2);
        }));

        self.tab4.connect_activate(clone!(@weak gui => move |_,_| {
            gui.notebook.set_page(3);
        }));

        self.tab5.connect_activate(clone!(@weak gui => move |_,_| {
            gui.notebook.set_page(4);
        }));

        self.tab6.connect_activate(clone!(@weak gui => move |_,_| {
            gui.notebook.set_page(5);
        }));

        self.tab7.connect_activate(clone!(@weak gui => move |_,_| {
            gui.notebook.set_page(6);
        }));

        self.tab8.connect_activate(clone!(@weak gui => move |_,_| {
            gui.notebook.set_page(7);
        }));

        self.tab9.connect_activate(clone!(@weak gui => move |_,_| {
            gui.notebook.set_page(8);
        }));

        self.reload.connect_activate(clone!(@weak gui => move |_,_| {
            match gui.reload_current_tab() {
                Ok(_) => {},
                Err(e) => eprintln!("{}", e),
            }
        }));

        self.new_window
            .connect_activate(clone!(@weak gui, @strong app => move |_,_| {
                let new_gui = build_ui(&app);
                new_gui.new_tab(None);
            }));

        self.open_bookmarks.connect_activate(clone!(@weak gui => move |_,_| {
            println!("Not implemented yet");
        }));

        self.bookmark_page.connect_activate(clone!(@weak gui => move |_,_| {
            println!("Not implemented yet");
        }));

        self.open_history.connect_activate(clone!(@weak gui => move |_,_| {
            println!("Not implemented yet");
        }));

        self.clear_history.connect_activate(clone!(@weak gui => move |_,_| {
            println!("Not implemented yet");
        }));

        self.open_prefs.connect_activate(clone!(@weak gui => move |_,_| {
            gui.dialogs.preferences.show();
        }));

        self.open_about.connect_activate(clone!(@weak gui => move |_,_| {
            gui.dialogs.about.show();
        }));

        self.quit.connect_activate(clone!(@weak gui => move |_,_| {
            gui.window.close();
        }));
    }
}

struct Gui {
    window: gtk::ApplicationWindow,
    notebook: gtk::Notebook,
    tabs: RefCell<HashMap<String, Tab>>,
    dialogs: Dialogs,
}

impl Default for Gui {
    fn default() -> Self {
        let builder = gtk::Builder::from_string(include_str!("main.ui"));
        let window: gtk::ApplicationWindow = builder.object("mainWindow").unwrap();
        let notebook: gtk::Notebook = builder.object("mainNotebook").unwrap();
        let tabs: RefCell<HashMap<String, Tab>> = RefCell::new(HashMap::new());
        let dialogs: Dialogs = Dialogs::init(&window, &builder);

        Self {
            window,
            notebook,
            tabs,
            dialogs,
        }
    }
}

impl Gui {
    fn add_actions(&self, app: &gtk::Application) -> Actions {
        let actions = Actions::default();

        app.set_accels_for_action("win.new_tab", &["<primary>T"]);
        app.set_accels_for_action("win.close_tab", &["<primary>W"]);
        app.set_accels_for_action("win.next_tab", &["<primary>Page_Down"]);
        app.set_accels_for_action("win.prev_tab", &["<primary>Page_Up"]);
        app.set_accels_for_action("win.tab1", &["<Alt>1"]);
        app.set_accels_for_action("win.tab2", &["<Alt>2"]);
        app.set_accels_for_action("win.tab3", &["<Alt>3"]);
        app.set_accels_for_action("win.tab4", &["<Alt>4"]);
        app.set_accels_for_action("win.tab5", &["<Alt>5"]);
        app.set_accels_for_action("win.tab6", &["<Alt>6"]);
        app.set_accels_for_action("win.tab7", &["<Alt>7"]);
        app.set_accels_for_action("win.tab8", &["<Alt>8"]);
        app.set_accels_for_action("win.tab9", &["<Alt>9"]);
        app.set_accels_for_action("win.reload", &["<primary>R"]);
        app.set_accels_for_action("win.new_window", &["<primary>N"]);
        app.set_accels_for_action("win.open_bookmarks", &["<primary><Shift>O"]);
        app.set_accels_for_action("win.bookmark_page", &["<primary>D"]);
        app.set_accels_for_action("win.open_history", &["<primary>H"]);
        app.set_accels_for_action("win.open_prefs", &["<primary><Shift>P"]);
        app.set_accels_for_action("win.open_about", &["<primary><Shift>A"]);
        app.set_accels_for_action("win.quit", &["<primary>Q"]);

        self.window.add_action(&actions.new_tab);
        self.window.add_action(&actions.close_tab);
        self.window.add_action(&actions.next_tab);
        self.window.add_action(&actions.prev_tab);
        self.window.add_action(&actions.tab1);
        self.window.add_action(&actions.tab2);
        self.window.add_action(&actions.tab3);
        self.window.add_action(&actions.tab4);
        self.window.add_action(&actions.tab5);
        self.window.add_action(&actions.tab6);
        self.window.add_action(&actions.tab7);
        self.window.add_action(&actions.tab8);
        self.window.add_action(&actions.tab9);
        self.window.add_action(&actions.reload);
        self.window.add_action(&actions.new_window);
        self.window.add_action(&actions.open_bookmarks);
        self.window.add_action(&actions.bookmark_page);
        self.window.add_action(&actions.open_history);
        self.window.add_action(&actions.clear_history);
        self.window.add_action(&actions.open_prefs);
        self.window.add_action(&actions.open_about);
        self.window.add_action(&actions.quit);
        actions
    }

    fn new_tab(&self, uri: Option<&str>) {
        let newtab = tab::Tab::default();
        self.tabs.borrow_mut().insert(newtab.tab().widget_name().to_string(), newtab.clone());
        if let Some(uri) = uri {
            if let Ok(u) = gmi::url::Url::try_from(uri) {
                let host = u.authority.host;
                newtab.label().label().set_label(&host);
            }
            newtab.addr_bar().set_text(uri);
            newtab.reload_button().set_sensitive(true);
            match newtab.viewer().visit(uri) {
                Ok(_) => {},
                Err(e) => eprintln!("{:?}", e),
            }
        }
        self.notebook.append_page(&newtab.tab(), Some(&newtab.label().handle()));
        self.notebook.set_tab_reorderable(&newtab.tab(), true);
        let tab = newtab.clone();
        let notebook = self.notebook.clone();
        newtab.label().close_button().connect_clicked(move |_| {
            let _name = tab.tab().widget_name().to_string();
            notebook.detach_tab(&tab.tab())
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
            t.reload_button().set_sensitive(false);
        });
        let t = newtab.clone();
        newtab.viewer().connect_page_load_redirect(move |_,uri| {
            t.addr_bar().set_text(&uri);
        });
        let t = newtab.clone();
        newtab.viewer().connect_page_loaded(move |_,uri| {
            t.addr_bar().set_text(&uri);
            t.reload_button().set_sensitive(true);
        });
        let t = newtab.clone();
        newtab.viewer().connect_page_load_failed(move |_,uri| {
            t.addr_bar().set_text(&uri);
            t.reload_button().set_sensitive(true);
        });
        let t = newtab.clone();
        newtab.reload_button().connect_clicked(move |_| {
            match t.viewer().reload() {
                Ok(_) => {},
                Err(e) => eprintln!("{:?}", e),
            }
        });
    }

    fn current_page(&self) -> Option<u32> {
        self.notebook.current_page()
    }

    fn current_tab(&self) -> Option<Tab> {
        if let Some(t) = self.notebook.nth_page(self.current_page()) {
            match self.tabs.borrow().get(&t.widget_name().to_string()) {
                Some(t) => Some(t.clone()),
                None => None,
            }
        } else {
            None
        }
    }

    fn next_tab(&self) {
        if let Some(current) = self.notebook.current_page() {
            let pages = self.notebook.n_pages();
            if current == pages - 1 {
                self.notebook.set_page(0);
            } else {
                self.notebook.set_page((current + 1).try_into().unwrap());
            }
        }
    }

    fn prev_tab(&self) {
        if let Some(current) = self.current_page() {
            let pages = self.notebook.n_pages();
            if current == 0 {
                self.notebook.set_page((pages - 1).try_into().unwrap());
            } else {
                self.notebook.set_page((current - 1).try_into().unwrap());
            }
        }
    }

    fn close_current_tab(&self) {
        if let Some(page) = self.current_page() {
            if let Some(tab) = self.current_tab() {
                let name = tab.tab().widget_name().to_string();
                self.tabs.borrow_mut().remove(&name);
            }
            self.notebook.remove_page(Some(page));
        }
    }

    fn close_tab_named(&self, name: &str) {
        match self.tabs.borrow().get(name) {
            Some(tab) => self.notebook.detach_tab(&tab.tab()),
            None => {},
        }
        self.tabs.borrow_mut().remove(name);
    }

    fn cleanup_tabs(&self) {
        for (name, tab) in self.tabs.borrow().clone() {
            match self.notebook.page_num(&tab.tab()) {
                Some(_) => {},
                None => _ = self.tabs.borrow_mut().remove(&name),
            }
        }
    }

    fn reload_current_tab(&self)-> Result<(), Box<dyn std::error::Error>> {
        if let Some(tab) = self.current_tab() {
            match tab.viewer().reload() {
                Ok(c) => Ok(c),
                Err(e) => Err(e),
            }
        } else {
            Err(String::from("Error getting tab").into())
        }
    }
}

pub fn run() {
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
    let _cfg = CONFIG.lock().unwrap().clone();
    gui.window.set_application(Some(app));
    gui.notebook.connect_page_removed(clone!(@weak gui => move |nb,_page,_| {
        gui.cleanup_tabs();
        if nb.n_pages() == 0 {
            gui.window.close();
        }
    }));

    gui.window.show();
    gui
}
