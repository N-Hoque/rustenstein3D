use rsfml::{graphics::Texture, SfBox};

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
        match Texture::from_file(texture_path) {
            Some(tex) => {
                self.textures.push(tex);
                true
            }
            None => false,
        }
    }

    pub fn get_texture(&self, index: i32) -> &Texture {
        &*(self.textures[index as usize])
    }
}

impl Default for TextureLoader {
    fn default() -> Self {
        Self::new()
    }
}
