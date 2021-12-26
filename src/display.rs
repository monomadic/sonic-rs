use crate::engine::Game;
use crate::objects::SegaLogo;

use image::*;
use minifb::{Key, Window, WindowOptions};

pub(crate) fn run(mut game: Game) -> Result<(), Box<dyn std::error::Error>> {
    let (width, height) = (320, 240);
    let mut screen = image::DynamicImage::new_rgb8(width, height); // use rgba8 for alpha blending
    screen.invert();

    // let logo = SegaLogo::new();

    let mut sprites = io::Reader::open("resources/sonic/Data/Sprites/Title/Title.gif")?.decode()?;

    // // gifs have 1-bit magenta transparency
    // if let Some(pixels) = sprites.as_mut_rgba8() {
    //     for pixel in pixels.pixels_mut() {
    //         if pixel[0] == 255 && pixel[1] == 0 && pixel[2] == 255 {
    //             pixel[0] = 0;
    //             pixel[1] = 255;
    //             pixel[2] = 255;
    //             pixel[3] = 0;
    //         }
    //     }
    // };

    let sega_title = sprites.crop_imm(323, 242, 188, 58);
    // screen.copy_from(&sega_title, 0, 0)?;

    for k in 0..sega_title.height() {
        for i in 0..sega_title.width() {
            let p = sega_title.get_pixel(i, k);
            // if magenta (1-bit transparent pixel) don't draw
            if p[0] == 255 && p[1] == 0 && p[2] == 255 {
                // alpha blending
                // screen.blend_pixel(i + 0, k + 0, p);
            } else {
                screen.put_pixel(i + 0, k + 0, p);
            }
        }
    }

    let mut window = Window::new(
        &game.config.window_title,
        width as usize,
        height as usize,
        WindowOptions::default(),
    )?;

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let u32_buffer: Vec<u32> = screen
        .into_rgba8()
        .chunks(4)
        .map(|v| ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | v[2] as u32)
        .collect();

    while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
        window.update_with_buffer(&u32_buffer, width as usize, height as usize)?;
    }

    Ok(())
}
