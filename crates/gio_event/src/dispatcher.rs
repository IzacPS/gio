// use std::collections::VecDeque;

// use crate::event::EventPublisher;

// pub struct EventDispatcher<'a> {
//     queue: VecDeque<&'a EventPublisher<'a>>,
// }

// impl<'a> EventDispatcher<'a> {
//     pub fn new() -> Self {
//         Self {
//             queue: VecDeque::new(),
//         }
//     }

//     pub fn queue(&self) -> &VecDeque<&'a EventPublisher<'a>> {
//         &self.queue
//     }

//     pub fn push(&mut self, publisher: &'a EventPublisher<'a>) -> &mut Self {
//         self.queue.push_back(publisher);
//         self
//     }

//     pub fn dispatch(&mut self) {
//         while let Some(publisher) = self.queue.pop_front() {
//             publisher.notify_subscribers();
//         }
//     }
// }
