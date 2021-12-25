mod display;
mod engine;
mod image;
mod objects;
mod scene;

fn main() {
    let _ = display::run(engine::Game::default());
}
