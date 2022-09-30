// let mut byte: u8 = 0b0000_0000;

// byte |= 0b0000_1000; // Set a bit
// println!("0b{:08b}", byte);

// byte &= 0b1111_0111; // Unset a bit
// println!("0b{:08b}", byte);

// byte ^= 0b0000_1000; // Toggle a bit
// println!("0b{:08b}", byte);

pub struct Key {
    pub is_pressed: bool,
    pub is_repeating: bool,
}

impl Key {
    pub fn new() -> Self {
        Self {
            is_pressed: false,
            is_repeating: false,
        }
    }
}
pub struct Button {
    pub is_pressed: bool,
}

impl Button {
    pub fn new() -> Self {
        Self { is_pressed: false }
    }
}
