use sfml::{graphics, system::SfBox};

use super::{Resource, LoadingResourceError, get_resource_path};

impl Resource for SfBox<graphics::Font> {
    fn load_from(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let p = get_resource_path(path)?;
        graphics::Font::from_file(&p).map_or_else(
            || Err(LoadingResourceError { resource_name: path.to_owned() }.into()),
            |r| Ok(r.into())
        )
    }
}