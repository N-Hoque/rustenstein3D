use rsfml::{graphics::Texture, SfBox};

#[derive(Default)]
pub struct TextureLoader {
    textures: Box<[SfBox<Texture>]>,
}

impl TextureLoader {
    pub(crate) fn with_textures(texture_paths: &[&str]) -> Self {
        let textures = texture_paths
            .iter()
            .map(|t| {
                Texture::from_file(t).unwrap_or_else(|_| panic!("Loading texture from: {}", t))
            })
            .collect();
        Self { textures }
    }

    pub(crate) fn get_texture(&self, index: i32) -> &Texture {
        &self.textures[index as usize]
    }
}
