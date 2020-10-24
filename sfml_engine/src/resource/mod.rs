use std::{error::Error, fmt, path::PathBuf};

pub mod font;
pub mod texture;

pub use font::{Font, FontLoadingError};
pub use texture::{Texture, TextureLoadingError};

pub trait FileResource: Sized {
    fn load_from_path(path: &str) -> Result<Self, Box<dyn Error>>;
    fn load_from_res(name: &str) -> Result<Self, Box<dyn Error>> {
        let mut path_buf = PathBuf::new();
        path_buf.push("resources");
        path_buf.push(name);
        path_buf.to_str().map_or_else(
            || Err(InvalidResourceName(name.to_owned()).into()),
            |path| FileResource::load_from_path(path),
        )
    }
}

#[derive(Debug)]
pub struct InvalidResourceName(String);

impl Error for InvalidResourceName {}

impl fmt::Display for InvalidResourceName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid resource name: \"{}\"!", self.0)
    }
}

#[derive(Debug)]
pub struct ResourceNotFoundError(String);

impl Error for ResourceNotFoundError {}

impl fmt::Display for ResourceNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Resource \"{}\" not found!", self.0)
    }
}
