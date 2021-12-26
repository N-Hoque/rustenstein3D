use rsfml::graphics::Texture;

use super::TextureLoader;

impl TextureLoader {
    pub fn with_textures(texture_paths: &[&str]) -> Self {
        let textures = texture_paths
            .iter()
            .map(|t| Texture::from_file(t).unwrap_or_else(|| panic!("Loading texture from: {}", t)))
            .collect();
        Self { textures }
    }

    pub fn load_textures(&mut self, texture_paths: &[&str]) {
        texture_paths.iter().for_each(|texture_path| {
            self.load_texture(texture_path);
        });
    }

    pub fn load_texture(&mut self, texture_path: &str) {
        Texture::from_file(texture_path).map_or_else(
            || panic!("Loading texture from: {}", texture_path),
            |tex| self.textures.push(tex),
        );
    }

    pub fn get_texture(&self, index: i32) -> &Texture {
        &self.textures[index as usize]
    }
}
