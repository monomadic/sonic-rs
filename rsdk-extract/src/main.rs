use std::env;

mod container;
mod extract;
mod read;

fn main() -> std::io::Result<()> {
    println!("RSDK Extractor");

    let args: Vec<String> = env::args().collect();

    if let Some(filename) = args.get(1) {
        println!("Reading {}", filename);

        let file = std::fs::read(filename)?;

        match extract::extract(&file) {
            Ok(_) => println!("done."),
            Err(e) => println!("error parsing {:?}", e),
        }
    } else {
        println!("Usage: rsdk-extract <filename>");
    }

    println!("done.");
    Ok(())
}
