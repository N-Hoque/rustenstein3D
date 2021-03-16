use rsfml::{
    graphics::RenderWindow,
    window::{mouse::Button as MouseButton, Event, Key},
};

pub struct EventHandler {
    pub events: Vec<Event>,
}

impl EventHandler {
    pub fn new() -> EventHandler {
        EventHandler { events: Vec::new() }
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        Key::is_pressed(key)
    }

    pub fn has_closed_event(&self) -> bool {
        for ev in self.events.iter() {
            match *ev {
                Event::Closed => return true,
                _ => {}
            }
        }
        false
    }

    pub fn has_gained_focus_event(&self) -> bool {
        for ev in self.events.iter() {
            match *ev {
                Event::GainedFocus => return true,
                _ => {}
            }
        }
        false
    }

    pub fn has_lost_focus_event(&self) -> bool {
        for ev in self.events.iter() {
            match *ev {
                Event::LostFocus => return true,
                _ => {}
            }
        }
        false
    }

    pub fn has_text_entered(&self) -> Option<char> {
        for ev in self.events.iter() {
            if let Event::TextEntered { unicode: code } = *ev {
                return Some(code);
            }
        }
        None
    }

    pub fn has_key_pressed_event(&self, key: Key) -> Option<(Key, bool, bool, bool, bool)> {
        for ev in self.events.iter() {
            match *ev {
                Event::KeyPressed {
                    code,
                    alt,
                    ctrl,
                    shift,
                    system,
                } => {
                    if code == key {
                        return Some((code, alt, ctrl, shift, system));
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub fn has_key_released_event(&self, key: Key) -> Option<(Key, bool, bool, bool, bool)> {
        for ev in self.events.iter() {
            match *ev {
                Event::KeyReleased {
                    code,
                    alt,
                    ctrl,
                    shift,
                    system,
                } => {
                    if code == key {
                        return Some((code, alt, ctrl, shift, system));
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub fn has_mouse_wheel_moved_event(&self) -> Option<(i32, i32, i32)> {
        for ev in self.events.iter() {
            match *ev {
                Event::MouseWheelScrolled {
                    wheel: _,
                    delta,
                    x,
                    y,
                } => return Some((delta as i32, x, y)),
                _ => {}
            }
        }
        None
    }

    pub fn has_mouse_button_pressed_event(
        &self,
        mouse_button: MouseButton,
    ) -> Option<(MouseButton, i32, i32)> {
        for ev in self.events.iter() {
            match *ev {
                Event::MouseButtonPressed { button, x, y } => {
                    if mouse_button == button {
                        return Some((button, x, y));
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub fn has_mouse_button_released_event(
        &self,
        mouse_button: MouseButton,
    ) -> Option<(MouseButton, i32, i32)> {
        for ev in self.events.iter() {
            match *ev {
                Event::MouseButtonReleased { button, x, y } => {
                    if mouse_button == button {
                        return Some((button, x, y));
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub fn has_mouse_moved_event(&self) -> Option<(i32, i32)> {
        for ev in self.events.iter() {
            match *ev {
                Event::MouseMoved { x, y } => return Some((x, y)),
                _ => {}
            }
        }
        None
    }

    pub fn has_mouse_entered_event(&self) -> bool {
        for ev in self.events.iter() {
            match *ev {
                Event::MouseEntered => return true,
                _ => {}
            }
        }
        false
    }

    pub fn has_mouse_left_event(&self) -> bool {
        for ev in self.events.iter() {
            match *ev {
                Event::MouseLeft => return true,
                _ => {}
            }
        }
        false
    }

    // pub fn get_mouse_position(&self) -> Vector2i {
    //     self.render_window.get_mouse_position()
    // }

    pub fn get_events(&self) -> Vec<Event> {
        let mut r_events = Vec::new();
        for ev in self.events.iter() {
            r_events.push(*ev)
        }
        r_events
    }

    pub fn update_events(&mut self, render_window: &mut RenderWindow) -> () {
        self.events.clear();
        let mut ev;
        loop {
            ev = render_window.poll_event();
            if let Some(ev) = ev {
                self.events.push(ev)
            } else {
                break;
            }
        }
    }
}

// TODO IMPLEMENT FUNCTION FOR JOYSTICK HANDLE
// JoystickButtonPressed { joystickid : int, button : int },
// JoystickButtonReleased { joystickid : int, button : int },
// JoystickMoved { joystickid : uint, axis : Axis, position : float },
// JoystickConnected { joystickid : uint },
// JoystickDisconnected { joystickid : uint },
