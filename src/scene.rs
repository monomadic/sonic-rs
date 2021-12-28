use crate::surface::Surface;

pub trait Object {
    fn init(&self);
    fn next(&mut self) -> &Surface;
}

pub struct SegaLogo {
    frame: usize,
    sega_text: Surface,
}

impl SegaLogo {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let sprites = Surface::from_image("resources/sonic/Data/Sprites/Title/Title.gif")?;
        let title = sprites.crop_into(323, 242, 188, 58);
        Ok(Self {
            frame: 0,
            sega_text: title,
        })
    }
}

impl Object for SegaLogo {
    fn init(&self) {}
    fn next(&mut self) -> &Surface {
        &self.sega_text
    }
}

pub struct BigSonic {
    frame: usize,
    frameskip: usize,
    frames: Vec<Surface>,
}

impl BigSonic {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let sprites =
            crate::surface::Surface::from_image("resources/sonic/Data/Sprites/Title/Title.gif")?;

        Ok(Self {
            frame: 0,
            frameskip: 4,
            frames: vec![
                sprites.crop_into(1, 1, 65, 88),
                sprites.crop_into(67, 1, 74, 88),
                sprites.crop_into(142, 1, 74, 88),
                sprites.crop_into(217, 1, 79, 88),
                sprites.crop_into(297, 1, 72, 88),
                sprites.crop_into(370, 1, 81, 88),
                sprites.crop_into(1, 90, 81, 88),
                sprites.crop_into(83, 90, 81, 88),
            ],
        })
    }
}

impl Object for BigSonic {
    fn init(&self) {}
    fn next(&mut self) -> &Surface {
        let mut sprite_frame = self.frame / self.frameskip;

        if sprite_frame > self.frames.len() - 1 {
            sprite_frame = 0;
            self.frame = 1;
        } else {
            self.frame = self.frame + 1
        };

        &self.frames[sprite_frame]
    }
}
