// use std::{cell::RefCell, rc::Rc};

// use gio_event::{dispatcher::EventDispatcher, event::Event};
// use windows_sys::{
//     Win32::Foundation::*, Win32::System::LibraryLoader::*, Win32::UI::WindowsAndMessaging::*,
// };

// use crate::input::Input;

// use super::{
//     input::{get_key, get_mouse_button},
//     utils::{get_mods, hiword, loword},
// };

// pub struct WindowInterface {
//     instance: HINSTANCE,
//     handle: HANDLE,
// }

// impl WindowInterface {
//     pub fn new() -> Self {
//         Self {
//             instance: 0,
//             handle: 0,
//         }
//     }
// }

// pub struct Window<'a> {
//     //TODO: change windowhandle rc to arc and mutex
//     window_interface: WindowInterface,
//     pub input: Rc<RefCell<Input>>,
//     pub event_dispatcher: Rc<RefCell<EventDispatcher<'a>>>,
//     pub has_requested_to_close: bool,
// }

// impl<'a> Window<'a> {
//     pub fn new(
//         input: Rc<RefCell<Input>>,
//         event_dispatcher: Rc<RefCell<EventDispatcher<'a>>>,
//     ) -> Self {
//         Self {
//             window_interface: WindowInterface::new(),
//             input,
//             event_dispatcher,
//             has_requested_to_close: false,
//         }
//     }

//     pub fn window_interface(&self) -> &WindowInterface {
//         &self.window_interface
//     }

//     pub fn window_interface_mut(&mut self) -> &mut WindowInterface {
//         &mut self.window_interface
//     }

//     pub fn startup(
//         &mut self,
//         app_name: String,
//         x: i32,
//         y: i32,
//         width: i32,
//         height: i32,
//     ) -> Option<()> {
//         unsafe {
//             self.window_interface_mut().instance = GetModuleHandleA(std::ptr::null());
//             let wc = WNDCLASSA {
//                 style: CS_DBLCLKS,
//                 lpfnWndProc: Some(Self::wndproc),
//                 hCursor: LoadCursorW(0, IDC_ARROW),
//                 hInstance: self.window_interface().instance,
//                 lpszClassName: "gioengine_win32_window".as_ptr(),
//                 cbClsExtra: 0,
//                 cbWndExtra: 0,
//                 hIcon: LoadIconW(self.window_interface().instance, IDI_APPLICATION),
//                 hbrBackground: 0,
//                 lpszMenuName: std::ptr::null(),
//             };

//             if RegisterClassA(&wc) == 0 {
//                 None
//             } else {
//                 let mut windowx = x;
//                 let mut windowy = y;
//                 let mut window_width = width;
//                 let mut window_height = height;

//                 let window_style = WS_OVERLAPPED
//                     | WS_SYSMENU
//                     | WS_CAPTION
//                     | WS_MAXIMIZEBOX
//                     | WS_MINIMIZEBOX
//                     | WS_THICKFRAME;

//                 let mut border = RECT {
//                     left: 0,
//                     right: 0,
//                     top: 0,
//                     bottom: 0,
//                 };
//                 AdjustWindowRectEx(&mut border, window_style, 0, WS_EX_APPWINDOW);

//                 windowx += border.left;
//                 windowy += border.top;
//                 window_width += border.right - border.left;
//                 window_height += border.bottom - border.top;

//                 let instance = self.window_interface().instance;
//                 self.window_interface_mut().handle = CreateWindowExA(
//                     WS_EX_APPWINDOW,
//                     "gioengine_win32_window".as_ptr(),
//                     app_name.as_ptr(),
//                     window_style,
//                     windowx,
//                     windowy,
//                     window_width,
//                     window_height,
//                     0,
//                     0,
//                     instance,
//                     self as *mut _ as _,
//                 );
//                 if self.window_interface().handle == 0 {
//                     None
//                 } else {
//                     //         //let mut frequency = std::mem::zeroed();
//                     //         //QueryPerformanceFrequency(&mut frequency);
//                     //         //crate::platform::time::CLOCK_FREQUENCY = 1.0 / (frequency as f64);
//                     ShowWindow(self.window_interface().handle, SW_SHOW);
//                     Some(())
//                 }
//             }
//         }
//     }

//     pub fn poll_events(&self) {
//         unsafe {
//             let mut msg = std::mem::zeroed();

//             while PeekMessageA(&mut msg, 0, 0, 0, PM_REMOVE) != 0 {
//                 TranslateMessage(&msg);
//                 DispatchMessageA(&msg);
//             }
//         }
//     }

//     extern "system" fn wndproc(
//         window: HWND,
//         message: u32,
//         wparam: WPARAM,
//         lparam: LPARAM,
//     ) -> LRESULT {
//         unsafe {
//             if message == WM_NCCREATE {
//                 let cs = lparam as *const CREATESTRUCTA;
//                 let this = (*cs).lpCreateParams as *mut Self;
//                 SetWindowLongPtrW(window, GWLP_USERDATA, this as _);
//             } else {
//                 let ptr = GetWindowLongPtrW(window, GWLP_USERDATA);
//                 let this = ptr as *mut Self;
//                 if !this.is_null() {
//                     return (*this).handle_messages(window, message, wparam, lparam);
//                 }
//             }
//             DefWindowProcA(window, message, wparam, lparam)
//         }
//     }

