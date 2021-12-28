use crate::engine::Game;
use crate::objects::SegaLogo;

use minifb::{Key, Window, WindowOptions};

pub(crate) fn run(mut game: Game) -> Result<(), Box<dyn std::error::Error>> {
    let (width, height) = (320, 240);

    let mut screen = crate::surface::Surface::new(width, height);

    let sprites =
        crate::surface::Surface::from_image("resources/sonic/Data/Sprites/Title/Title.gif")?;
    let title = sprites.crop_into(323, 242, 188, 58);

    screen.composite(&title, 10, 10)?;

    let mut window = Window::new(
        &game.config.window_title,
        width as usize,
        height as usize,
        WindowOptions::default(),
    )?;

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let u32_buffer: Vec<u32> = screen
        .buffer
        .into_rgba8()
        .chunks(4)
        .map(|v| ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | v[2] as u32)
        .collect();

    while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
        window.update_with_buffer(&u32_buffer, width as usize, height as usize)?;
    }

    Ok(())
}
