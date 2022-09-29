use gio_event::event::{Event, EventPublisher, EventSubscriber};

pub struct KeyboardKey<'a> {
    pub pressed: EventPublisher<'a>,
    pub released: EventPublisher<'a>,
}

impl<'a> KeyboardKey<'a> {
    pub fn new() -> Self {
        Self {
            pressed: EventPublisher::new(Event::None),
            released: EventPublisher::new(Event::None),
        }
    }
}

type MouseButton<'a> = KeyboardKey<'a>;

pub struct Mouse<'a> {
    pub buttons: MouseButton<'a>,
    pub wheel: EventPublisher<'a>,
    pub movement: EventPublisher<'a>,
}

impl<'a> Mouse<'a> {
    pub fn new() -> Self {
        Self {
            buttons: MouseButton::new(),
            wheel: EventPublisher::new(Event::None),
            movement: EventPublisher::new(Event::None),
        }
    }
}

pub struct Input<'a> {
    pub keyboard: KeyboardKey<'a>,
    pub mouse: Mouse<'a>,
}

impl<'a> Input<'a> {
    pub fn new() -> Self {
        Self {
            keyboard: KeyboardKey::new(),
            mouse: Mouse::new(),
        }
    }

    pub fn bind_key_pressed_event(
        &mut self,
        subscriber: &'a (dyn EventSubscriber + Sync),
    ) -> &mut Self {
        self.keyboard.pressed.attach(subscriber);
        self
    }

    pub fn bind_key_released_event(
        &mut self,
        subscriber: &'a (dyn EventSubscriber + Sync),
    ) -> &mut Self {
        self.keyboard.released.attach(subscriber);
        self
    }

    pub fn bind_mouse_button_pressed_event(
        &mut self,
        subscriber: &'a (dyn EventSubscriber + Sync),
    ) -> &mut Self {
        self.mouse.buttons.pressed.attach(subscriber);
        self
    }

    pub fn bind_mouse_button_released_event(
        &mut self,
        subscriber: &'a (dyn EventSubscriber + Sync),
    ) -> &mut Self {
        self.mouse.buttons.released.attach(subscriber);
        self
    }

    pub fn bind_mouse_scroll_event(
        &mut self,
        subscriber: &'a (dyn EventSubscriber + Sync),
    ) -> &mut Self {
        self.mouse.wheel.attach(subscriber);
        self
    }

    pub fn bind_mouse_movement_event(
        &mut self,
        subscriber: &'a (dyn EventSubscriber + Sync),
    ) -> &mut Self {
        self.mouse.movement.attach(subscriber);
        self
    }
}
