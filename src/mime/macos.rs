use std::error::Error;
use std::process::Command;

pub fn open(uri: &str) {
    Command::new("open")
        .arg(uri)
        .spawn()?;
    Ok(())
}
