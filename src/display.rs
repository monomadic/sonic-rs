use crate::engine::Game;
use crate::image::Image;
use minifb::{Key, Window, WindowOptions};

pub(crate) fn run(mut game: Game) -> Result<(), Box<dyn std::error::Error>> {
    let sprite = Image::load_gif("resources/sonic/Data/Sprites/Title/Title.gif")?;

    println!("{} x {}", sprite.width, sprite.height);

    let mut window = Window::new(
        &game.config.window_title,
        sprite.width as usize,
        sprite.height as usize,
        WindowOptions::default(),
    )?;

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
        window.update_with_buffer(
            &sprite.buffer_u32(),
            sprite.width as usize,
            sprite.height as usize,
        )?;
    }

    Ok(())
}
