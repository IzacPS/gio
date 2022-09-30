// pub mod input;

use std::sync::{Arc, Mutex};

use gio_input::Input;
use window::Window;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub mod window {
    pub type Window = super::linux::window::Window;
}

#[cfg(target_os = "windows")]
mod win32;
#[cfg(target_os = "windows")]
pub mod window {
    pub type Window = super::win32::window::Window;
}

pub struct Platform {
    input: Arc<Mutex<Input>>,
    window: Arc<Mutex<Window>>,
}

impl Platform {
    pub fn new() -> Self {
        use linux::window::Window;
        let input = Arc::new(Mutex::new(Input::new()));
        Self {
            input: input.clone(),
            window: Arc::new(Mutex::new(Window::new(input))),
        }
    }
}

// #[cfg(test)]
// mod tests {

//     #[test]
//     fn test_create_window() {}
// }
