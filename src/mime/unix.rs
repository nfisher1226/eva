use std::error::Error;
use std::process::Command;

pub fn open(url: &str) -> Result<(), Box<dyn Error>> {
        match guess_desktop_env() {
            "kde" => run("kde-open", vec!(url))
                .or_else(|_| run("kde-open5", vec!(url))),
            "gnome" => run("gio", vec!("open", url))
                .or_else(|_| run("gvfs-open", vec!(url)))
                .or_else(|_| run("gnome-open", vec!(url))),
            "mate" => run("gio", vec!("open", url))
                .or_else(|_| run("gvfs-open", vec!(url)))
                .or_else(|_| run("mate-open", vec!(url))),
            "xfce" => run("exo-open", vec!(url))
                .or_else(|_| run("gio", vec!("open", url)))
                .or_else(|_| run("gvfs-open", vec!(url))),
            _ => run("xdg-open", vec!(url))
        }?;
    Ok(())
}

// from 'https://github.com/amodm/webbrowser-rs' `src/unix.rs`
fn guess_desktop_env() -> &'static str {
    let unknown = "unknown";
    let xcd: String = std::env::var("XDG_CURRENT_DESKTOP")
        .unwrap_or_else(|_| unknown.into())
        .to_ascii_lowercase();
    let dsession: String = std::env::var("DESKTOP_SESSION")
        .unwrap_or_else(|_| unknown.into())
        .to_ascii_lowercase();

    if xcd.contains("gnome") || xcd.contains("cinnamon") || dsession.contains("gnome") {
        // GNOME and its derivatives
        "gnome"
    } else if xcd.contains("kde")
        || std::env::var("KDE_FULL_SESSION").is_ok()
        || std::env::var("KDE_SESSION_VERSION").is_ok()
    {
        // KDE: https://userbase.kde.org/KDE_System_Administration/Environment_Variables#Automatically_Set_Variables
        "kde"
    } else if xcd.contains("mate") || dsession.contains("mate") {
        // We'll treat MATE as distinct from GNOME due to mate-open
        "mate"
    } else if xcd.contains("xfce") || dsession.contains("xfce") {
        // XFCE
        "xfce"
    } else {
        // All others
        unknown
    }
}

fn run(handler: &str, options: Vec<&str>) -> Result<(), Box<dyn Error>> {
    Command::new(handler)
        .args(options)
        .spawn()?;
    Ok(())
}
