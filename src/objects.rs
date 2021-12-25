use image::*;

pub(crate) struct SegaLogo {
    logo_sprite: image::DynamicImage,
}

impl SegaLogo {
    pub(crate) fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let sprites = io::Reader::open("resources/sonic/Data/Sprites/Title/Title.gif")?.decode()?;

        Ok(Self {
            logo_sprite: sprites.crop_imm(323, 242, 188, 58),
        })
    }
    pub(crate) fn init(&mut self) {
        // load both sprites
    }

    pub(crate) fn update(&mut self) {}

    pub(crate) fn draw(&self) {}
}
