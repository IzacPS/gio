use gio_input::keycode::{keyboard, mouse};
use gio_input::keymod::Keymod;
use x11_dl::xlib::KeySym;
use x11_dl::xlib::{ControlMask, LockMask, Mod1Mask, Mod4Mask, ShiftMask};
use xcb::x::KeyButMask;

#[allow(non_upper_case_globals)]
pub fn get_key_from_keysym(symbol: KeySym) -> usize {
    // clang-format off
    match symbol as u32 {
        XK_Shift_L => keyboard::L_SHIFT,
        XK_Shift_R => keyboard::R_SHIFT,
        XK_Control_L => keyboard::L_CONTROL,
        XK_Control_R => keyboard::R_CONTROL,
        XK_Alt_L => keyboard::L_ALT,
        XK_Alt_R => keyboard::R_ALT,
        XK_Super_L => keyboard::L_SYSTEM,
        XK_Super_R => keyboard::R_SYSTEM,
        XK_Menu => keyboard::MENU,
        XK_Escape => keyboard::ESCAPE,
        XK_semicolon => keyboard::SEMICOLON,
        XK_slash => keyboard::SLASH,
        XK_equal => keyboard::EQUAL,
        XK_minus => keyboard::HYPHEN,
        XK_bracketleft => keyboard::L_BRACKET,
        XK_bracketright => keyboard::R_BRACKET,
        XK_comma => keyboard::COMMA,
        XK_period => keyboard::PERIOD,
        XK_apostrophe => keyboard::QUOTE,
        XK_backslash => keyboard::BACKSLASH,
        XK_grave => keyboard::TILDE,
        XK_space => keyboard::SPACE,
        XK_Return => keyboard::ENTER,
        XK_KP_Enter => keyboard::ENTER,
        XK_BackSpace => keyboard::BACKSPACE,
        XK_Tab => keyboard::TAB,
        XK_Prior => keyboard::PAGEUP,
        XK_Next => keyboard::PAGEDOWN,
        XK_End => keyboard::END,
        XK_Home => keyboard::HOME,
        XK_Insert => keyboard::INSERT,
        XK_Delete => keyboard::DELETE,
        XK_KP_Add => keyboard::ADD,
        XK_KP_Subtract => keyboard::SUBTRACT,
        XK_KP_Multiply => keyboard::MULTIPLY,
        XK_KP_Divide => keyboard::DIVIDE,
        XK_Pause => keyboard::PAUSE,
        XK_F1 => keyboard::F1,
        XK_F2 => keyboard::F2,
        XK_F3 => keyboard::F3,
        XK_F4 => keyboard::F4,
        XK_F5 => keyboard::F5,
        XK_F6 => keyboard::F6,
        XK_F7 => keyboard::F7,
        XK_F8 => keyboard::F8,
        XK_F9 => keyboard::F9,
        XK_F10 => keyboard::F10,
        XK_F11 => keyboard::F11,
        XK_F12 => keyboard::F12,
        XK_F13 => keyboard::F13,
        XK_F14 => keyboard::F14,
        XK_F15 => keyboard::F15,
        XK_Left => keyboard::LEFT,
        XK_Right => keyboard::RIGHT,
        XK_Up => keyboard::UP,
        XK_Down => keyboard::DOWN,
        XK_KP_Insert => keyboard::NUMPAD_0,
        XK_KP_End => keyboard::NUMPAD_1,
        XK_KP_Down => keyboard::NUMPAD_2,
        XK_KP_Page_Down => keyboard::NUMPAD_3,
        XK_KP_Left => keyboard::NUMPAD_4,
        XK_KP_Begin => keyboard::NUMPAD_5,
        XK_KP_Right => keyboard::NUMPAD_6,
        XK_KP_Home => keyboard::NUMPAD_7,
        XK_KP_Up => keyboard::NUMPAD_8,
        XK_KP_Page_Up => keyboard::NUMPAD_9,
        XK_a => keyboard::KEY_A,
        XK_b => keyboard::KEY_B,
        XK_c => keyboard::KEY_C,
        XK_d => keyboard::KEY_D,
        XK_e => keyboard::KEY_E,
        XK_f => keyboard::KEY_F,
        XK_g => keyboard::KEY_G,
        XK_h => keyboard::KEY_H,
        XK_i => keyboard::KEY_I,
        XK_j => keyboard::KEY_J,
        XK_k => keyboard::KEY_K,
        XK_l => keyboard::KEY_L,
        XK_m => keyboard::KEY_M,
        XK_n => keyboard::KEY_N,
        XK_o => keyboard::KEY_O,
        XK_p => keyboard::KEY_P,
        XK_q => keyboard::KEY_Q,
        XK_r => keyboard::KEY_R,
        XK_s => keyboard::KEY_S,
        XK_t => keyboard::KEY_T,
        XK_u => keyboard::KEY_U,
        XK_v => keyboard::KEY_V,
        XK_w => keyboard::KEY_W,
        XK_x => keyboard::KEY_X,
        XK_y => keyboard::KEY_Y,
        XK_z => keyboard::KEY_Z,
        XK_0 => keyboard::KEY_0,
        XK_1 => keyboard::KEY_1,
        XK_2 => keyboard::KEY_2,
        XK_3 => keyboard::KEY_3,
        XK_4 => keyboard::KEY_4,
        XK_5 => keyboard::KEY_5,
        XK_6 => keyboard::KEY_6,
        XK_7 => keyboard::KEY_7,
        XK_8 => keyboard::KEY_8,
        XK_9 => keyboard::KEY_9,
        _ => keyboard::UNKNOWN,
    }
    // clang-format on
}

#[allow(non_upper_case_globals)]
pub fn get_mouse_button(detail: u32) -> usize {
    match detail {
        Button1 => mouse::BUTTON_LEFT,
        Button2 => mouse::BUTTON_MIDDLE,
        Button3 => mouse::BUTTON_RIGHT,
        8 => mouse::BUTTON_EXTRA_1,
        9 => mouse::BUTTON_EXTRA_2,
        _ => mouse::BUTTON_UNKNOWN,
    }
}

#[inline(always)]
pub fn get_mods(state: KeyButMask) -> Keymod {
    let mut mods = Keymod::default();
    if (state.bits() & Mod1Mask) != 0 {
        mods.set_alt();
    }
    if (state.bits() & ControlMask) != 0 {
        mods.set_control();
    }
    if (state.bits() & ShiftMask) != 0 {
        mods.set_shift();
    }
    //TODO: window keyboard key?
    if (state.bits() & Mod4Mask) != 0 {
        mods.set_system();
    }
    if (state.bits() & LockMask) != 0 {
        mods.set_capslock();
    }
    mods
}
