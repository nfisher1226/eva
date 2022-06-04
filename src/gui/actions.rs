use {
    gtk::{gio::SimpleAction, glib::{self, clone}, prelude::*},
    std::rc::Rc,
    super::Gui,
};

pub struct Actions {
    pub new_tab: SimpleAction,
    pub close_tab: SimpleAction,
    pub next_tab: SimpleAction,
    pub prev_tab: SimpleAction,
    pub tab1: SimpleAction,
    pub tab2: SimpleAction,
    pub tab3: SimpleAction,
    pub tab4: SimpleAction,
    pub tab5: SimpleAction,
    pub tab6: SimpleAction,
    pub tab7: SimpleAction,
    pub tab8: SimpleAction,
    pub tab9: SimpleAction,
    pub reload: SimpleAction,
    pub go_home: SimpleAction,
    pub go_previous: SimpleAction,
    pub go_next: SimpleAction,
    pub new_window: SimpleAction,
    pub open_bookmarks: SimpleAction,
    pub bookmark_page: SimpleAction,
    pub open_history: SimpleAction,
    pub clear_history: SimpleAction,
    pub view_source: SimpleAction,
    pub save_page: SimpleAction,
    pub open_prefs: SimpleAction,
    pub open_about: SimpleAction,
    pub quit: SimpleAction,
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
            go_home: SimpleAction::new("go_home", None),
            go_previous: SimpleAction::new("go_previous", None),
            go_next: SimpleAction::new("go_next", None),
            new_window: SimpleAction::new("new_window", None),
            open_bookmarks: SimpleAction::new("open_bookmarks", None),
            bookmark_page: SimpleAction::new("bookmark_page", None),
            open_history: SimpleAction::new("open_history", None),
            clear_history: SimpleAction::new("clear_history", None),
            view_source: SimpleAction::new("view_source", None),
            save_page: SimpleAction::new("save_page", None),
            open_prefs: SimpleAction::new("open_prefs", None),
            open_about: SimpleAction::new("open_about", None),
            quit: SimpleAction::new("quit", None),
        }
    }
}

impl Actions {
    pub fn connect_tab(&self, gui: &Rc<Gui>) {
        self.new_tab
            .connect_activate(clone!(@strong gui => move |_, _| {
                gui.new_tab(None);
            }));

        self.close_tab
            .connect_activate(clone!(@weak gui => move |_,_| {
                gui.close_current_tab();
            }));

        self.next_tab
            .connect_activate(clone!(@weak gui => move |_,_| {
                gui.next_tab();
            }));

        self.prev_tab
            .connect_activate(clone!(@weak gui => move |_,_| {
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
    }

    pub fn connect_nav(&self, gui: &Rc<Gui>) {
        self.reload
            .connect_activate(clone!(@weak gui => move |_,_| {
                if let Err(e) = gui.reload_current_tab() {
                    eprintln!("{}", e);
                }
            }));

        self.go_home
            .connect_activate(clone!(@weak gui => move |_,_| {
                if let Err(e) = gui.go_home() {
                    eprintln!("{}", e);
                }
            }));

        self.go_previous
            .connect_activate(clone!(@weak gui => move |_,_| {
                if let Err(e) = gui.go_previous() {
                    eprintln!("{}", e);
                }
            }));

        self.go_next
            .connect_activate(clone!(@weak gui => move |_,_| {
                if let Err(e) = gui.go_next() {
                    eprintln!("{}", e);
                }
            }));
    }

    pub fn connect(&self, gui: &Rc<Gui>) {
        self.connect_tab(gui);
        self.connect_nav(gui);
        self.open_bookmarks
            .connect_activate(clone!(@weak gui => move |_,_| {
                gui.open_bookmarks();
            }));

        self.bookmark_page
            .connect_activate(clone!(@weak gui => move |_,_| {
                if let Some(tab) = gui.current_tab() {
                    tab.bookmark_editor().popover().popup();
                }
            }));

        self.open_history
            .connect_activate(clone!(@weak gui => move |_,_| {
                println!("Not implemented yet");
            }));

        self.view_source
            .connect_activate(clone!(@weak gui => move |_,_| {
                if let Some(tab) = gui.current_tab() {
                    tab.view_source();
                }
            }));

        self.save_page
            .connect_activate(clone!(@weak gui => move |_,_| {
                gui.save_page();
            }));

        self.clear_history
            .connect_activate(clone!(@weak gui => move |_,_| {
                println!("Not implemented yet");
            }));

        self.open_prefs
            .connect_activate(clone!(@weak gui => move |_,_| {
                gui.dialogs.preferences.show();
            }));

        self.open_about
            .connect_activate(clone!(@weak gui => move |_,_| {
                gui.dialogs.about.show();
            }));

        self.quit.connect_activate(clone!(@weak gui => move |_,_| {
            gui.window.close();
        }));
    }
}
