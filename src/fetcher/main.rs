// This is free and unencumbered software released into the public domain.

#[cfg(feature = "std")]
fn main() -> Result<clientele::SysexitsError, Box<dyn std::error::Error>> {
    use asimov_near_module::BlockUrl;
    use clientele::SysexitsError::*;
    use std::io::stdout;

    // Load environment variables from `.env`:
    clientele::dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = clientele::args_os()?;

    // Configure logging & tracing:
    #[cfg(feature = "tracing")]
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing_subscriber::filter::LevelFilter::WARN)
        .init();

    // Parse URLs from command-line arguments:
    let urls: Vec<String> = args
        .iter()
        .skip(1)
        .map(|arg| arg.to_string_lossy().into())
        .collect();
    if urls.is_empty() {
        return Ok(EX_OK);
    }

    // Process each of the given URL arguments:
    for url in urls {
        let block: BlockUrl = url.parse()?;
        let block_json = block.fetch()?;

        // Serialize the response data:
        if cfg!(feature = "pretty") {
            let block_json: serde_json::Value = serde_json::from_str(&block_json)?;
            colored_json::write_colored_json(&block_json, &mut stdout())?;
            println!();
        } else {
            println!("{}", block_json);
        }
    }

    Ok(EX_OK)
}

#[cfg(not(feature = "std"))]
fn main() {
    unimplemented!("asimov-near-fetcher requires the 'std' feature")
}
