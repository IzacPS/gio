//#define LOWORD(l)           ((WORD)(((DWORD_PTR)(l)) & 0xffff))
//#define HIWORD(l)           ((WORD)((((DWORD_PTR)(l)) >> 16) & 0xffff))

use gio_input::keycode;
use gio_input::keymods;
use gio_input::mouse;
use gio_input::Keymod;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::GetKeyState;

#[inline(always)]
pub fn hiword(l: u64) -> u16 {
    ((l >> 16) & 0xffff) as u16
}
#[inline(always)]
pub fn loword(l: u64) -> u16 {
    (l & 0xffff) as u16
}

use windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_CAPITAL;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_CONTROL;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_LWIN;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_MENU;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_RWIN;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_SHIFT;

#[inline(always)]
pub unsafe fn get_mods() -> Keymod {
    let mut mods = Keymod::default();
    if hiword(GetKeyState(VK_MENU as i32) as u64) != 0 {
        mods.set_alt();
    }
    if hiword(GetKeyState(VK_CONTROL as i32) as u64) != 0 {
        mods.set_control();
    }
    if hiword(GetKeyState(VK_SHIFT as i32) as u64) != 0 {
        mods.set_shift();
    }
    //TODO: window keyboard key?
    if hiword((GetKeyState(VK_LWIN as i32) | GetKeyState(VK_RWIN as i32)) as u64) != 0 {
        mods.set_system();
    }
    if hiword(GetKeyState(VK_CAPITAL as i32) as u64) != 0 {
        mods.set_capslock();
    }
    mods
}

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

pub fn get_key(key: WPARAM, flags: LPARAM) -> usize {
    match key as u16 {
        VK_SHIFT => {
            let lshift = unsafe { MapVirtualKeyW(VK_LSHIFT as u32, MAPVK_VK_TO_VSC) as isize };
            let scancode = (flags & (0xFF << 16)) >> 16;
            if lshift == scancode {
                keycode::L_SHIFT
            } else {
                keycode::R_SHIFT
            }
        }
        VK_MENU => {
            if (hiword(flags as u64) & (KF_EXTENDED as u16)) != 0 {
                keycode::R_ALT
            } else {
                keycode::L_ALT
            }
        }
        VK_CONTROL => {
            if (hiword(flags as u64) & (KF_EXTENDED as u16)) != 0 {
                keycode::R_CONTROL
            } else {
                keycode::L_CONTROL
            }
        }
        0x41 => keycode::KEY_A,
        0x42 => keycode::KEY_B,
        0x43 => keycode::KEY_C,
        0x44 => keycode::KEY_D,
        0x45 => keycode::KEY_E,
        0x46 => keycode::KEY_F,
        0x47 => keycode::KEY_G,
        0x48 => keycode::KEY_H,
        0x49 => keycode::KEY_I,
        0x4A => keycode::KEY_J,
        0x4B => keycode::KEY_K,
        0x4C => keycode::KEY_L,
        0x4D => keycode::KEY_M,
        0x4E => keycode::KEY_N,
        0x4F => keycode::KEY_O,
        0x50 => keycode::KEY_P,
        0x51 => keycode::KEY_Q,
        0x52 => keycode::KEY_R,
        0x53 => keycode::KEY_S,
        0x54 => keycode::KEY_T,
        0x55 => keycode::KEY_U,
        0x56 => keycode::KEY_V,
        0x57 => keycode::KEY_W,
        0x58 => keycode::KEY_X,
        0x59 => keycode::KEY_Y,
        0x5A => keycode::KEY_Z,
        0x30 => keycode::KEY_0,
        0x31 => keycode::KEY_1,
        0x32 => keycode::KEY_2,
        0x33 => keycode::KEY_3,
        0x34 => keycode::KEY_4,
        0x35 => keycode::KEY_5,
        0x36 => keycode::KEY_6,
        0x37 => keycode::KEY_7,
        0x38 => keycode::KEY_8,
        0x39 => keycode::KEY_9,
        VK_ESCAPE => keycode::ESCAPE,
        VK_LWIN => keycode::L_SYSTEM,
        VK_RWIN => keycode::R_SYSTEM,
        VK_APPS => keycode::MENU,
        VK_OEM_4 => keycode::L_BRACKET,
        VK_OEM_6 => keycode::R_BRACKET,
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
