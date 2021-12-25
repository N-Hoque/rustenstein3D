pub mod logic;

use rsfml::window::Event;

pub struct EventHandler {
    pub events: Vec<Event>,
}

impl Default for EventHandler {
    fn default() -> Self {
        Self { events: Vec::new() }
    }
}
