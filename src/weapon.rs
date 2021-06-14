//! Module for drawing weapons

use std::ops::Range;

use rsfml::{
    graphics::{RectangleShape, RenderTarget, RenderWindow, Shape, Transformable},
    system::Vector2f,
    window::{mouse::Button as MouseButton, Key},
};

use crate::{animation::*, event_handler::EventHandler, texture_loader::TextureLoader};

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
    /// Instantiates a new Weapon.
    ///
    /// # Arguments
    /// `window_size` - The size of the window to draw the weapons onto
    ///
    /// `texture_loader` - A [TextureLoader] to obtain weapon textures from
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

    fn create_animation_by_range(texture_id_range: Range<i32>) -> Animation {
        Animation::new(
            texture_id_range.collect(),
            AnimationState::Stop,
            AnimationPlayMode::Once,
            0.07,
            3,
        )
    }

    fn initialize_animation() -> Vec<Animation> {
        let mut animations = Vec::new();
        animations.push(Weapon::create_animation_by_range(12..18));
        animations.push(Weapon::create_animation_by_range(19..25));
        animations.push(Weapon::create_animation_by_range(26..32));
        animations.push(Weapon::create_animation_by_range(33..39));
        animations
    }

    fn update_animations<'r>(&'r mut self) {
        self.animations
            .get_mut(self.current_weapon as usize)
            .unwrap()
            .update();
    }

    fn update_reload<'r>(&'r mut self, event_handler: &EventHandler) {
        if event_handler.is_key_pressed(Key::E) {
            self.animations
                .get_mut(self.current_weapon as usize)
                .unwrap()
                .set_state(AnimationState::Play);
        }
    }

    fn update_action<'r>(&'r mut self, event_handler: &EventHandler) {
        if !self.mouse_fire {
            if let Some(_) = event_handler.get_mouse_button_pressed_event(MouseButton::Left) {
                self.animations
                    .get_mut(self.current_weapon as usize)
                    .unwrap()
                    .set_state(AnimationState::Play);
                self.mouse_fire = true
            };
        } else if let Some(_) = event_handler.get_mouse_button_released_event(MouseButton::Left) {
            self.mouse_fire = false
        } else {
            self.animations
                .get_mut(self.current_weapon as usize)
                .unwrap()
                .set_state(AnimationState::Play)
        };
    }

    fn update_selection<'r>(&'r mut self, event_handler: &EventHandler) {
        if let Some(_) = event_handler.has_key_pressed_event(Key::Num1) {
            self.current_weapon = 0
        };
        if let Some(_) = event_handler.has_key_pressed_event(Key::Num2) {
            self.current_weapon = 1
        };
        if let Some(_) = event_handler.has_key_pressed_event(Key::Num3) {
            self.current_weapon = 2
        };
        if let Some(_) = event_handler.has_key_pressed_event(Key::Num4) {
            self.current_weapon = 3
        };
    }

    /// Updates the weapon state
    ///
    /// # Arguments
    /// `event_handler` - The event handler to read input from to update state
    pub fn update<'r>(&'r mut self, event_handler: &'r EventHandler) -> () {
        self.update_selection(event_handler);

        self.update_action(event_handler);

        self.update_reload(event_handler);

        self.update_animations();
    }

    /// Updates the weapon animation on the next frame
    ///
    /// # Arguments
    /// `render_window` - The window to draw the weapon onto
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
