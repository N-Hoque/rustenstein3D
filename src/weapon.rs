use rsfml::{
    graphics::{RectangleShape, RenderTarget, RenderWindow, Shape, Transformable},
    system::Vector2f,
    window::{mouse, Key},
};

use crate::animation::*;
use crate::event_handler::EventHandler;
use crate::texture_loader::TextureLoader;

pub struct Weapon<'s> {
    weapons: RectangleShape<'s>,
    animations: Vec<Animation>,
    texture_loader: &'s TextureLoader,
    shadows: RectangleShape<'s>,
    shadows_id: Vec<i32>,
    current_weapon: i32,
    mouse_fire: bool,
}

impl<'s> Weapon<'s> {
    pub fn new(window_size: &Vector2f, texture_loader: &'s TextureLoader) -> Weapon<'s> {
        Weapon {
            weapons: Weapon::initialize_weapons(window_size),
            animations: Weapon::initialize_animation(),
            texture_loader,
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
        let mut animations = Vec::new();
        animations.push(Animation::new(
            vec![12, 13, 14, 15, 16, 17],
            AnimationState::Stop,
            AnimationPlayMode::Once,
            0.07,
            3,
        ));
        animations.push(Animation::new(
            vec![19, 20, 21, 22, 23, 24],
            AnimationState::Stop,
            AnimationPlayMode::Once,
            0.07,
            3,
        ));
        animations.push(Animation::new(
            vec![26, 27, 28, 29, 30, 31],
            AnimationState::Stop,
            AnimationPlayMode::Once,
            0.07,
            3,
        ));
        animations.push(Animation::new(
            vec![33, 34, 35, 36, 37, 38],
            AnimationState::Stop,
            AnimationPlayMode::Once,
            0.07,
            3,
        ));

        animations
    }

    pub fn update<'r>(&'r mut self, event_handler: &'r EventHandler) -> () {
        if let Some(_) = event_handler.has_key_pressed_event(Key::NUM1) {
            self.current_weapon = 0
        };
        if let Some(_) = event_handler.has_key_pressed_event(Key::NUM2) {
            self.current_weapon = 1
        };
        if let Some(_) = event_handler.has_key_pressed_event(Key::NUM3) {
            self.current_weapon = 2
        };
        if let Some(_) = event_handler.has_key_pressed_event(Key::NUM4) {
            self.current_weapon = 3
        };

        if !self.mouse_fire {
            if let Some(_) = event_handler.has_mouse_button_pressed_event(mouse::Button::LEFT) {
                self.animations
                    .get_mut(self.current_weapon as usize)
                    .unwrap()
                    .set_state(AnimationState::Play);
                self.mouse_fire = true
            };
        } else if let Some(_) = event_handler.has_mouse_button_released_event(mouse::Button::LEFT) {
            self.mouse_fire = false
        } else {
            self.animations
                .get_mut(self.current_weapon as usize)
                .unwrap()
                .set_state(AnimationState::Play)
        };

        if event_handler.is_key_pressed(Key::E) {
            self.animations
                .get_mut(self.current_weapon as usize)
                .unwrap()
                .set_state(AnimationState::Play);
        }
        self.animations
            .get_mut(self.current_weapon as usize)
            .unwrap()
            .update();
    }

    pub fn draw<'r>(&'r mut self, render_window: &'r mut RenderWindow) -> () {
        self.weapons.set_texture(
            self.texture_loader.get_texture(
                self.animations[self.current_weapon as usize].get_current_texture_id(),
            ),
            false,
        );
        self.shadows.set_texture(
            self.texture_loader
                .get_texture(self.shadows_id[self.current_weapon as usize]),
            false,
        );
        render_window.draw(&self.weapons);
        render_window.draw(&self.shadows);
    }
}
