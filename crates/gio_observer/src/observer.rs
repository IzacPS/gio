pub trait Observer<T> {
    fn update(&self, message: T);
}
