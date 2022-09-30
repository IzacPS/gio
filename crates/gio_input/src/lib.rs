mod keyboard;
pub mod keycode;
pub mod keymod;
mod mouse;
mod utils;

use keyboard::Keyboard;
use mouse::Mouse;

pub struct Input {
    keyboard: Keyboard,
    mouse: Mouse,
}

impl Input {
    pub fn new() -> Self {
        Self {
            keyboard: Keyboard::new(),
            mouse: Mouse::new(),
        }
    }

    pub fn keyboard(&self) -> &Keyboard {
        &self.keyboard
    }
    pub fn keyboard_mut(&mut self) -> &mut Keyboard {
        &mut self.keyboard
    }
    pub fn mouse(&self) -> &Mouse {
        &self.mouse
    }
    pub fn mouse_mut(&mut self) -> &mut Mouse {
        &mut self.mouse
    }
}

// use std::sync::{Arc, Mutex};

// use lazy_static::lazy_static;

// lazy_static! {
//     pub static ref GIO_INPUT: Arc<Mutex<Input>> = Arc::new(Mutex::new(Input::new()));
// }

#[cfg(test)]
mod tests {
    use crate::Input;

    #[test]
    fn test_input() {}
}
