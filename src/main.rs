mod display;
mod engine;
mod scene;

fn main() {
    display::run(engine::Game::default());
}
