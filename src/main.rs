mod display;
mod engine;
mod image;
mod objects;
mod scene;
mod surface;

fn main() {
    match display::run(engine::Game::default()) {
        Ok(_) => println!("done."),
        Err(e) => println!("Error: {:?}", e),
    }
}
