pub struct Listener<Type: Default + Copy> {
    data: Option<Type>,
}

impl<Type: Default + Copy> Listener<Type> {
    pub fn new() -> Self {
        Self { data: None }
    }
    pub fn update(&mut self, data: Type) {
        self.data = Some(data);
    }
    pub fn data(&mut self) -> Option<Type> {
        self.data
    }
}
