// //#define LOWORD(l)           ((WORD)(((DWORD_PTR)(l)) & 0xffff))
// //#define HIWORD(l)           ((WORD)((((DWORD_PTR)(l)) >> 16) & 0xffff))

// use windows_sys::Win32::UI::Input::KeyboardAndMouse::GetKeyState;

// #[inline(always)]
// pub fn hiword(l: u64) -> u16 {
//     ((l >> 16) & 0xffff) as u16
// }
// #[inline(always)]
// pub fn loword(l: u64) -> u16 {
//     (l & 0xffff) as u16
// }

// use windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_CAPITAL;
// use windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_CONTROL;
// use windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_LWIN;
// use windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_MENU;
// use windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_RWIN;
// use windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_SHIFT;

// use crate::common::keymods;

// #[inline(always)]
// pub unsafe fn get_mods() -> usize {
//     let mut mods: usize = 0;
//     if hiword(GetKeyState(VK_MENU as i32) as u64) != 0 {
//         mods = mods | keymods::ALT;
//     }
//     if hiword(GetKeyState(VK_CONTROL as i32) as u64) != 0 {
//         mods = mods | keymods::CONTROL;
//     }
//     if hiword(GetKeyState(VK_SHIFT as i32) as u64) != 0 {
//         mods = mods | keymods::SHIFT;
//     }
//     //TODO: window keyboard key?
//     if hiword((GetKeyState(VK_LWIN as i32) | GetKeyState(VK_RWIN as i32)) as u64) != 0 {
//         mods = mods | keymods::SYSTEM;
//     }
//     if hiword(GetKeyState(VK_CAPITAL as i32) as u64) != 0 {
//         mods = mods | keymods::CAPSLOCK;
//     }
//     mods
// }
