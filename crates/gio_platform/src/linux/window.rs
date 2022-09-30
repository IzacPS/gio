use std::{
    ptr,
    sync::{Arc, Mutex},
};

use gio_input::Input;
// use gio_input::{Input, GIO_INPUT};
use x11_dl::xlib::{self, Button4, Button5, Display};
use xcb::{x, Xid, XidNew};

// use crate::GIO_PLATFORM;

use super::utils::{get_key_from_keysym, get_mods, get_mouse_button};
//TODO: Solve pub problems
struct Atoms {
    pub wm_protocols: x::Atom,
    pub wm_del_window: x::Atom,
    pub wm_state: x::Atom,
    pub wm_state_maxv: x::Atom,
    pub wm_state_maxh: x::Atom,
}

pub struct WindowInterface {
    pub handle: x::Window,
    pub instance: Option<xcb::Connection>,
}

impl WindowInterface {
    pub fn new() -> Self {
        Self {
            handle: unsafe { x::Window::new(0) },
            instance: None,
        }
    }
}

pub struct Window {
    interface: WindowInterface,
    input: Arc<Mutex<Input>>,
    screen: usize,
    atoms: Atoms,
    display: *mut Display,
    xlib: xlib::Xlib,
}

impl Window {
    pub fn new(input: Arc<Mutex<Input>>) -> Self {
        Self {
            interface: WindowInterface::new(),
            screen: 0,
            input: input.clone(),
            atoms: unsafe {
                Atoms {
                    wm_protocols: x::Atom::new(0),
                    wm_del_window: x::Atom::new(0),
                    wm_state: x::Atom::new(0),
                    wm_state_maxv: x::Atom::new(0),
                    wm_state_maxh: x::Atom::new(0),
                }
            },
            display: std::ptr::null_mut(),
            xlib: xlib::Xlib::open().unwrap(),
        }
    }

    pub fn platform_interface(&self) -> &WindowInterface {
        &self.interface
    }

    pub fn platform_interface_mut(&mut self) -> &mut WindowInterface {
        &mut self.interface
    }

