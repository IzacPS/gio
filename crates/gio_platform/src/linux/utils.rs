use gio_input::Keymod;
use gio_input::{keycode, mouse};
use x11_dl::xlib::KeySym;
use x11_dl::xlib::{ControlMask, LockMask, Mod1Mask, Mod4Mask, ShiftMask};
use xcb::x::KeyButMask;

#[allow(non_upper_case_globals)]
pub fn get_key_from_keysym(symbol: KeySym) -> usize {
    // clang-format off
    match symbol as u32 {
        XK_Shift_L => keycode::L_SHIFT,
        XK_Shift_R => keycode::R_SHIFT,
        XK_Control_L => keycode::L_CONTROL,
        XK_Control_R => keycode::R_CONTROL,
        XK_Alt_L => keycode::L_ALT,
        XK_Alt_R => keycode::R_ALT,
        XK_Super_L => keycode::L_SYSTEM,
        XK_Super_R => keycode::R_SYSTEM,
        XK_Menu => keycode::MENU,
        XK_Escape => keycode::ESCAPE,
        XK_semicolon => keycode::SEMICOLON,
        XK_slash => keycode::SLASH,
        XK_equal => keycode::EQUAL,
        XK_minus => keycode::HYPHEN,
        XK_bracketleft => keycode::L_BRACKET,
        XK_bracketright => keycode::R_BRACKET,
        XK_comma => keycode::COMMA,
        XK_period => keycode::PERIOD,
        XK_apostrophe => keycode::QUOTE,
        XK_backslash => keycode::BACKSLASH,
        XK_grave => keycode::TILDE,
        XK_space => keycode::SPACE,
        XK_Return => keycode::ENTER,
        XK_KP_Enter => keycode::ENTER,
        XK_BackSpace => keycode::BACKSPACE,
        XK_Tab => keycode::TAB,
        XK_Prior => keycode::PAGEUP,
        XK_Next => keycode::PAGEDOWN,
        XK_End => keycode::END,
        XK_Home => keycode::HOME,
        XK_Insert => keycode::INSERT,
        XK_Delete => keycode::DELETE,
        XK_KP_Add => keycode::ADD,
        XK_KP_Subtract => keycode::SUBTRACT,
        XK_KP_Multiply => keycode::MULTIPLY,
        XK_KP_Divide => keycode::DIVIDE,
        XK_Pause => keycode::PAUSE,
        XK_F1 => keycode::F1,
        XK_F2 => keycode::F2,
        XK_F3 => keycode::F3,
        XK_F4 => keycode::F4,
        XK_F5 => keycode::F5,
        XK_F6 => keycode::F6,
        XK_F7 => keycode::F7,
        XK_F8 => keycode::F8,
        XK_F9 => keycode::F9,
        XK_F10 => keycode::F10,
        XK_F11 => keycode::F11,
        XK_F12 => keycode::F12,
        XK_F13 => keycode::F13,
        XK_F14 => keycode::F14,
        XK_F15 => keycode::F15,
        XK_Left => keycode::LEFT,
        XK_Right => keycode::RIGHT,
        XK_Up => keycode::UP,
        XK_Down => keycode::DOWN,
        XK_KP_Insert => keycode::NUMPAD_0,
        XK_KP_End => keycode::NUMPAD_1,
        XK_KP_Down => keycode::NUMPAD_2,
        XK_KP_Page_Down => keycode::NUMPAD_3,
        XK_KP_Left => keycode::NUMPAD_4,
        XK_KP_Begin => keycode::NUMPAD_5,
        XK_KP_Right => keycode::NUMPAD_6,
        XK_KP_Home => keycode::NUMPAD_7,
        XK_KP_Up => keycode::NUMPAD_8,
        XK_KP_Page_Up => keycode::NUMPAD_9,
        XK_a => keycode::KEY_A,
        XK_b => keycode::KEY_B,
        XK_c => keycode::KEY_C,
        XK_d => keycode::KEY_D,
        XK_e => keycode::KEY_E,
        XK_f => keycode::KEY_F,
        XK_g => keycode::KEY_G,
        XK_h => keycode::KEY_H,
        XK_i => keycode::KEY_I,
        XK_j => keycode::KEY_J,
        XK_k => keycode::KEY_K,
        XK_l => keycode::KEY_L,
        XK_m => keycode::KEY_M,
        XK_n => keycode::KEY_N,
        XK_o => keycode::KEY_O,
        XK_p => keycode::KEY_P,
        XK_q => keycode::KEY_Q,
        XK_r => keycode::KEY_R,
        XK_s => keycode::KEY_S,
        XK_t => keycode::KEY_T,
        XK_u => keycode::KEY_U,
        XK_v => keycode::KEY_V,
        XK_w => keycode::KEY_W,
        XK_x => keycode::KEY_X,
        XK_y => keycode::KEY_Y,
        XK_z => keycode::KEY_Z,
        XK_0 => keycode::KEY_0,
        XK_1 => keycode::KEY_1,
        XK_2 => keycode::KEY_2,
        XK_3 => keycode::KEY_3,
        XK_4 => keycode::KEY_4,
        XK_5 => keycode::KEY_5,
        XK_6 => keycode::KEY_6,
        XK_7 => keycode::KEY_7,
        XK_8 => keycode::KEY_8,
        XK_9 => keycode::KEY_9,
        _ => keycode::UNKNOWN,
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
