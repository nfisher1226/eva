#[cfg(all(target_family = "unix",not(target_os = "macos")))]
mod unix;
#[cfg(all(target_family = "unix",not(target_os = "macos")))]
pub use unix::open;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::open;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::open;
