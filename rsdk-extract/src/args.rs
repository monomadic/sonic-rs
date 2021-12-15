use structopt::StructOpt;
// use std::env;
use std::path::PathBuf;

pub(crate) fn run() -> std::io::Result<()> {
    println!("RSDK Extractor");

    match Commands::from_args() {
        Commands::Extract { input } => {}
        Commands::Process => {
            let file = std::fs::read("resources/Data/Game/GameConfig.bin")?;
            crate::gameconfig::extract(&file);
        }
    }

    // App::new("rsdk-extract")
    //     .subcommand(App::new("extract").arg(Arg::with_name("input")))
    //     .get_matches();

    // let args: Vec<String> = env::args().collect();

    // if let Some(filename) = args.get(1) {
    //     println!("Reading {}", filename);

    //     // let file = std::fs::read(filename)?;
    //     // match container::container(&file) {
    //     match crate::extract::load(&filename) {
    //         Ok(s) => println!("{:?}", s),
    //         Err(e) => println!("error parsing {:?}", e),
    //     }
    // } else {
    //     println!("Usage: rsdk-extract <filename>");
    // }

    println!("done.");
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
