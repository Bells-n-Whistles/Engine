use sfml::{graphics, system::SfBox};
use std::{error::Error, fmt};

use super::FileResource;

pub struct Font(SfBox<graphics::Font>);

impl FileResource for Font {
    fn load_from_path(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        graphics::Font::from_file(path).map_or_else(
            || Err(FontLoadingError(path.to_owned()).into()),
            |font| Ok(Font(font)),
        )
    }
}

#[derive(Debug)]
pub struct FontLoadingError(String);

impl Error for FontLoadingError {}

impl fmt::Display for FontLoadingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to load font \"{}\"!", self.0)
    }
}
