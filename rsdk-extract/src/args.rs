use std::path::PathBuf;
use structopt::StructOpt;

pub(crate) fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("RSDK Extractor v0.1.1\n");

    let args = Args::from_args();
    let input = args.input;

    info!("Reading {:?}", input);

    let buffer = std::fs::read(input)?;
    let output_path = crate::detect::output_path(&buffer);

    if !args.skip_extractor {
        crate::extract::rsdkv4(&buffer, &output_path)?;
    }

    // if !args.skip_postprocessor {
    //     crate::postprocessor::run(&output_path)?;
    // }

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

    /// Skip postprocessing files after extraction
    #[structopt(long = "skip-extractor")]
    skip_extractor: bool,
}
