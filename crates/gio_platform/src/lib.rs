pub mod common;
pub mod input;

use crate::input::Input;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref INPUT: Arc<Mutex<Input<'static>>> = Arc::new(Mutex::new(Input::new()));
}

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub mod window {
    pub type Window = super::linux::window::Window;
    pub type WindowInterface = super::linux::window::WindowInterface;
}

#[cfg(target_os = "windows")]
mod win32;
#[cfg(target_os = "windows")]
pub mod window {
    pub type Window = super::win32::window::Window;
    pub type WindowInterface = super::win32::window::WindowInterface;
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_create_window() {}
}
