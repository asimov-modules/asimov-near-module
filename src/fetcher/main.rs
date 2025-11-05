// This is free and unencumbered software released into the public domain.

use clap::Parser;
use clientele::StandardOptions;
use std::io::Write as _;

/// asimov-near-fetcher
#[derive(Debug, Parser)]
#[command(arg_required_else_help = true)]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    /// The output format.
    #[arg(value_name = "FORMAT", short = 'o', long)]
    output: Option<String>,

    urls: Vec<String>,
}

#[cfg(feature = "std")]
fn main() -> Result<clientele::SysexitsError, Box<dyn std::error::Error>> {
    use asimov_near_module::BlockUrl;
    use clientele::SysexitsError::*;
    use std::io::stdout;

    // Load environment variables from `.env`:
    clientele::dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = asimov_module::args_os()?;

    // Parse command-line options:
    let options = Options::parse_from(args);

    // Handle the `--version` flag:
    if options.flags.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(EX_OK);
    }

    // Handle the `--license` flag:
    if options.flags.license {
        print!("{}", include_str!("../../UNLICENSE"));
        return Ok(EX_OK);
    }

    // Configure logging & tracing:
    #[cfg(feature = "tracing")]
    asimov_module::init_tracing_subscriber(&options.flags).expect("failed to initialize logging");

    if options.urls.is_empty() {
        return Ok(EX_OK);
    }

    let mut stdout = stdout().lock();

    // Process each of the given URL arguments:
    for url in options.urls {
        let block: BlockUrl = url.parse()?;
        let block_json = block.fetch()?;

        match writeln!(&mut stdout, "{}", &block_json) {
            Ok(_) => (),
            // break as we can't write to stdout:
            Err(err) if err.kind() == std::io::ErrorKind::BrokenPipe => break,
            Err(err) => return Err(err.into()),
        }
    }

    Ok(EX_OK)
}

#[cfg(not(feature = "std"))]
fn main() {
    unimplemented!("asimov-near-fetcher requires the 'std' feature")
}
