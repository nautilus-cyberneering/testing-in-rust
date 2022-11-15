use std::error::Error;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Event {
    Connect,
    Announce,
    Scrape,
}

pub trait EventSender {
    fn send_event(&self, event: Event) -> Result<(), Box<dyn Error>>;
}

#[derive(Clone)]
pub struct TrackerEventSender {}

impl EventSender for TrackerEventSender {
    fn send_event(&self, event: Event) -> Result<(), Box<dyn Error>> {
        println!("Event::{:?} sent.", event);
        Ok(())
    }
}
