// use gio_event::event::{Event, EventPublisher, EventSubscriber};

// pub struct KeyboardKey {
//     pub pressed: EventPublisher<'static>,
//     pub released: EventPublisher<'static>,
// }

// impl KeyboardKey {
//     pub fn new() -> Self {
//         Self {
//             pressed: EventPublisher::new(Event::None),
//             released: EventPublisher::new(Event::None),
//         }
//     }
// }

// type MouseButton = KeyboardKey;

// pub struct Mouse {
//     pub buttons: MouseButton,
//     pub wheel: EventPublisher<'static>,
//     pub movement: EventPublisher<'static>,
// }

// impl Mouse {
//     pub fn new() -> Self {
//         Self {
//             buttons: MouseButton::new(),
//             wheel: EventPublisher::new(Event::None),
//             movement: EventPublisher::new(Event::None),
//         }
//     }
// }

// pub struct Input {
//     pub keyboard: KeyboardKey,
//     pub mouse: Mouse,
// }

// impl Input {
//     pub fn new() -> Self {
//         Self {
//             keyboard: KeyboardKey::new(),
//             mouse: Mouse::new(),
//         }
//     }

//     pub fn bind_key_pressed_event(
//         &mut self,
//         subscriber: &'static dyn EventSubscriber,
//     ) -> &mut Self {
//         self.keyboard.pressed.attach(subscriber);
//         self
//     }

//     pub fn bind_key_released_event(
//         &mut self,
//         subscriber: &'static dyn EventSubscriber,
//     ) -> &mut Self {
//         self.keyboard.released.attach(subscriber);
//         self
//     }

//     pub fn bind_mouse_button_pressed_event(
//         &mut self,
//         subscriber: &'static dyn EventSubscriber,
//     ) -> &mut Self {
//         self.mouse.buttons.pressed.attach(subscriber);
//         self
//     }

//     pub fn bind_mouse_button_released_event(
//         &mut self,
//         subscriber: &'static dyn EventSubscriber,
//     ) -> &mut Self {
//         self.mouse.buttons.released.attach(subscriber);
//         self
//     }

//     pub fn bind_mouse_scroll_event(
//         &mut self,
//         subscriber: &'static dyn EventSubscriber,
//     ) -> &mut Self {
//         self.mouse.wheel.attach(subscriber);
//         self
//     }

//     pub fn bind_mouse_movement_event(
//         &mut self,
//         subscriber: &'static dyn EventSubscriber,
//     ) -> &mut Self {
//         self.mouse.movement.attach(subscriber);
//         self
//     }
// }
