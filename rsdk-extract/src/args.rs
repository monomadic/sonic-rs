use structopt::StructOpt;
// use std::env;
use std::path::PathBuf;

pub(crate) fn run() -> std::io::Result<()> {
    println!("RSDK Extractor v0.1.1\n");

    match Commands::from_args() {
        Commands::Extract { input } => {
            info!("Reading {:?}", input);

            match crate::extract::read(&input) {
                Ok(s) => println!("{:?}", s),
                Err(e) => println!("error parsing {:?}", e),
            }
        }
        Commands::Process => {
            info!("reading resources/Data/Game/GameConfig.bin");
            let file = std::fs::read("resources/Data/Game/GameConfig.bin")?;
            let config = crate::gameconfig::extract(&file);
            let json = serde_json::to_string(&config).unwrap();
            info!("writing resources/Data/Game/GameConfig.json");
            std::fs::write("resources/Data/Game/GameConfig.json", json)?;
        }
    }

    Ok(())
}

#[derive(StructOpt)]
#[structopt(name = "extract", about = "RSDK Extractor")]
enum Commands {
    Extract {
        /// Input file
        #[structopt(
            parse(from_os_str),
            short = "i",
            long = "input",
            default_value = "Data.rsdk"
        )]
        input: PathBuf,
    },
    Process,
}
