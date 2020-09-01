pub mod font;
pub mod texture;

use std::{fmt, error::Error, path::PathBuf};

#[derive(Debug, Clone)]
pub struct LoadingResourceError {
    resource_name: String
}

impl fmt::Display for LoadingResourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to load resource \"{}\"!", self.resource_name)
    }
}

impl Error for LoadingResourceError {}

pub trait Resource: Sized {
    fn load_from(path: &str) -> Result<Self, Box<dyn Error>>;
}

#[derive(Debug)]
pub struct InvalidResourcePathError {
    resource_name: String
}

impl Error for InvalidResourcePathError {}

impl fmt::Display for InvalidResourcePathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Resource path for resource \"{}\" is invalid!", self.resource_name)
    }
}

pub fn get_resource_path(resource_name: &str) -> Result<String, InvalidResourcePathError> {
    let mut pb = PathBuf::new();
    pb.push("resources");
    pb.push(resource_name);
    pb.to_str().map_or_else(
        || Err(InvalidResourcePathError{ resource_name: resource_name.to_owned() }), 
        |s| Ok(s.to_owned()))
}