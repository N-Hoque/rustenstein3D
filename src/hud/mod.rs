pub mod logic;

use rsfml::{
    graphics::RectangleShape,
    system::{Clock, Vector2f},
};

use crate::{animation::Animation, texture_loader::TextureLoader};

#[allow(clippy::upper_case_acronyms)]
pub struct HUD<'s> {
    window_size: Vector2f,
    background: RectangleShape<'s>,
    face: RectangleShape<'s>,
    face_animation: Animation,
    texture_loader: &'s TextureLoader,
    face_clock: Clock,
}
