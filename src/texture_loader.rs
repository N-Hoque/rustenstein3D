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

    pub fn load_texture(&mut self, texture_path: String) -> bool {
        match Texture::from_file(texture_path.as_str()) {
            Some(tex) => {
                self.textures.push(tex);
                true
            }
            None => false,
        }
    }

    pub fn get_texture<'r>(&'r self, index: i32) -> &'r Texture {
        &*(self.textures[index as usize])
    }
}
