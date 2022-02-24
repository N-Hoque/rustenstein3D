pub mod logic;

use rsfml::{graphics::Texture, SfBox};

#[derive(Default)]
pub struct TextureLoader {
    textures: Vec<SfBox<Texture>>,
}
