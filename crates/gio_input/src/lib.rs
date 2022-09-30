pub mod keycode;
pub mod keymods;
pub mod mouse;

struct Key {
    is_pressed: bool,
    is_repeating: bool,
    caracter: char,
}

impl Key {
    fn new() -> Self {
        Self {
            is_pressed: false,
            is_repeating: false,
            caracter: char::default(),
        }
    }
}
struct Button {
    is_pressed: bool,
}

impl Button {
    fn new() -> Self {
        Self { is_pressed: false }
    }
}

struct Keyboard {
    keys: Vec<Key>,
}

impl Keyboard {
    fn new() -> Self {
        Self {
            keys: (0..256).into_iter().map(|_| Key::new()).collect(),
        }
    }
    fn is_key_pressed(&self, key: usize) -> bool {
        self.keys[key].is_pressed
    }
    fn is_repeating(&self, key: usize) -> bool {
        self.keys[key].is_repeating
    }
    fn get_character(&self, key: usize) -> char {
        self.keys[key].caracter
    }
}
struct Mouse {
    buttons: Vec<Button>,
    has_moved: bool,
    x: u32,
    y: u32,
    scroll_delta: f64,
    scroll_deltah: f64,
}

impl Mouse {
    fn new() -> Self {
        Self {
            buttons: (0..16).into_iter().map(|_| Button::new()).collect(),
            has_moved: false,
            x: 0,
            y: 0,
            scroll_delta: 0.0,
            scroll_deltah: 0.0,
        }
    }
}

struct Input {
    keyboard: Keyboard,
    mouse: Mouse,
}

impl Input {
    fn new() -> Self {
        Self {
            keyboard: Keyboard::new(),
            mouse: Mouse::new(),
        }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
