pub struct Tileset {
    tile_map: Surface,
}

impl Tileset {
    fn load<AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            tile_map: Surface::from_image(path)?
        })
    }

    fn get(&self, index: usize) -> Option<&Surface> {
        None
    }
}