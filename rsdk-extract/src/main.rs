use std::env;

fn main() {
    println!("RSDK Extractor");

    let args: Vec<String> = env::args().collect();

    if let Some(filename) = args.get(1) {
        println!("Reading {}", filename);
    } else {
        println!("Usage: rsdk-extract <filename>");
    }

    println!("done.");
}
