pub mod logic;

use rsfml::{
    graphics::{RectangleShape, VertexArray},
    system::{Clock, Vector2f},
};

use crate::{animation::Animation, texture_loader::TextureLoader};

#[allow(clippy::upper_case_acronyms)]
pub struct HUD<'s> {
    window_size: Vector2f,
    background: RectangleShape<'s>,
    hud_vertex_array: VertexArray,
    face: RectangleShape<'s>,
    face_animation: Animation,
    texture_loader: &'s TextureLoader,
    face_clock: Clock,
}
