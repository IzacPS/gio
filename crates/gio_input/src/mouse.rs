use crate::utils::Button;

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
