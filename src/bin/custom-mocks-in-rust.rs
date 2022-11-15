use std::{sync::Arc, rc::Rc};

use testing_in_rust::example01::{
    events::TrackerEventSender, handlers::handle_connect, tracker::Tracker,
};

fn main() {
    let event_sender = Rc::new(TrackerEventSender {});
    let tracker = Arc::new(Tracker::new(event_sender));

    handle_connect(tracker)
}
