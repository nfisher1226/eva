use {
    crate::keys::Keys,
    crate::prelude::*,
    gtk::{
        gio::SimpleAction,
        glib::{self, clone},
        prelude::*,
        subclass::prelude::*,
    },
};

const ACTIONS: [&str; 18] = [
    "new_tab",
    "close_tab",
    "next_tab",
    "prev_tab",
    "reload",
    "go_home",
    "go_previous",
    "go_next",
    "new_window",
    "open_bookmarks",
    "bookmark_page",
    "open_history",
    "clear_history",
    "view_source",
    "save_page",
    "open_prefs",
    "open_about",
    "quit",
];

pub fn add(win: &Window, app: &Application) {
    let keys = Keys::from_file().unwrap_or_default();
    for name in &ACTIONS {
        let action = SimpleAction::new(name, None);
        app.set_accels_for_action(&format!("win.{}", name), &[keys.get(name)]);
        win.add_action(&action);
        match *name {
            "new_tab" => {
                action.connect_activate(clone!(@strong win, @weak app => move |_,_| {
                    match app.imp().settings.string("new-page").as_str() {
                        "home" => {
                            let mut page = app.imp().settings.string("homepage").to_string();
                            win.open_tab(Some(&mut page));
                        },
                        _ => win.open_tab(None),
                    };
                }));
            }
            "close_tab" => {
                action.connect_activate(clone!(@weak win => move |_,_| {
                    win.close_current_page();
                }));
            }
            "next_tab" => {
                action.connect_activate(clone!(@weak win => move |_,_| {
                    //win.next_tab();
                }));
            }
            "prev_tab" => {
                action.connect_activate(clone!(@weak win => move |_,_| {
                    //win.prev_tab();
                }));
            }
            "reload" => {
                action.connect_activate(clone!(@weak win => move |_,_| {
                    win.on_reload();
                }));
            }
            "go_home" => {
                action.connect_activate(clone!(@weak win => move |_,_| {
                    //if let Err(e) = gui.go_home() {
                    //    eprintln!("{}", e);
                    //}
                }));
            }
            "go_previous" => {
                action.connect_activate(clone!(@weak win => move |_,_| {
                    win.on_go_previous();
                }));
            }
            "go_next" => {
                action.connect_activate(clone!(@weak win => move |_,_| {
                    win.on_go_next();
                }));
            }
            "new_window" => {
                action.connect_activate(clone!(@weak win, @strong app => move |_,_| {
                    //let new_gui = crate::gui::build_ui(&app);
                    //new_gui.new_tab(None);
                }));
            }
            "open_bookmarks" => {
                action.connect_activate(clone!(@weak win => move |_,_| {
                    win.current_tab().map(|t| t.request_eva_page("eva://bookmarks"));
                }));
            }
            "bookmark_page" => {
                action.connect_activate(clone!(@weak win => move |_,_| {
                    //if let Some(tab) = gui.current_tab() {
                    //    tab.bookmark_editor.popup();
                    //}
                }));
            }
            "open_history" => {
                action.connect_activate(clone!(@weak win => move |_,_| {
                    win.current_tab().map(|t| t.request_eva_page("eva://history"));
                }));
            }
            "clear_history" => {
                action.connect_activate(clone!(@weak win => move |_,_| {
                    //println!("Not implemented yet");
                }));
            }
            "view_source" => {
                action.connect_activate(clone!(@weak win => move |_,_| {
                    //if let Some(tab) = gui.current_tab() {
                    //    tab.view_source();
                    //}
                }));
            }
            "save_page" => {
                action.connect_activate(clone!(@weak win => move |_,_| {
                    //gui.save_page();
                }));
            }
            "open_prefs" => {
                action.connect_activate(clone!(@weak app => move |_,_| {
                    PreferencesWindow::new(app.settings()).present();
                }));
            }
            "open_about" => {
                action.connect_activate(clone!(@weak win => move |_,_| {
                    //gui.dialogs.about.show();
                }));
            }
            "quit" => {
                action.connect_activate(clone!(@weak win => move |_,_| {
                    win.close();
                }));
            }
            _ => {}
        }
    }
}
