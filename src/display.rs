use crate::engine::Game;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

pub(crate) fn run(mut game: Game) {
    // let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    // let mut buffer: Vec<u8> = vec![0; WIDTH * HEIGHT];
    // load gif
    // let mut decoder = gif::DecodeOptions::new();
    // decoder.set_color_output(gif::ColorOutput::RGBA);
    // let file = std::fs::File::open("resources/sonic/Data/Sprites/Title/Title.gif").unwrap();
    // let mut decoder = decoder.read_info(file).unwrap();
    // let width = decoder.width();
    // let height = decoder.height();
    // let size = width as usize * height as usize;
    // let mut buffer: &mut Vec<u8> = &mut vec![0u8; size];
    // println!("{} x {}", width, height);

    let file = std::fs::File::open("resources/sonic/Data/Sprites/Title/Title.gif")
        .expect("failed to open input file");
    let mut decoder = {
        let mut options = gif::DecodeOptions::new();
        options.allow_unknown_blocks(true);
        options.read_info(file).unwrap()
    };
    let width = decoder.width() as usize;
    let height = decoder.height() as usize;
    let frame = decoder.read_next_frame().unwrap().unwrap();

    println!("{} x {}", width, height);

    // decoder
    // .read_into_buffer(&mut buffer)
    // .fill_buffer(&mut buffer)
    // .expect("image did not read into buffer");
    // println!("{:?}", buffer);

    // let u32_buffer: Vec<u32> = frame
    //     .buffer
    //     .chunks(4)
    //     .map(|v| ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | v[2] as u32)
    //     .collect();
    let u32_buffer: Vec<u32> = frame
        .buffer
        .iter()
        .map(|v| u32::try_from(*v).unwrap())
        .collect();

    // let u32_buffer: Vec<u32> = buffer.iter().map(|v| u32::from(v)).collect();

    let mut window = Window::new(
        "Test - ESC to exit",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(
                &u32_buffer,
                decoder.width() as usize,
                decoder.height() as usize,
            )
            .unwrap();
    }
}
