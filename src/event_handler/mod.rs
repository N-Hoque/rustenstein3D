pub mod logic;

use rsfml::window::Event;

#[derive(Default)]
pub struct EventHandler {
    pub events: Vec<Event>,
}
