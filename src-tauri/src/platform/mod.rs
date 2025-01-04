#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod mac;

pub trait FocusControllerTrait {
    fn initialize(&mut self);
    fn finalize(&mut self);

    fn focus_on(&mut self);
    fn focus_off(&mut self);
}

#[cfg(target_os = "windows")]
pub use windows::FocusController;

#[cfg(target_os = "macos")]
pub use mac::FocusController;