    pub fn startup(
        &mut self,
        app_name: String,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> xcb::Result<()> {
        //let xlib = xlib::Xlib::open().unwrap();

        self.display = unsafe { (self.xlib.XOpenDisplay)(ptr::null()) };
        //self.window_interface.display = unsafe { XOpenDisplay(std::ptr::null()) };
        //unsafe{XAutoRepeatOff(self.window_interface.display)};

        //self.window_interface.connection = unsafe{ XGetXCBConnection(self.window_interface.display)};
        //TODO: check if connection has erros
        //if (xcb_connection_has_error(state_ptr->connection))
        //let setup = unsafe{xcb_get_setup(self.window_interface.connection)};
        //let it : xcb_screen_iterator_t;
        // Connect to the X server.
        let (conn, screen_num) = xcb::Connection::connect(None)?;
        self.screen = screen_num as usize;
        //self.window_interface.connection.get_setup();
        // Fetch the `x::Setup` and get the main `x::Screen` object.
        let screen = conn.get_setup().roots().nth(screen_num as usize).unwrap();
        //self.window_interface.screen = screen;

        // Generate an `Xid` for the client window.
        // The type inference is needed here.
        self.platform_interface_mut().handle = conn.generate_id();

        // We can now create a window. For this we pass a `Request`
        // object to the `send_request_checked` method. The method
        // returns a cookie that will be used to check for success.
        let cookie = conn.send_request_checked(&x::CreateWindow {
            depth: x::COPY_FROM_PARENT as u8,
            wid: self.platform_interface().handle,
            parent: screen.root(),
            x: x as i16,
            y: y as i16,
            width: width as u16,
            height: height as u16,
            border_width: 0,
            class: x::WindowClass::InputOutput,
            visual: screen.root_visual(),
            // this list must be in same order than `Cw` enum order
            value_list: &[
                x::Cw::BackPixel(screen.black_pixel()),
                x::Cw::EventMask(
                    x::EventMask::EXPOSURE
                        | x::EventMask::BUTTON_PRESS
                        | x::EventMask::BUTTON_RELEASE
                        | x::EventMask::KEY_PRESS
                        | x::EventMask::KEY_RELEASE
                        | x::EventMask::POINTER_MOTION
                        | x::EventMask::STRUCTURE_NOTIFY,
                ),
            ],
        });
        // We now check if the window creation worked.
        // A cookie can't be cloned; it is moved to the function.
        conn.check_request(cookie)?;

        // Let's change the window title
        let cookie = conn.send_request_checked(&x::ChangeProperty {
            mode: x::PropMode::Replace,
            window: self.platform_interface_mut().handle,
            property: x::ATOM_WM_NAME,
            r#type: x::ATOM_STRING,
            data: app_name.as_bytes(), //b"My XCB Window",
        });
        // And check for success again
        conn.check_request(cookie)?;

        // We now show ("map" in X terminology) the window.
        // This time we do not check for success, so we discard the cookie.
        conn.send_request(&x::MapWindow {
            window: self.platform_interface_mut().handle,
        });

        // We need a few atoms for our application.
        // We send a few requests in a row and wait for the replies after.
        self.atoms = {
            let cookies = (
                conn.send_request(&x::InternAtom {
                    only_if_exists: true,
                    name: b"WM_PROTOCOLS",
                }),
                conn.send_request(&x::InternAtom {
                    only_if_exists: true,
                    name: b"WM_DELETE_WINDOW",
                }),
                conn.send_request(&x::InternAtom {
                    only_if_exists: true,
                    name: b"_NET_WM_STATE",
                }),
                conn.send_request(&x::InternAtom {
                    only_if_exists: true,
                    name: b"_NET_WM_STATE_MAXIMIZED_VERT",
                }),
                conn.send_request(&x::InternAtom {
                    only_if_exists: true,
                    name: b"_NET_WM_STATE_MAXIMIZED_HORZ",
                }),
            );
            Atoms {
                wm_protocols: conn.wait_for_reply(cookies.0)?.atom(),
                wm_del_window: conn.wait_for_reply(cookies.1)?.atom(),
                wm_state: conn.wait_for_reply(cookies.2)?.atom(),
                wm_state_maxv: conn.wait_for_reply(cookies.3)?.atom(),
                wm_state_maxh: conn.wait_for_reply(cookies.4)?.atom(),
            }
        };

        // We now activate the window close event by sending the following request.
        // If we don't do this we can still close the window by clicking on the "x" button,
        // but the event loop is notified through a connection shutdown error.
        conn.check_request(conn.send_request_checked(&x::ChangeProperty {
            mode: x::PropMode::Replace,
            window: self.platform_interface_mut().handle,
            property: self.atoms.wm_protocols,
            r#type: x::ATOM_ATOM,
            data: &[self.atoms.wm_del_window],
        }))?;

        // Previous request was checked, so a flush is not necessary in this case.
        // Otherwise, here is how to perform a connection flush.
        conn.flush()?;

        //let mut maximized = false;
        self.platform_interface_mut().instance = Some(conn);

        Ok(())
    }

    pub fn poll_events(&mut self) {
        if let Some(conn) = &self.platform_interface().instance {
            if let Some(ev) = conn.wait_for_event().ok() {
                match ev {
                    xcb::Event::X(x::Event::KeyPress(event)) => {
                        let key = unsafe {
                            get_key_from_keysym((self.xlib.XkbKeycodeToKeysym)(
                                self.display,
                                event.detail(),
                                0,
                                0,
                            ))
                        };
                        let mods = get_mods(event.state());
                        // let mut input = GIO_INPUT.lock().unwrap();
                        {
                            self.input
                                .lock()
                                .unwrap()
                                .keyboard_mut()
                                .set_press_state(key, true)
                                .set_mods(mods);
                        }
                    }
                    xcb::Event::X(x::Event::KeyRelease(event)) => {
                        let key = unsafe {
                            get_key_from_keysym((self.xlib.XkbKeycodeToKeysym)(
                                self.display,
                                event.detail(),
                                0,
                                0,
                            ))
                        };
                        let mods = get_mods(event.state());
                        // let mut input = GIO_INPUT.lock().unwrap();

                        {
                            self.input
                                .lock()
                                .unwrap()
                                .keyboard_mut()
                                .set_press_state(key, false)
                                .set_mods(mods);
                        }
                        // for e in &self.input.borrow().keyboard.owners_released {
                        //     let item = event::InputEventItem {
                        //         owner: e.clone(),
                        //         event_type: InputEventType::KeyReleasedEvent { keycode: key },
                        //     };
                        //     self.event_system.borrow_mut().push_input_event(item);
                        // }
                        // if event.detail() == 0x3a {
                        //     // The M key was pressed
                        //     // (M only on qwerty keyboards. Keymap support is done
                        //     // with the `xkb` extension and the `xkbcommon-rs` crate)

                        //     // We toggle maximized state, for this we send a message
                        //     // by building a `x::ClientMessageEvent` with the proper
                        //     // atoms and send it to the server.

                        //     // let data = x::ClientMessageData::Data32([
                        //     //     //if maximized { 0 } else { 1 },
                        //     //     if false { 0 } else { 1 },
                        //     //     self.window_interface.atoms.wm_state_maxv.resource_id(),
                        //     //     self.window_interface.atoms.wm_state_maxh.resource_id(),
                        //     //     0,
                        //     //     0,
                        //     // ]);
                        //     // let event = x::ClientMessageEvent::new(self.window_interface.window, self.window_interface.atoms.wm_state, data);
                        //     // let cookie = conn.send_request_checked(&x::SendEvent {
                        //     //     propagate: false,
                        //     //     destination: x::SendEventDest::Window(conn.get_setup().roots().nth(self.window_interface.screen).unwrap().root()),
                        //     //     event_mask: x::EventMask::STRUCTURE_NOTIFY,
                        //     //     event: &event,
                        //     // });
                        //     // conn.check_request(cookie);

                        //     // Same as before, if we don't check for error, we have to flush
                        //     // the connection.
                        //     // conn.flush()?;

                        //     //maximized = !maximized;
                        // } else if event.detail() == 0x18 {
                        //     // Q (on qwerty)

                        //     // We exit the event loop (and the program)
                        //     //break Ok(());
                        // }
                    }
                    xcb::Event::X(x::Event::ButtonPress(event)) => {
                        let button = get_mouse_button(event.detail() as u32);
                        // let mut input = GIO_INPUT.lock().unwrap();
                        {
                            self.input
                                .lock()
                                .unwrap()
                                .mouse_mut()
                                .set_position(event.event_x() as u32, event.event_y() as u32)
                                .set_press_state(button, true);
                        }

                        // self.input.mouse.buttons.pressed.set_event(
                        //     Event::MouseButtonPressedEvent {
                        //         button,
                        //         x: event.event_x() as u16,
                        //         y: event.event_y() as u16,
                        //     },
                        // );
                        // self.event_dispatcher
                        //     .push(&self.input.mouse.buttons.pressed);
                        // for e in &self.input.borrow().mouse.buttons.owners_pressed {
                        //     let item = event::InputEventItem {
                        //         owner: e.clone(),
                        //         event_type: InputEventType::MouseButtonPressedEvent {
                        //             button,
                        //             x: event.event_x() as u16,
                        //             y: event.event_y() as u16,
                        //         },
                        //     };
                        //     self.event_system.borrow_mut().push_input_event(item);
                        // }
                    }
                    xcb::Event::X(x::Event::ButtonRelease(event)) => {
                        let button = get_mouse_button(event.detail() as u32);
                        if button == Button4 as usize || button == Button5 as usize {
                            let delta = if button == Button4 as usize {
                                1.0
                            } else {
                                -1.0
                            };
                            // let mut input = GIO_INPUT.lock().unwrap();

                            {
                                self.input
                                    .lock()
                                    .unwrap()
                                    .mouse_mut()
                                    .set_position(event.event_x() as u32, event.event_y() as u32)
                                    .set_scroll_delta(delta);
                            }
                            // for e in &self.input.borrow().mouse.wheel {
                            //     let item = event::InputEventItem {
                            //         owner: e.clone(),
                            //         event_type: InputEventType::MouseWheelScrolledEvent {
                            //             x: event.event_x() as u16,
                            //             y: event.event_y() as u16,
                            //             delta,
                            //         },
                            //     };
                            //     self.event_system.borrow_mut().push_input_event(item);
                            // }
                        } else if button == 6 || button == 7 {
                            let delta = if button == Button4 as usize {
                                1.0
                            } else {
                                -1.0
                            };
                            //TODO: this is horiontal mouse wheel movement
                            // let mut input = GIO_INPUT.lock().unwrap();
                            {
                                self.input
                                    .lock()
                                    .unwrap()
                                    .mouse_mut()
                                    .set_position(event.event_x() as u32, event.event_y() as u32)
                                    .set_scroll_delta(delta);
                            }
                            // for e in &self.input.borrow().mouse.wheel {
                            //     let item = event::InputEventItem {
                            //         owner: e.clone(),
                            //         event_type: InputEventType::MouseWheelHScrolledEvent {
                            //             x: event.event_x() as u16,
                            //             y: event.event_y() as u16,
                            //             delta,
                            //         },
                            //     };
                            // }
                        } else {
                            // let mut input = GIO_INPUT.lock().unwrap();
                            {
                                self.input
                                    .lock()
                                    .unwrap()
                                    .mouse_mut()
                                    .set_position(event.event_x() as u32, event.event_y() as u32)
                                    .set_press_state(button, false);
                            }
                            // self.input.mouse.buttons.released.set_event(
                            //     Event::MouseButtonReleasedEvent {
                            //         button,
                            //         x: event.event_x() as u16,
                            //         y: event.event_y() as u16,
                            //     },
                            // );
                            // self.event_dispatcher
                            //     .push(&self.input.mouse.buttons.released);
                            // for e in &self.input.borrow().mouse.buttons.owners_released {
                            //     let item = event::InputEventItem {
                            //         owner: e.clone(),
                            //         event_type: InputEventType::MouseButtonReleasedEvent {
                            //             button,
                            //             x: event.event_x() as u16,
                            //             y: event.event_y() as u16,
                            //         },
                            //     };
                            //     self.event_system.borrow_mut().push_input_event(item);
                            // }
                        }
                    }
                    xcb::Event::X(x::Event::MotionNotify(event)) => {
                        // let mut input = GIO_INPUT.lock().unwrap();
                        {
                            self.input
                                .lock()
                                .unwrap()
                                .mouse_mut()
                                .set_position(event.event_x() as u32, event.event_y() as u32);
                        }
                        // self.input.mouse.movement.set_event(Event::MouseMovedEvent {
                        //     x: event.event_x() as u16,
                        //     y: event.event_y() as u16,
                        // });
                        // self.event_dispatcher.push(&self.input.mouse.movement);
                        // for e in &self.input.borrow().mouse.movement {
                        //     let item = event::InputEventItem {
                        //         owner: e.clone(),
                        //         event_type: InputEventType::MouseMovedEvent {
                        //             x: event.event_x() as u16,
                        //             y: event.event_y() as u16,
                        //         },
                        //     };
                        //     self.event_system.borrow_mut().push_input_event(item);
                        // }
                    }
                    xcb::Event::X(x::Event::ConfigureNotify(event)) => {}
                    xcb::Event::X(x::Event::ClientMessage(ev)) => {
                        // We have received a message from the server
                        if let x::ClientMessageData::Data32([atom, ..]) = ev.data() {
                            if atom == self.atoms.wm_del_window.resource_id() {
                                // The received atom is "WM_DELETE_WINDOW".
                                // We can check here if the user needs to save before
                                // exit, or in our case, exit right away.
                                //break Ok(());
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
