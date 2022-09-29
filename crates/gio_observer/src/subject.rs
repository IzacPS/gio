use crate::observer::Observer;

pub trait Subject<'a, T> {
    fn attach(&mut self, observer: &'a dyn Observer<Item = T>);
    fn detach(&mut self, observer: &'a dyn Observer<Item = T>);
    fn notify_observers(&self);
}
