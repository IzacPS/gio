pub mod dispatcher;
pub mod event;

use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

use crate::dispatcher::EventDispatcher;

lazy_static! {
    pub static ref DISPATCHER: Arc<Mutex<EventDispatcher<'static>>> =
        Arc::new(Mutex::new(EventDispatcher::new()));
}

#[cfg(test)]
mod tests {
    use crate::{
        dispatcher::EventDispatcher,
        event::{Event, EventPublisher, EventSubscriber},
    };

    struct App {}

    impl EventSubscriber for App {
        fn on_event(&self, event: crate::event::Event) {
            match event {
                Event::ClosedEvent => {
                    println!("Close Event!");
                }
                _ => {
                    println!("None");
                }
            }
        }

        fn get_id(&self) -> usize {
            100
        }
    }

    #[test]
    fn test_new_publisher() {
        let publisher = EventPublisher::new(Event::ClosedEvent);
        assert_eq!(*publisher.event(), Event::ClosedEvent);
    }
    #[test]
    fn test_new_subscriber() {
        let app = App {};
        let mut publisher = EventPublisher::new(Event::ClosedEvent);
        assert_eq!(*publisher.event(), Event::ClosedEvent);

        publisher.attach(&app).attach(&app);
        assert_eq!(publisher.event_subscribers().last().unwrap().get_id(), 100);
        assert_eq!(publisher.event_subscribers().len(), 2);
    }

    #[test]
    fn test_new_dispatcher() {
        let app = App {};
        let mut publisher = EventPublisher::new(Event::ClosedEvent);
        assert_eq!(*publisher.event(), Event::ClosedEvent);

        publisher.attach(&app);
        assert_eq!(publisher.event_subscribers().last().unwrap().get_id(), 100);

        let mut dispatcher = EventDispatcher::new();
        dispatcher.push(&publisher).push(&publisher);
        assert_eq!(
            *dispatcher.queue().back().unwrap().event(),
            Event::ClosedEvent
        );
        assert_eq!(dispatcher.queue().len(), 2);
    }
}
