//! ```text
//! cargo run --bin custom-mocks-in-rust
//! ```
use std::rc::Rc;

use testing_in_rust::example01::{
    events::TrackerEventSender, handlers::handle_connect, tracker::Tracker,
};

fn main() {
    let event_sender = Rc::new(TrackerEventSender {});
    let tracker = Rc::new(Tracker::new(event_sender));

    handle_connect(&tracker);
}
