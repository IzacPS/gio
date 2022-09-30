pub mod keycode;
pub mod keymods;
pub mod mouse;

// let mut byte: u8 = 0b0000_0000;

// byte |= 0b0000_1000; // Set a bit
// println!("0b{:08b}", byte);

// byte &= 0b1111_0111; // Unset a bit
// println!("0b{:08b}", byte);

// byte ^= 0b0000_1000; // Toggle a bit
// println!("0b{:08b}", byte);

struct Key {
    is_pressed: bool,
    is_repeating: bool,
}

impl Key {
    pub fn new() -> Self {
        Self {
            is_pressed: false,
            is_repeating: false,
        }
    }
}
struct Button {
    is_pressed: bool,
}

impl Button {
    pub fn new() -> Self {
        Self { is_pressed: false }
    }
}

#[derive(Default)]
pub struct Keymod {
    state: usize,
}

impl Keymod {
    pub fn set_shift(&mut self) -> &mut Self {
        self.state |= keymods::SHIFT;
        self
    }
    pub fn set_control(&mut self) -> &mut Self {
        self.state |= keymods::CONTROL;
        self
    }

    pub fn set_alt(&mut self) -> &mut Self {
        self.state |= keymods::ALT;
        self
    }
    pub fn set_capslock(&mut self) -> &mut Self {
        self.state |= keymods::CAPSLOCK;
        self
    }
    pub fn set_system(&mut self) -> &mut Self {
        self.state |= keymods::SYSTEM;
        self
    }
    pub fn set_lshift(&mut self) -> &mut Self {
        self.state |= keymods::LSHIFT;
        self
    }
    pub fn set_rshift(&mut self) -> &mut Self {
        self.state |= keymods::RSHIFT;
        self
    }
    pub fn set_lcontrol(&mut self) -> &mut Self {
        self.state |= keymods::LCONTROL;
        self
    }
    pub fn set_rcontrol(&mut self) -> &mut Self {
        self.state |= keymods::RCONTROL;
        self
    }
    pub fn set_lalt(&mut self) -> &mut Self {
        self.state |= keymods::LALT;
        self
    }
    pub fn set_ralt(&mut self) -> &mut Self {
        self.state |= keymods::RALT;
        self
    }
    pub fn set_lsystem(&mut self) -> &mut Self {
        self.state |= keymods::LSYSTEM;
        self
    }
    pub fn set_rsystem(&mut self) -> &mut Self {
        self.state |= keymods::RSYSTEM;
        self
    }

    pub fn unset_shift(&mut self) -> &mut Self {
        self.state &= keymods::SHIFT;
        self
    }
    pub fn unset_control(&mut self) -> &mut Self {
        self.state &= keymods::CONTROL;
        self
    }

    pub fn unset_alt(&mut self) -> &mut Self {
        self.state &= keymods::ALT;
        self
    }
    pub fn unset_capslock(&mut self) -> &mut Self {
        self.state &= keymods::CAPSLOCK;
        self
    }
    pub fn unset_system(&mut self) -> &mut Self {
        self.state &= keymods::SYSTEM;
        self
    }
    pub fn unset_lshift(&mut self) -> &mut Self {
        self.state &= keymods::LSHIFT;
        self
    }
    pub fn unset_rshift(&mut self) -> &mut Self {
        self.state &= keymods::RSHIFT;
        self
    }
    pub fn unset_lcontrol(&mut self) -> &mut Self {
        self.state &= keymods::LCONTROL;
        self
    }
    pub fn unset_rcontrol(&mut self) -> &mut Self {
        self.state &= keymods::RCONTROL;
        self
    }
    pub fn unset_lalt(&mut self) -> &mut Self {
        self.state &= keymods::LALT;
        self
    }
    pub fn unset_ralt(&mut self) -> &mut Self {
        self.state &= keymods::RALT;
        self
    }
    pub fn unset_lsystem(&mut self) -> &mut Self {
        self.state &= keymods::LSYSTEM;
        self
    }
    pub fn unset_rsystem(&mut self) -> &mut Self {
        self.state &= keymods::RSYSTEM;
        self
    }

