mod display;
mod engine;
mod image;
mod scene;

fn main() {
    display::run(engine::Game::default());
}
