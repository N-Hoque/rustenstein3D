use rsfml::{
    graphics::RenderWindow,
    window::{mouse::Button, Event, Key},
};

use crate::RenderUpdate;
#[derive(Default)]
pub struct EventHandler {
    pub events: Vec<Event>,
}

impl EventHandler {
    pub fn is_key_pressed(&self, key: Key) -> bool {
        Key::is_pressed(key)
    }

    pub fn has_closed_event(&self) -> bool {
        self.events.iter().any(|e| e == &Event::Closed)
    }

    pub fn has_key_pressed_event(&self, key: Key) -> Option<(Key, bool, bool, bool, bool)> {
        self.events.iter().find_map(|e| match *e {
            Event::KeyPressed {
                code,
                alt,
                ctrl,
                shift,
                system,
            } if code == key => Some((code, alt, ctrl, shift, system)),
            _ => None,
        })
    }

    pub fn has_mouse_button_pressed_event(
        &self,
        mouse_button: Button,
    ) -> Option<(Button, i32, i32)> {
        self.events.iter().find_map(|e| match *e {
            Event::MouseButtonPressed { button, x, y } if mouse_button == button => {
                Some((button, x, y))
            }
            _ => None,
        })
    }

    pub fn has_mouse_button_released_event(
        &self,
        mouse_button: Button,
    ) -> Option<(Button, i32, i32)> {
        self.events.iter().find_map(|e| match *e {
            Event::MouseButtonReleased { button, x, y } if mouse_button == button => {
                Some((button, x, y))
            }
            _ => None,
        })
    }

    pub fn has_mouse_moved_event(&self) -> Option<(i32, i32)> {
        self.events.iter().find_map(|e| match *e {
            Event::MouseMoved { x, y } => Some((x, y)),
            _ => None,
        })
    }
}

impl RenderUpdate for EventHandler {
    fn update(&mut self, render_window: &mut RenderWindow) {
        self.events.clear();
        while let Some(ev) = render_window.poll_event() {
            self.events.push(ev);
        }
    }
}
