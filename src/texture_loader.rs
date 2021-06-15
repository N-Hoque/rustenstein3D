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

    pub fn load_texture(&mut self, texture_path: &str) -> Result<(), String> {
        let texture = Texture::from_file(texture_path);

        if let Some(tex) = texture {
            self.textures.push(tex);
            Ok(())
        } else {
            Err(format!("ERROR: Failed to load texture {}", texture_path))
        }
    }

    pub fn get_texture(&self, index: i32) -> &Texture {
        &self.textures[index as usize]
    }
}
