pub mod logic;

use rsfml::graphics::RectangleShape;

use crate::{animation::Animation, texture_loader::TextureLoader};

pub struct Weapon<'s> {
    weapons: RectangleShape<'s>,
    animations: Vec<Animation>,
    texture_loader: &'s TextureLoader,
    shadows: RectangleShape<'s>,
    shadows_id: Vec<i32>,
    current_weapon: usize,
    mouse_fire: bool,
}
