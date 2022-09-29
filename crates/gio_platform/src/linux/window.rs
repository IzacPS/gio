use std::ptr;

use crate::input::Input;
use gio_event::{dispatcher::EventDispatcher, event::Event};
use x11_dl::xlib::{self, Button4, Button5, Display};
use xcb::{x, Xid, XidNew};

use super::{
    input::{get_key_from_keysym, get_mouse_button},
    utils::get_mods,
};
//TODO: Solve pub problems
pub struct Atoms {
    pub wm_protocols: x::Atom,
    pub wm_del_window: x::Atom,
    pub wm_state: x::Atom,
    pub wm_state_maxv: x::Atom,
    pub wm_state_maxh: x::Atom,
}

pub struct WindowInterface {
    pub window: x::Window,
    pub connection: Option<xcb::Connection>,
    pub screen: usize,
    pub atoms: Atoms,
    pub display: *mut Display,
    pub xlib: xlib::Xlib,
}

impl WindowInterface {
    pub fn new() -> Self {
        Self {
            window: unsafe { x::Window::new(0) },
            connection: None,
            screen: 0,
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
}

pub struct Window<'a> {
    window_interface: WindowInterface,
    pub input: &'a mut Input,
    pub event_dispatcher: &'a mut EventDispatcher<'a>,
    pub has_requested_to_close: bool,
}

impl<'a> Window<'a> {
    pub fn new(input: &'a mut Input, event_dispatcher: &'a mut EventDispatcher<'a>) -> Self {
        Self {
            window_interface: WindowInterface::new(),
            input,
            event_dispatcher,
            has_requested_to_close: false,
        }
    }

    pub fn window_interface(&self) -> &WindowInterface {
        &self.window_interface
    }

    pub fn window_interface_mut(&mut self) -> &mut WindowInterface {
        &mut self.window_interface
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

        self.window_interface_mut().display =
            unsafe { (self.window_interface.xlib.XOpenDisplay)(ptr::null()) };
        //self.window_interface.display = unsafe { XOpenDisplay(std::ptr::null()) };
        //unsafe{XAutoRepeatOff(self.window_interface.display)};

        //self.window_interface.connection = unsafe{ XGetXCBConnection(self.window_interface.display)};
        //TODO: check if connection has erros
        //if (xcb_connection_has_error(state_ptr->connection))
        //let setup = unsafe{xcb_get_setup(self.window_interface.connection)};
        //let it : xcb_screen_iterator_t;
        // Connect to the X server.
        let (conn, screen_num) = xcb::Connection::connect(None)?;
        self.window_interface_mut().screen = screen_num as usize;
        //self.window_interface.connection.get_setup();
        // Fetch the `x::Setup` and get the main `x::Screen` object.
        let screen = conn.get_setup().roots().nth(screen_num as usize).unwrap();
        //self.window_interface.screen = screen;

        // Generate an `Xid` for the client window.
        // The type inference is needed here.
        let window: x::Window = conn.generate_id();

        // We can now create a window. For this we pass a `Request`
        // object to the `send_request_checked` method. The method
        // returns a cookie that will be used to check for success.
        let cookie = conn.send_request_checked(&x::CreateWindow {
            depth: x::COPY_FROM_PARENT as u8,
            wid: window,
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
            window,
            property: x::ATOM_WM_NAME,
            r#type: x::ATOM_STRING,
            data: app_name.as_bytes(), //b"My XCB Window",
        });
        // And check for success again
        conn.check_request(cookie)?;

        // We now show ("map" in X terminology) the window.
        // This time we do not check for success, so we discard the cookie.
        conn.send_request(&x::MapWindow { window });

        // We need a few atoms for our application.
        // We send a few requests in a row and wait for the replies after.
        self.window_interface_mut().atoms = {
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
            window,
            property: self.window_interface().atoms.wm_protocols,
            r#type: x::ATOM_ATOM,
            data: &[self.window_interface().atoms.wm_del_window],
        }))?;

        // Previous request was checked, so a flush is not necessary in this case.
        // Otherwise, here is how to perform a connection flush.
        conn.flush()?;

        //let mut maximized = false;
        self.window_interface_mut().connection = Some(conn);

        Ok(())
    }

    pub fn poll_events(&'a mut self) {
        if let Some(conn) = &self.window_interface().connection {
            if let Some(ev) = conn.wait_for_event().ok() {
                match ev {
                    xcb::Event::X(x::Event::KeyPress(event)) => {
                        let key = unsafe {
                            get_key_from_keysym((self.window_interface.xlib.XkbKeycodeToKeysym)(
                                self.window_interface().display,
                                event.detail(),
                                0,
                                0,
                            ))
                        };
                        let mods = get_mods(event.state());
                        self.input
                            .keyboard
                            .pressed
                            .set_event(Event::KeyPressedEvent { keycode: key, mods });
                        self.event_dispatcher.push(&self.input.keyboard.pressed);
                        // for e in &self.input.borrow().keyboard.owners_pressed {
                        //     let item = event::InputEventItem {
                        //         owner: e.clone(),
                        //         event_type: InputEventType::KeyPressedEvent { keycode: key, mods },
                        //     };
                        //     self.event_system.borrow_mut().push_input_event(item);
                        // }
                    }
                    xcb::Event::X(x::Event::KeyRelease(event)) => {
                        let key = unsafe {
                            get_key_from_keysym((self.window_interface.xlib.XkbKeycodeToKeysym)(
                                self.window_interface().display,
                                event.detail(),
                                0,
                                0,
                            ))
                        };
                        let mods = get_mods(event.state());
                        self.input
                            .keyboard
                            .released
                            .set_event(Event::KeyReleasedEvent { keycode: key });
                        self.event_dispatcher.push(&self.input.keyboard.released);
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
                        self.input.mouse.buttons.pressed.set_event(
                            Event::MouseButtonPressedEvent {
                                button,
                                x: event.event_x() as u16,
                                y: event.event_y() as u16,
                            },
                        );
                        self.event_dispatcher
                            .push(&self.input.mouse.buttons.pressed);
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
                            self.input
                                .mouse
                                .wheel
                                .set_event(Event::MouseWheelScrolledEvent {
                                    x: event.event_x() as u16,
                                    y: event.event_y() as u16,
                                    delta,
                                });
                            self.event_dispatcher.push(&self.input.mouse.wheel);
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
                            self.input
                                .mouse
                                .wheel
                                .set_event(Event::MouseWheelHScrolledEvent {
                                    x: event.event_x() as u16,
                                    y: event.event_y() as u16,
                                    delta,
                                });
                            self.event_dispatcher.push(&self.input.mouse.wheel);
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
                            self.input.mouse.buttons.released.set_event(
                                Event::MouseButtonReleasedEvent {
                                    button,
                                    x: event.event_x() as u16,
                                    y: event.event_y() as u16,
                                },
                            );
                            self.event_dispatcher
                                .push(&self.input.mouse.buttons.released);
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
                        self.input.mouse.movement.set_event(Event::MouseMovedEvent {
                            x: event.event_x() as u16,
                            y: event.event_y() as u16,
                        });
                        self.event_dispatcher.push(&self.input.mouse.movement);
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
                            if atom == self.window_interface().atoms.wm_del_window.resource_id() {
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
