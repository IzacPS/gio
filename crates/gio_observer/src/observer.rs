pub trait Observer {
    type Item;
    fn update(&self, event: Self::Item);
}