    pub fn is_shift_set(&self) -> bool {
        (self.state & keymods::SHIFT) != 0
    }
    pub fn is_control_set(&self) -> bool {
        (self.state & keymods::CONTROL) != 0
    }

    pub fn is_alt_set(&self) -> bool {
        (self.state & keymods::ALT) != 0
    }
    pub fn is_capslock_set(&self) -> bool {
        (self.state & keymods::CAPSLOCK) != 0
    }
    pub fn is_system_set(&self) -> bool {
        (self.state & keymods::SYSTEM) != 0
    }
    pub fn is_lshift_set(&self) -> bool {
        (self.state & keymods::LSHIFT) != 0
    }
    pub fn is_rshift_set(&self) -> bool {
        (self.state & keymods::RSHIFT) != 0
    }
    pub fn is_lcontrol_set(&self) -> bool {
        (self.state & keymods::LCONTROL) != 0
    }
    pub fn is_rcontrol_set(&self) -> bool {
        (self.state & keymods::RCONTROL) != 0
    }
    pub fn is_lalt_set(&self) -> bool {
        (self.state & keymods::LALT) != 0
    }
    pub fn is_ralt_set(&self) -> bool {
        (self.state & keymods::RALT) != 0
    }
    pub fn is_lsystem_set(&self) -> bool {
        (self.state & keymods::LSYSTEM) != 0
    }
    pub fn is_rsystem_set(&self) -> bool {
        (self.state & keymods::RSYSTEM) != 0
    }
}
pub struct Keyboard {
    keys: Vec<Key>,
    mods: Keymod,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            keys: (0..256).into_iter().map(|_| Key::new()).collect(),
            mods: Keymod::default(),
        }
    }
    pub fn mods(&self) -> &Keymod {
        &self.mods
    }
    pub fn set_mods(&mut self, mods: Keymod) -> &mut Self {
        self.mods = mods;
        self
    }
    pub fn mods_mut(&mut self) -> &mut Keymod {
        &mut self.mods
    }
    pub fn is_key_pressed(&self, key: usize) -> bool {
        self.keys[key].is_pressed
    }
    pub fn is_repeating(&self, key: usize) -> bool {
        self.keys[key].is_repeating
    }
    pub fn set_press_state(&mut self, key: usize, state: bool) -> &mut Self {
        if !state {
            self.keys[key].is_repeating = false;
        }
        self.keys[key].is_pressed = state;
        self
    }
    pub fn set_repeat_state(&mut self, key: usize, state: bool) -> &mut Self {
        self.keys[key].is_repeating = state;
        self
    }
}

#[derive(Default)]
pub struct MousePosition {
    x: u32,
    y: u32,
}
pub struct Mouse {
    buttons: Vec<Button>,
    has_moved: bool,
    position: MousePosition,
    scroll_delta: f64,
    scroll_deltah: f64,
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            buttons: (0..16).into_iter().map(|_| Button::new()).collect(),
            has_moved: false,
            position: MousePosition::default(),
            scroll_delta: 0.0,
            scroll_deltah: 0.0,
        }
    }
    pub fn is_button_pressed(&self, button: usize) -> bool {
        self.buttons[button].is_pressed
    }
    pub fn has_moved(&self) -> bool {
        self.has_moved
    }

    pub fn position(&self) -> &MousePosition {
        &self.position
    }
    pub fn set_position(&mut self, x: u32, y: u32) -> &mut Self {
        self.position = MousePosition { x, y };
        self
    }
    pub fn scroll_delta(&self) -> f64 {
        self.scroll_delta
    }
    pub fn scroll_deltah(&self) -> f64 {
        self.scroll_deltah
    }
    pub fn set_scroll_delta(&mut self, delta: f64) -> &mut Self {
        self.scroll_delta = delta;
        self
    }
    pub fn set_scroll_deltah(&mut self, delta: f64) -> &mut Self {
        self.scroll_delta = delta;
        self
    }
    pub fn set_press_state(&mut self, button: usize, state: bool) -> &mut Self {
        self.buttons[button].is_pressed = state;
        self
    }
}

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

use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref GIO_INPUT: Arc<Mutex<Input>> = Arc::new(Mutex::new(Input::new()));
}

#[cfg(test)]
mod tests {
    use crate::Input;

    #[test]
    fn test_input() {}
}
