pub mod logic;

use rsfml::{graphics::Texture, SfBox};

pub struct TextureLoader {
    textures: Vec<SfBox<Texture>>,
}

impl Default for TextureLoader {
    fn default() -> Self {
        Self {
            textures: Vec::new(),
        }
    }
}
