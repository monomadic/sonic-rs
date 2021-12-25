use structopt::StructOpt;
// use std::env;
use std::path::PathBuf;

pub(crate) fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("RSDK Extractor v0.1.1\n");

    let args = Args::from_args();
    let input = args.input;

    info!("Reading {:?}", input);

    crate::extract::read(&input)?;

    if !args.skip_postprocessor {
        crate::postprocessor::run()?;
    }

    Ok(())
}

#[derive(StructOpt)]
#[structopt(name = "extract", about = "RSDK Extractor")]
struct Args {
    /// Input file
    #[structopt(parse(from_os_str), default_value = "Data.rsdk")]
    input: PathBuf,

    /// Skip postprocessing files after extraction
    #[structopt(long = "skip-postprocessor")]
    skip_postprocessor: bool,
}
