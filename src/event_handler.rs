use rsfml::{
    graphics::RenderWindow,
    window::{joystick::Axis, mouse::Button, Event, Key},
};

pub struct EventHandler {
    pub events: Vec<Event>,
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl EventHandler {
    pub fn new() -> EventHandler {
        EventHandler { events: Vec::new() }
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        Key::is_pressed(key)
    }

    pub fn has_closed_event(&self) -> bool {
        self.events.iter().any(|e| e == &Event::Closed)
    }

    pub fn has_gained_focus_event(&self) -> bool {
        self.events.iter().any(|e| e == &Event::GainedFocus)
    }

    pub fn has_lost_focus_event(&self) -> bool {
        self.events.iter().any(|e| e == &Event::LostFocus)
    }

    pub fn has_text_entered(&self) -> Option<char> {
        self.events.iter().find_map(|e| {
            if let Event::TextEntered { unicode } = *e {
                Some(unicode)
            } else {
                None
            }
        })
    }

    pub fn has_key_pressed_event(&self, key: Key) -> Option<(Key, bool, bool, bool, bool)> {
        self.events.iter().find_map(|e| {
            if let Event::KeyPressed {
                code,
                alt,
                ctrl,
                shift,
                system,
            } = *e
            {
                if code == key {
                    Some((code, alt, ctrl, shift, system))
                } else {
                    None
                }
            } else {
                None
            }
        })
    }

    pub fn has_key_released_event(&self, key: Key) -> Option<(Key, bool, bool, bool, bool)> {
        self.events.iter().find_map(|e| {
            if let Event::KeyReleased {
                code,
                alt,
                ctrl,
                shift,
                system,
            } = *e
            {
                if code == key {
                    Some((code, alt, ctrl, shift, system))
                } else {
                    None
                }
            } else {
                None
            }
        })
    }

    pub fn has_mouse_wheel_moved_event(&self) -> Option<(f32, i32, i32)> {
        self.events.iter().find_map(|e| {
            if let Event::MouseWheelScrolled { delta, x, y, .. } = *e {
                Some((delta, x, y))
            } else {
                None
            }
        })
    }

    pub fn has_mouse_button_pressed_event(
        &self,
        mouse_button: Button,
    ) -> Option<(Button, i32, i32)> {
        self.events.iter().find_map(|e| {
            if let Event::MouseButtonPressed { button, x, y } = *e {
                if mouse_button == button {
                    Some((button, x, y))
                } else {
                    None
                }
            } else {
                None
            }
        })
    }

    pub fn has_mouse_button_released_event(
        &self,
        mouse_button: Button,
    ) -> Option<(Button, i32, i32)> {
        self.events.iter().find_map(|e| {
            if let Event::MouseButtonReleased { button, x, y } = *e {
                if mouse_button == button {
                    Some((button, x, y))
                } else {
                    None
                }
            } else {
                None
            }
        })
    }

    pub fn has_mouse_moved_event(&self) -> Option<(i32, i32)> {
        self.events.iter().find_map(|e| {
            if let Event::MouseMoved { x, y } = *e {
                Some((x, y))
            } else {
                None
            }
        })
    }

    pub fn has_mouse_entered_event(&self) -> bool {
        self.events.iter().any(|e| e == &Event::MouseEntered)
    }

    pub fn has_mouse_left_event(&self) -> bool {
        self.events.iter().any(|e| e == &Event::MouseLeft)
    }

    pub fn get_events(&self) -> Vec<Event> {
        self.events.to_vec()
    }

    pub fn update_events(&mut self, render_window: &mut RenderWindow) {
        self.events.clear();
        while let Some(ev) = render_window.poll_event() {
            self.events.push(ev);
        }
    }

    pub fn has_joystick_button_pressed(&self, joystick_button: u32) -> Option<(u32, u32)> {
        self.events.iter().find_map(|e| {
            if let Event::JoystickButtonPressed { button, joystickid } = *e {
                if joystick_button == button {
                    Some((button, joystickid))
                } else {
                    None
                }
            } else {
                None
            }
        })
    }

    pub fn has_joystick_button_released(&self, joystick_button: u32) -> Option<(u32, u32)> {
        self.events.iter().find_map(|e| {
            if let Event::JoystickButtonReleased { button, joystickid } = *e {
                if joystick_button == button {
                    Some((button, joystickid))
                } else {
                    None
                }
            } else {
                None
            }
        })
    }

    pub fn has_joystick_moved(&self) -> Option<(Axis, f32, u32)> {
        self.events.iter().find_map(|e| {
            if let Event::JoystickMoved {
                axis,
                position,
                joystickid,
            } = *e
            {
                Some((axis, position, joystickid))
            } else {
                None
            }
        })
    }

    pub fn has_joystick_connected(&self) -> Option<u32> {
        self.events.iter().find_map(|e| {
            if let Event::JoystickConnected { joystickid } = *e {
                Some(joystickid)
            } else {
                None
            }
        })
    }

    pub fn has_joystick_disconnected(&self) -> Option<u32> {
        self.events.iter().find_map(|e| {
            if let Event::JoystickDisconnected { joystickid } = *e {
                Some(joystickid)
            } else {
                None
            }
        })
    }
}
