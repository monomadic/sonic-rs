use image::*;

pub struct Surface {
    pub buffer: DynamicImage,
    pub blend_mode: BlendMode,
}

#[derive(Clone)]
pub enum BlendMode {
    Opaque,
    Alpha1bit,
    Alpha8bit,
}

impl Surface {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            buffer: image::DynamicImage::new_rgb8(width, height), // use rgba8 for alpha blending
            blend_mode: BlendMode::Opaque,
        }
    }

    pub fn from_image<P: AsRef<std::path::Path>>(
        path: P,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            buffer: io::Reader::open(path)?.decode()?,
            blend_mode: BlendMode::Alpha1bit,
        })
    }

    pub fn width(&self) -> u32 {
        self.buffer.width()
    }

    pub fn height(&self) -> u32 {
        self.buffer.height()
    }

    pub fn composite(
        &mut self,
        surface: &Surface,
        offset_x: u32,
        offset_y: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(match surface.blend_mode {
            BlendMode::Opaque => self.buffer.copy_from(&surface.buffer, offset_x, offset_y)?,
            BlendMode::Alpha1bit => self.blend1bit(surface, offset_x, offset_y),
            BlendMode::Alpha8bit => self.blend8bit(surface, offset_x, offset_y),
        })
    }

    pub fn fill(&mut self, r: u8, g: u8, b: u8, a: u8) {
        let p = image::Rgba([r, g, b, a]);
        for y in 0..self.buffer.height() {
            for x in 0..self.buffer.width() {
                self.buffer.put_pixel(x, y, p);
            }
        }
    }

    pub fn crop_into(&self, x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            buffer: self.buffer.crop_imm(x, y, width, height),
            blend_mode: self.blend_mode.clone(),
        }
    }

    pub fn crop(&self, x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            buffer: self.buffer.clone().crop(x, y, width, height),
            blend_mode: self.blend_mode.clone(),
        }
    }

    fn blend1bit(&mut self, surface: &Surface, offset_x: u32, offset_y: u32) {
        for y in 0..(surface.buffer.height()) {
            if (y + offset_y) >= self.buffer.height() {
                break;
            }

            for x in 0..(surface.buffer.width()) {
                if (x + offset_x) >= self.buffer.width() {
                    break;
                }
                let p = surface.buffer.get_pixel(x, y);
                // if magenta (1-bit transparent pixel) don't draw
                if p[0] == 255 && p[1] == 0 && p[2] == 255 {
                    // alpha blending
                    // screen.blend_pixel(i + 0, k + 0, p);
                } else {
                    self.buffer.put_pixel(x + offset_x, y + offset_y, p);
                }
            }
        }
    }

    fn blend8bit(&mut self, surface: &Surface, offset_x: u32, offset_y: u32) {
        for k in 0..surface.buffer.height() {
            for i in 0..surface.buffer.width() {
                let p = surface.buffer.get_pixel(i, k);
                if p[0] == 255 && p[1] == 0 && p[2] == 255 {
                    // alpha blending
                    self.buffer.blend_pixel(i + 0, k + 0, p);
                } else {
                    // not transparent, just copy
                    self.buffer.put_pixel(i + 0, k + 0, p);
                }
            }
        }
    }

    pub fn as_u32(&self) -> Vec<u32> {
        self.buffer
            .clone()
            .into_rgba8()
            .chunks(4)
            .map(|v| ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | v[2] as u32)
            .collect()
    }
}
