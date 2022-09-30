use crate::listener::Listener;

pub struct Publisher<Type: Default + Copy> {
    listeners: Vec<Listener<Type>>,
}

impl<Type: Default + Copy> Publisher<Type> {
    pub fn new() -> Self {
        Self {
            listeners: Vec::new(),
        }
    }

    pub fn publish(&mut self, data: Type) {
        for listener in &mut self.listeners {
            listener.update(data);
        }
    }
}
