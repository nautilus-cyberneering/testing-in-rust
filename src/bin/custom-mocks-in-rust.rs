//! cargo run --bin custom-mocks-in-rust

use std::{rc::Rc, sync::Arc};

use testing_in_rust::example01::{
    events::TrackerEventSender, handlers::handle_connect, tracker::Tracker,
};

fn main() {
    let event_sender = Rc::new(TrackerEventSender {});
    let tracker = Arc::new(Tracker::new(event_sender));

    handle_connect(tracker)
}
