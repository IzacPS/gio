use crate::{keymod::Keymod, utils::Key};

pub struct Keyboard {
    pub keys: Vec<Key>,
    pub mods: Keymod,
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