//     fn handle_messages(
//         &'a mut self,
//         window: HWND,
//         message: u32,
//         wparam: WPARAM,
//         lparam: LPARAM,
//     ) -> LRESULT {
//         unsafe {
//             match message {
//                 WM_CLOSE => {
//                     self.has_requested_to_close = true;
//                     0
//                 }
//                 WM_KEYDOWN | WM_SYSKEYDOWN => {
//                     let key = get_key(wparam, lparam);
//                     let mods = get_mods();
//                     if (hiword(lparam as u64) as u32 & KF_REPEAT) == 0 {
//                         // self.input
//                         //     .borrow_mut()
//                         //     .keyboard
//                         //     .pressed
//                         //     .set_event(Event::KeyPressedEvent { keycode: key, mods });
//                         self.event_dispatcher
//                             .borrow_mut()
//                             .push(&self.input.borrow().keyboard.pressed);
//                     } else {
//                         // self.input
//                         //     .keyboard
//                         //     .pressed
//                         //     .set_event(Event::KeyPressedRepeatingEvent { keycode: key, mods });
//                         // self.event_dispatcher.push(&self.input.keyboard.pressed);
//                     };

//                     0
//                 }
//                 WM_KEYUP | WM_SYSKEYUP => {
//                     //let mods = get_mods();
//                     let key = get_key(wparam, lparam);
//                     // self.input
//                     //     .keyboard
//                     //     .released
//                     //     .set_event(Event::KeyReleasedEvent { keycode: key });
//                     // self.event_dispatcher.push(&self.input.keyboard.released);

//                     0
//                 }
//                 WM_LBUTTONDOWN | WM_RBUTTONDOWN | WM_MBUTTONDOWN | WM_XBUTTONDOWN => {
//                     let mbutton = get_mouse_button(message, wparam);
//                     let mousex = loword(lparam as u64);
//                     let mousey = hiword(lparam as u64);
//                     // self.input
//                     //     .mouse
//                     //     .buttons
//                     //     .pressed
//                     //     .set_event(Event::MouseButtonPressedEvent {
//                     //         button: mbutton,
//                     //         x: mousex,
//                     //         y: mousey,
//                     //     });
//                     // self.event_dispatcher
//                     //     .push(&self.input.mouse.buttons.pressed);

//                     0
//                 }
//                 WM_LBUTTONUP | WM_MBUTTONUP | WM_RBUTTONUP | WM_XBUTTONUP => {
//                     let mbutton = get_mouse_button(message, wparam);
//                     let mousex = loword(lparam as u64);
//                     let mousey = hiword(lparam as u64);
//                     // self.input
//                     //     .mouse
//                     //     .buttons
//                     //     .released
//                     //     .set_event(Event::MouseButtonReleasedEvent {
//                     //         button: mbutton,
//                     //         x: mousex,
//                     //         y: mousey,
//                     //     });
//                     // self.event_dispatcher
//                     //     .push(&self.input.mouse.buttons.released);

//                     0
//                 }
//                 WM_MOUSEMOVE => {
//                     let mousex = loword(lparam as u64);
//                     let mousey = hiword(lparam as u64);
//                     // self.input.mouse.movement.set_event(Event::MouseMovedEvent {
//                     //     x: mousex,
//                     //     y: mousey,
//                     // });
//                     // self.event_dispatcher.push(&self.input.mouse.movement);

//                     0
//                 }
//                 WM_MOUSEWHEEL => {
//                     let mousex = loword(lparam as u64);
//                     let mousey = hiword(lparam as u64);
//                     let delta = hiword(wparam as u64) as f64 / 120f64;
//                     // self.input
//                     //     .mouse
//                     //     .wheel
//                     //     .set_event(Event::MouseWheelScrolledEvent {
//                     //         x: mousex,
//                     //         y: mousey,
//                     //         delta,
//                     //     });
//                     // self.event_dispatcher.push(&self.input.mouse.wheel);

//                     0
//                 }
//                 WM_MOUSEHWHEEL => {
//                     let mousex = loword(lparam as u64);
//                     let mousey = hiword(lparam as u64);
//                     let mut delta = hiword(wparam as u64) as f64 / 120f64;
//                     delta = -delta;
//                     // self.input
//                     //     .mouse
//                     //     .wheel
//                     //     .set_event(Event::MouseWheelScrolledEvent {
//                     //         x: mousex,
//                     //         y: mousey,
//                     //         delta,
//                     //     });
//                     // self.event_dispatcher.push(&self.input.mouse.wheel);

//                     0
//                 }
//                 WM_SIZE => {
//                     return 0;
//                 }
//                 _ => DefWindowProcA(window, message, wparam, lparam),
//             }
//         }
//     }
// }
