use std::sync::Arc;

use crate::example01::tracker::Tracker;

/// Controller for the UDP connect request
pub fn handle_connect(tracker: Arc<Tracker>) {
    tracker.connect().unwrap();
}
