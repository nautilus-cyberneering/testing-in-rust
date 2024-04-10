use std::{error::Error, rc::Rc};

use crate::example01::events::{Event, EventSender};

/// `BitTorrent` tracker
pub struct Tracker {
    event_sender: Rc<dyn EventSender>,
}

impl Tracker {
    pub fn new(event_sender: Rc<dyn EventSender>) -> Self {
        Self { event_sender }
    }

    /// # Errors
    ///
    /// Will return an error if `Connect` event cant' be sent.
    pub fn connect(&self) -> Result<(), Box<dyn Error>> {
        println!("Tracker::connect.");

        // After connecting the tracker sends an event
        self.event_sender.send_event(Event::Connect)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, error::Error, rc::Rc};

    use crate::example01::{
        events::{Event, EventSender, TrackerEventSender},
        tracker::Tracker,
    };

    #[test]
    fn the_tracker_should_allow_connections() {
        // This is just a dummy test to show how we use the real struct instead of the mock
        let event_sender = Rc::new(TrackerEventSender {});
        let tracker = Rc::new(Tracker::new(event_sender));

        assert!(tracker.connect().is_ok());
    }

    #[derive(Clone)]
    struct TrackerEventSenderMock {
        pub sent_event: RefCell<Option<Event>>,
    }

    impl TrackerEventSenderMock {
        pub fn new() -> Self {
            Self {
                sent_event: RefCell::new(None),
            }
        }
    }

    impl EventSender for TrackerEventSenderMock {
        fn send_event(&self, event: Event) -> Result<(), Box<dyn Error>> {
            *self.sent_event.borrow_mut() = Some(event);

            // We return the expected value
            Ok(())
        }
    }

    #[test]
    fn the_tracker_should_send_a_connect_event_after_connecting() {
        // Test using a custom mock for the TrackerEventSender

        let event_sender = Rc::new(TrackerEventSenderMock::new());
        let tracker = Rc::new(Tracker::new(event_sender.clone()));

        tracker.connect().unwrap();

        assert_eq!(event_sender.sent_event.borrow().unwrap(), Event::Connect);
    }
}
