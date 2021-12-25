use crate::engine::Game;
// use crate::image::Image;
use image::io::Reader as ImageReader;
use image::*;
use minifb::{Key, Window, WindowOptions};

pub(crate) fn run(mut game: Game) -> Result<(), Box<dyn std::error::Error>> {
    // let sprite = Image::load_gif("resources/sonic/Data/Sprites/Title/Title.gif")?;

    let image = ImageReader::open("resources/sonic/Data/Sprites/Title/Title.gif")?.decode()?;
    let (width, height) = image.dimensions();

    println!("{} x {}", width, height);

    let mut window = Window::new(
        &game.config.window_title,
        width as usize,
        height as usize,
        WindowOptions::default(),
    )?;

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let u32_buffer: Vec<u32> = image
        .into_rgb8()
        .chunks(3)
        .map(|v| ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | v[2] as u32)
        .collect();

    while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
        window.update_with_buffer(&u32_buffer, width as usize, height as usize)?;
    }

    Ok(())
}
