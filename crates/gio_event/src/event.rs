// // Closed,                 //!< The window requested to be closed (no data)
// // Resized,                //!< The window was resized (data in event.size)
// // LostFocus,              //!< The window lost the focus (no data)
// // GainedFocus,            //!< The window gained the focus (no data)
// // TextEntered,            //!< A character was entered (data in event.text)
// // KeyPressed,             //!< A key was pressed (data in event.key)
// // KeyReleased,            //!< A key was released (data in event.key)
// // MouseWheelScrolled,     //!< The mouse wheel was scrolled (data in event.mouseWheelScroll)
// // MouseButtonPressed,     //!< A mouse button was pressed (data in event.mouseButton)
// // MouseButtonReleased,    //!< A mouse button was released (data in event.mouseButton)
// // MouseMoved,             //!< The mouse cursor moved (data in event.mouseMove)
// // MouseEntered,           //!< The mouse cursor entered the area of the window (no data)
// // MouseLeft,              //!< The mouse cursor left the area of the window (no data)
// // JoystickButtonPressed,  //!< A joystick button was pressed (data in event.joystickButton)
// // JoystickButtonReleased, //!< A joystick button was released (data in event.joystickButton)
// // JoystickMoved,          //!< The joystick moved along an axis (data in event.joystickMove)
// // JoystickConnected,      //!< A joystick was connected (data in event.joystickConnect)
// // JoystickDisconnected,   //!< A joystick was disconnected (data in event.joystickConnect)
// // TouchBegan,             //!< A touch event began (data in event.touch)
// // TouchMoved,             //!< A touch moved (data in event.touch)
// // TouchEnded,             //!< A touch event ended (data in event.touch)
// // SensorChanged,          //!< A sensor value changed (data in event.sensor)

// // Closed,                 //!< The window requested to be closed (no data)
// // Resized,                //!< The window was resized (data in event.size)
// // TextEntered,            //!< A character was entered (data in event.text)
// // KeyPressed,             //!< A key was pressed (data in event.key)
// // KeyReleased,            //!< A key was released (data in event.key)
// // MouseWheelScrolled,     //!< The mouse wheel was scrolled (data in event.mouseWheelScroll)
// // MouseButtonPressed,     //!< A mouse button was pressed (data in event.mouseButton)
// // MouseButtonReleased,    //!< A mouse button was released (data in event.mouseButton)
// // MouseMoved,             //!< The mouse cursor moved (data in event.mouseMove)

// // #[derive(Clone, Copy, PartialEq, Debug)]
// // pub enum Event {
// //     None,
// //     ClosedEvent,
// //     ResizedEvent { width: i32, height: i32 },
// //     KeyPressedEvent { keycode: usize, mods: usize },
// //     MouseMovedEvent { x: u16, y: u16 },
// //     TextEnteredEvent { character: char },
// //     KeyReleasedEvent { keycode: usize },
// //     MouseWheelScrolledEvent { x: u16, y: u16, delta: f64 },
// //     MouseButtonPressedEvent { button: usize, x: u16, y: u16 },
// //     KeyPressedRepeatingEvent { keycode: usize, mods: usize },
// //     MouseWheelHScrolledEvent { x: u16, y: u16, delta: f64 },
// //     MouseButtonReleasedEvent { button: usize, x: u16, y: u16 },
// // }

// // pub trait EventSubscriber {
// //     fn on_event(&self, event: Event);
// //     fn get_id(&self) -> usize;
// // }

// // pub struct EventPublisher<'a> {
// //     event: Event,
// //     event_subscribers: Vec<&'a dyn EventSubscriber>,
// // }

// // impl<'a> EventPublisher<'a> {
// //     pub fn new(event: Event) -> Self {
// //         Self {
// //             event,
// //             event_subscribers: Vec::new(),
// //         }
// //     }

// //     pub fn set_event(&mut self, e: Event) {
// //         self.event = e;
// //     }

// //     pub fn event(&self) -> &Event {
// //         &self.event
// //     }

// //     pub fn event_subscribers(&self) -> &Vec<&'a dyn EventSubscriber> {
// //         &self.event_subscribers
// //     }

// //     pub fn attach(&mut self, event_subscriber: &'a dyn EventSubscriber) -> &mut Self {
// //         self.event_subscribers.push(event_subscriber);
// //         self
// //     }
// //     pub fn detach(&mut self, event_subscriber: &'a dyn EventSubscriber) -> &mut Self {
// //         self.event_subscribers
// //             .retain(|&x| x.get_id() != event_subscriber.get_id());
// //         self
// //     }

// //     pub fn notify_subscribers(&self) {
// //         self.event_subscribers.iter().for_each(|&e| {
// //             e.on_event(self.event);
// //         });
// //     }
// // }

// pub struct Input{

// }
// pub struct Event {

// }
