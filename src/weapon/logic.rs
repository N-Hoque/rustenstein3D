use rsfml::{
    graphics::{RectangleShape, RenderTarget, RenderWindow, Shape, Transformable},
    system::Vector2f,
    window::{mouse::Button, Key},
};

use crate::{
    animation::{Animation, PlayMode, PlayState},
    event_handler::EventHandler,
    texture_loader::TextureLoader,
};

use super::Weapon;

impl<'s> Weapon<'s> {
    pub fn new(window_size: &Vector2f, texture_loader: &'s TextureLoader) -> Weapon<'s> {
        Weapon {
            texture_loader,
            weapons: Weapon::initialize_weapons(window_size),
            animations: Weapon::initialize_animation(),
            shadows: Weapon::initialize_shadows(window_size),
            shadows_id: vec![18, 25, 32, 39],
            current_weapon: 0,
            mouse_fire: false,
        }
    }

    fn initialize_weapons(window_size: &Vector2f) -> RectangleShape<'s> {
        let mut tmp_weapon = RectangleShape::with_size(Vector2f { x: 400., y: 400. });
        tmp_weapon.set_position(Vector2f::new(
            window_size.x / 2. - 200.,
            window_size.y - 400. - 81.,
        ));
        tmp_weapon
    }

    fn initialize_shadows(window_size: &Vector2f) -> RectangleShape<'s> {
        let mut tmp_shadow = RectangleShape::with_size(Vector2f { x: 99., y: 48. });
        tmp_shadow.set_position(Vector2f::new(window_size.x - 115., window_size.y - 66.));
        tmp_shadow
    }

    fn initialize_animation() -> Vec<Animation> {
        vec![
            Animation::new(
                vec![12, 13, 14, 15, 16, 17],
                PlayState::Stop,
                PlayMode::Once,
                0.07,
                3,
            ),
            Animation::new(
                vec![19, 20, 21, 22, 23, 24],
                PlayState::Stop,
                PlayMode::Once,
                0.07,
                3,
            ),
            Animation::new(
                vec![26, 27, 28, 29, 30, 31],
                PlayState::Stop,
                PlayMode::Once,
                0.07,
                3,
            ),
            Animation::new(
                vec![33, 34, 35, 36, 37, 38],
                PlayState::Stop,
                PlayMode::Once,
                0.07,
                3,
            ),
        ]
    }

    pub fn update(&mut self, event_handler: &EventHandler) {
        if event_handler.has_key_pressed_event(Key::NUM1).is_some() {
            self.current_weapon = 0
        };
        if event_handler.has_key_pressed_event(Key::NUM2).is_some() {
            self.current_weapon = 1
        };
        if event_handler.has_key_pressed_event(Key::NUM3).is_some() {
            self.current_weapon = 2
        };
        if event_handler.has_key_pressed_event(Key::NUM4).is_some() {
            self.current_weapon = 3
        };

        let animation = self
            .animations
            .get_mut(self.current_weapon as usize)
            .unwrap_or_else(|| {
                panic!(
                    "Getting animation for weapon at index: {}",
                    self.current_weapon
                )
            });

        if !self.mouse_fire {
            if event_handler
                .has_mouse_button_pressed_event(Button::LEFT)
                .is_some()
            {
                animation.set_state(PlayState::Play);
                self.mouse_fire = true;
            }
        } else if event_handler
            .has_mouse_button_released_event(Button::LEFT)
            .is_some()
        {
            self.mouse_fire = false;
        } else {
            animation.set_state(PlayState::Play);
        }

        if event_handler.is_key_pressed(Key::E) {
            animation.set_state(PlayState::Play);
        }
        animation.update();
    }

    pub fn draw(&mut self, render_window: &mut RenderWindow) {
        let current_weapon = self.current_weapon as usize;
        self.weapons.set_texture(
            self.texture_loader
                .get_texture(self.animations[current_weapon].get_current_texture_id()),
            false,
        );
        self.shadows.set_texture(
            self.texture_loader
                .get_texture(self.shadows_id[current_weapon]),
            false,
        );
        render_window.draw(&self.weapons);
        render_window.draw(&self.shadows);
    }
}
