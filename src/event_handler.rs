use rsfml::{
    graphics::RenderWindow,
    window::{Event, Key, mouse::Button as MouseButton},
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
        self.events.iter().any(|ev| *ev == Event::Closed)
    }

    pub fn has_gained_focus_event(&self) -> bool {
        self.events.iter().any(|ev| *ev == Event::GainedFocus)
    }

    pub fn has_lost_focus_event(&self) -> bool {
        self.events.iter().any(|ev| *ev == Event::LostFocus)
    }

    pub fn has_text_entered(&self) -> Option<char> {
        self.events.iter().find_map(|ev| match *ev {
            Event::TextEntered { unicode } => Some(unicode),
            _ => None
        })
    }

    pub fn has_key_pressed_event(&self, key: Key) -> Option<(Key, bool, bool, bool, bool)> {
        self.events.iter().find_map(|ev| match *ev {
            Event::KeyPressed {
                code,
                alt,
                ctrl,
                shift,
                system, } if code == key => Some((code, alt, ctrl, shift, system)),
            _ => None
        })
    }

    pub fn has_key_released_event(&self, key: Key) -> Option<(Key, bool, bool, bool, bool)> {
        self.events.iter().find_map(|ev| match *ev {
            Event::KeyReleased {
                code,
                alt,
                ctrl,
                shift,
                system, } if code == key => Some((code, alt, ctrl, shift, system)),
            _ => None
        })
    }

    pub fn has_mouse_wheel_moved_event(&self) -> Option<(i32, i32, i32)> {
        self.events.iter().find_map(|ev| match *ev {
            Event::MouseWheelScrolled {
                wheel: _,
                delta,
                x,
                y, } => Some((delta as i32, x, y)),
            _ => None
        })
    }

    pub fn has_mouse_button_pressed_event(
        &self,
        mouse_button: MouseButton,
    ) -> Option<(MouseButton, i32, i32)> {
        self.events.iter().find_map(|ev| match *ev {
            Event::MouseButtonPressed { button, x, y } if mouse_button == button => Some((button, x, y)),
            _ => None
        })
    }

    pub fn has_mouse_button_released_event(
        &self,
        mouse_button: MouseButton,
    ) -> Option<(MouseButton, i32, i32)> {
        self.events.iter().find_map(|ev| match *ev {
            Event::MouseButtonReleased { button, x, y } if mouse_button == button => Some((button, x, y)),
            _ => None
        })
    }

    pub fn has_mouse_moved_event(&self) -> Option<(i32, i32)> {
        self.events.iter().find_map(|ev| match *ev {
            Event::MouseMoved { x, y } => Some((x, y)),
            _ => None
        })
    }

    pub fn has_mouse_entered_event(&self) -> bool {
        self.events.iter().any(|ev| *ev == Event::MouseEntered)
    }

    pub fn has_mouse_left_event(&self) -> bool {
        self.events.iter().any(|ev| *ev == Event::MouseLeft)
    }

    // pub fn get_mouse_position(&self) -> Vector2i {
    //     self.render_window.get_mouse_position()
    // }

    pub fn get_events(&self) -> Vec<Event> {
        self.events.clone()
    }

    pub fn update_events(&mut self, render_window: &mut RenderWindow) -> () {
        self.events.clear();
        while let Some(ev) = render_window.poll_event() {
            self.events.push(ev)
        }
    }
}

// TODO IMPLEMENT FUNCTION FOR JOYSTICK HANDLE
// JoystickButtonPressed { joystickid : int, button : int },
// JoystickButtonReleased { joystickid : int, button : int },
// JoystickMoved { joystickid : uint, axis : Axis, position : float },
// JoystickConnected { joystickid : uint },
// JoystickDisconnected { joystickid : uint },
