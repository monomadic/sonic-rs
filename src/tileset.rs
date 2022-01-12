use crate::surface::Surface;
use std::path::Path;

pub struct Tileset {
    map: Surface,
    tile: [u32; 255],
}

impl Tileset {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            map: Surface::from_image(path)?,
            tile: [0; 255],
        })
    }

    pub fn get(&self, index: u32) -> Option<Surface> {
        Some(self.map.crop(0, index * 16, 16, 16))
    }
}
