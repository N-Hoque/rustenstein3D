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
        match texture {
            Some(tex) => {
                self.textures.push(tex);
                true
            }
            None => false,
        }
    }

    pub fn get_texture(&self, index: i32) -> &Texture {
        &self.textures[index as usize]
    }
}
