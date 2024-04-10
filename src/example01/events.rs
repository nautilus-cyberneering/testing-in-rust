use std::error::Error;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Event {
    Connect,
    Announce,
    Scrape,
}

pub trait EventSender {
    /// # Errors
    ///
    /// Will return an error if the event can't be sent.
    fn send_event(&self, event: Event) -> Result<(), Box<dyn Error>>;
}

#[derive(Clone)]
pub struct TrackerEventSender {}

impl EventSender for TrackerEventSender {
    fn send_event(&self, event: Event) -> Result<(), Box<dyn Error>> {
        println!("Event::{event:?} sent.");
        Ok(())
    }
}
