use std::rc::Rc;

use crate::example01::tracker::Tracker;

/// Controller for the UDP connect request
///
/// # Panics
///
/// Will panic if the tracker can't handle the connect request.
pub fn handle_connect(tracker: &Rc<Tracker>) {
    tracker
        .connect()
        .expect("Tracker should handle the connect request");
}
