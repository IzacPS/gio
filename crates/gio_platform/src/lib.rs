// pub mod input;

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

use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
use window::WindowInterface;

pub struct Platform {
    window_interface: Arc<WindowInterface>,
}

impl Platform {
    pub fn new() -> Self {
        Self {
            window_interface: Arc::new(WindowInterface::new()),
        }
    }
    pub fn set_window_interface(&mut self, interface: Arc<WindowInterface>) {
        self.window_interface = interface;
    }
    pub fn window_interface(&self) -> &Arc<WindowInterface> {
        &self.window_interface
    }
}

lazy_static! {
    pub static ref GIO_PLATFORM: Arc<Mutex<Platform>> = Arc::new(Mutex::new(Platform::new()));
}

// #[cfg(test)]
// mod tests {

//     #[test]
//     fn test_create_window() {}
// }
