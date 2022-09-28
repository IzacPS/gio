pub trait Subject<'a, T: crate::observer::Observer<T>> {
    fn attach(&mut self, observer: &'a T);
    fn detach(&mut self, observer: &'a T);
    fn notify_observers(&self);
}
