#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![warn(clippy::pedantic)]

use anyhow::anyhow;

use std::env;
use std::io::BufReader;
use std::fs::File;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let Some(file_path) = &args.get(1) else {
        return Err(anyhow!("Missing argument, usage: cargo run -- <CSV_FILE>"));
    };

    let file = File::open(file_path)?;
    let mut buf_reader = BufReader::new(file);

    engine::process_file(&mut buf_reader, &mut std::io::stdout())?;

    Ok(())
}
