use x11_dl::xlib::{ControlMask, LockMask, Mod1Mask, Mod4Mask, ShiftMask};
use xcb::x::KeyButMask;

use crate::common::keymods;

#[inline(always)]
pub fn get_mods(state: KeyButMask) -> usize {
    let mut mods: usize = 0;
    if (state.bits() & Mod1Mask) != 0 {
        mods = mods | keymods::ALT;
    }
    if (state.bits() & ControlMask) != 0 {
        mods = mods | keymods::CONTROL;
    }
    if (state.bits() & ShiftMask) != 0 {
        mods = mods | keymods::SHIFT;
    }
    //TODO: window keyboard key?
    if (state.bits() & Mod4Mask) != 0 {
        mods = mods | keymods::SYSTEM;
    }
    if (state.bits() & LockMask) != 0 {
        mods = mods | keymods::CAPSLOCK;
    }
    mods
}
