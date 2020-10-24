use sfml::{graphics, system::SfBox};
use std::{error::Error, fmt};

use super::FileResource;

pub struct Texture(SfBox<graphics::Texture>);

impl FileResource for Texture {
    fn load_from_path(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        graphics::Texture::from_file(path).map_or_else(
            || Err(TextureLoadingError(path.to_owned()).into()),
            |texture| Ok(Texture(texture)),
        )
    }
}

#[derive(Debug)]
pub struct TextureLoadingError(String);

impl Error for TextureLoadingError {}

impl fmt::Display for TextureLoadingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to load texture \"{}\"!", self.0)
    }
}
