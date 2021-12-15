use clap::{App, Arg, ArgMatches};
use std::env;
use std::path::PathBuf;

pub(crate) fn run() -> std::io::Result<()> {
    println!("RSDK Extractor");

    let file = std::fs::read("resources/Data/Game/GameConfig.bin")?;
    crate::gameconfig::extract(&file);

    // let opts: Opts = Opts::parse();

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

// #[derive(Parser)]
// #[clap(name = "rsdk-extract")]
// pub struct Opts {
//     #[clap(subcommand)]
//     command: Command,
// }

// #[derive(Parser)]
// enum Command {
//     #[clap()]
//     Extract {
//         /// Input file
//         #[clap(
//             parse(from_os_str),
//             short = 'i',
//             long = "input",
//             default_value = "Data.rsdk"
//         )]
//         input: PathBuf,
//     },
//     #[clap()]
//     Process,
// }
