pub(crate) struct Image {
    pub(crate) buffer: Vec<u8>,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl Image {
    pub(crate) fn load_gif(path: &str) -> Result<Image, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        let mut decoder = {
            let mut options = gif::DecodeOptions::new();
            options.allow_unknown_blocks(true);
            options.read_info(file).unwrap()
        };
        let width = decoder.width() as u32;
        let height = decoder.height() as u32;
        let frame = decoder.read_next_frame().unwrap().unwrap();
        // info!("{} ({} x {})", &path, &width, &height);
        Ok(Image {
            buffer: frame.buffer.to_vec(),
            width,
            height,
        })
    }

    pub(crate) fn buffer_u32(&self) -> Vec<u32> {
        self.buffer
            .iter()
            .map(|v| u32::try_from(*v).unwrap())
            .collect()
    }
}

// pub(crate) gif_sprite(spritesheet: &[u8])
