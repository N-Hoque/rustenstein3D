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

    pub fn with_textures(texture_paths: &[&str]) -> Self {
        let textures = texture_paths
            .iter()
            .map(|t| Texture::from_file(t).unwrap_or_else(|| panic!("Loading texture from {}", t)))
            .collect();
        Self { textures }
    }

    pub fn load_textures(&mut self, texture_paths: &[&str]) {
        for texture_path in texture_paths {
            self.load_texture(texture_path);
        }
    }

    pub fn load_texture(&mut self, texture_path: &str) {
        if let Some(tex) = Texture::from_file(texture_path) {
            self.textures.push(tex);
        } else {
            panic!("Cannot load texture at: {}", texture_path);
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
