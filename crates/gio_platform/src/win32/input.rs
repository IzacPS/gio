use windows_sys::Win32::{
    Foundation::{LPARAM, WPARAM},
    UI::{
        Input::KeyboardAndMouse::*,
        WindowsAndMessaging::{
            KF_EXTENDED, MAPVK_VK_TO_VSC, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN,
            WM_MBUTTONUP, WM_RBUTTONDOWN, WM_RBUTTONUP, WM_XBUTTONDOWN, WM_XBUTTONUP, XBUTTON1,
        },
    },
};

use crate::common::{keycode, mouse};

use super::utils::hiword;

pub fn get_key(key: WPARAM, flags: LPARAM) -> usize {
    match key as u16 {
        VK_SHIFT => {
            let lshift = unsafe { MapVirtualKeyW(VK_LSHIFT as u32, MAPVK_VK_TO_VSC) as isize };
            let scancode = (flags & (0xFF << 16)) >> 16;
            if lshift == scancode {
                keycode::LSHIFT
            } else {
                keycode::RSHIFT
            }
        }
        VK_MENU => {
            if (hiword(flags as u64) & (KF_EXTENDED as u16)) != 0 {
                keycode::RALT
            } else {
                keycode::LALT
            }
        }
        VK_CONTROL => {
            if (hiword(flags as u64) & (KF_EXTENDED as u16)) != 0 {
                keycode::RCONTROL
            } else {
                keycode::LCONTROL
            }
        }
        0x41 => keycode::_A,
        0x42 => keycode::_B,
        0x43 => keycode::_C,
        0x44 => keycode::_D,
        0x45 => keycode::_E,
        0x46 => keycode::_F,
        0x47 => keycode::_G,
        0x48 => keycode::_H,
        0x49 => keycode::_I,
        0x4A => keycode::_J,
        0x4B => keycode::_K,
        0x4C => keycode::_L,
        0x4D => keycode::_M,
        0x4E => keycode::_N,
        0x4F => keycode::_O,
        0x50 => keycode::_P,
        0x51 => keycode::_Q,
        0x52 => keycode::_R,
        0x53 => keycode::_S,
        0x54 => keycode::_T,
        0x55 => keycode::_U,
        0x56 => keycode::_V,
        0x57 => keycode::_W,
        0x58 => keycode::_X,
        0x59 => keycode::_Y,
        0x5A => keycode::_Z,
        0x30 => keycode::_0,
        0x31 => keycode::_1,
        0x32 => keycode::_2,
        0x33 => keycode::_3,
        0x34 => keycode::_4,
        0x35 => keycode::_5,
        0x36 => keycode::_6,
        0x37 => keycode::_7,
        0x38 => keycode::_8,
        0x39 => keycode::_9,
        VK_ESCAPE => keycode::ESCAPE,
        VK_LWIN => keycode::LSYSTEM,
        VK_RWIN => keycode::RSYSTEM,
        VK_APPS => keycode::MENU,
        VK_OEM_4 => keycode::LBRACKET,
        VK_OEM_6 => keycode::RBRACKET,
        VK_OEM_1 => keycode::SEMICOLON,
        VK_OEM_COMMA => keycode::COMMA,
        VK_OEM_PERIOD => keycode::PERIOD,
        VK_OEM_7 => keycode::QUOTE,
        VK_OEM_2 => keycode::SLASH,
        VK_OEM_5 => keycode::BACKSLASH,
        VK_OEM_3 => keycode::TILDE,
        VK_OEM_PLUS => keycode::EQUAL,
        VK_OEM_MINUS => keycode::HYPHEN,
        VK_SPACE => keycode::SPACE,
        VK_RETURN => keycode::ENTER,
        VK_BACK => keycode::BACKSPACE,
        VK_TAB => keycode::TAB,
        VK_PRIOR => keycode::PAGEUP,
        VK_NEXT => keycode::PAGEDOWN,
        VK_END => keycode::END,
        VK_HOME => keycode::HOME,
        VK_INSERT => keycode::INSERT,
        VK_DELETE => keycode::DELETE,
        VK_ADD => keycode::ADD,
        VK_SUBTRACT => keycode::SUBTRACT,
        VK_MULTIPLY => keycode::MULTIPLY,
        VK_DIVIDE => keycode::DIVIDE,
        VK_LEFT => keycode::LEFT,
        VK_RIGHT => keycode::RIGHT,
        VK_UP => keycode::UP,
        VK_DOWN => keycode::DOWN,
        VK_NUMPAD0 => keycode::NUMPAD_0,
        VK_NUMPAD1 => keycode::NUMPAD_1,
        VK_NUMPAD2 => keycode::NUMPAD_2,
        VK_NUMPAD3 => keycode::NUMPAD_3,
        VK_NUMPAD4 => keycode::NUMPAD_4,
        VK_NUMPAD5 => keycode::NUMPAD_5,
        VK_NUMPAD6 => keycode::NUMPAD_6,
        VK_NUMPAD7 => keycode::NUMPAD_7,
        VK_NUMPAD8 => keycode::NUMPAD_8,
        VK_NUMPAD9 => keycode::NUMPAD_9,
        VK_F1 => keycode::F1,
        VK_F2 => keycode::F2,
        VK_F3 => keycode::F3,
        VK_F4 => keycode::F4,
        VK_F5 => keycode::F5,
        VK_F6 => keycode::F6,
        VK_F7 => keycode::F7,
        VK_F8 => keycode::F8,
        VK_F9 => keycode::F9,
        VK_F10 => keycode::F10,
        VK_F11 => keycode::F11,
        VK_F12 => keycode::F12,
        VK_F13 => keycode::F13,
        VK_F14 => keycode::F14,
        VK_F15 => keycode::F15,
        VK_PAUSE => keycode::PAUSE,
        _ => keycode::UNKNOWN,
    }
}

#[inline(always)]
pub fn get_mouse_button(message: u32, wparam: WPARAM) -> usize {
    match message {
        WM_LBUTTONDOWN | WM_LBUTTONUP => mouse::BUTTON_LEFT,
        WM_RBUTTONDOWN | WM_RBUTTONUP => mouse::BUTTON_RIGHT,
        WM_MBUTTONDOWN | WM_MBUTTONUP => mouse::BUTTON_MIDDLE,
        WM_XBUTTONDOWN | WM_XBUTTONUP => {
            if hiword(wparam as u64) as u32 == XBUTTON1 {
                mouse::BUTTON_EXTRA_1
            } else {
                mouse::BUTTON_EXTRA_2
            }
        }
        _ => mouse::BUTTON_UNKNOWN,
    }
}

// pub const GIO_CURSOR_NORMAL : i32           = 1;
// pub const GIO_CURSOR_HIDDEN : i32           = 2;
// pub const GIO_CURSOR_DISABLED : i32         = 3;

//TODO: consoles doesnt have this feature. So, think about that...
