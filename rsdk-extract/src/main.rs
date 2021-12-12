use std::env;

mod container;
mod dictionary;
mod extract;
mod read;

fn main() -> std::io::Result<()> {
    println!("RSDK Extractor");

    let args: Vec<String> = env::args().collect();

    if let Some(filename) = args.get(1) {
        println!("Reading {}", filename);

        // let file = std::fs::read(filename)?;
        // match read::container(&file) {
        match extract::load(&filename) {
            Ok(s) => println!("{:?}", s),
            Err(e) => println!("error parsing {:?}", e),
        }
    } else {
        println!("Usage: rsdk-extract <filename>");
    }

    println!("done.");
    Ok(())
}
