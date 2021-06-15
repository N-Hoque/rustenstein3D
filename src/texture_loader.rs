//! Module for loading textures

use rsfml::{graphics::Texture, system::SfBox};

pub struct TextureLoader {
    textures: Vec<SfBox<Texture>>,
}

impl TextureLoader {
    pub fn new() -> TextureLoader {
        TextureLoader {
            textures: Vec::new(),
        }
    }

    pub fn load_texture(&mut self, texture_path: &str) -> bool {
        let texture = Texture::from_file(texture_path);
        let successfully_loaded = texture.is_some();

        if let Some(tex) = texture {
            self.textures.push(tex);
        }

        successfully_loaded
    }

    pub fn get_texture(&self, index: i32) -> &Texture {
        &self.textures[index as usize]
    }
}
