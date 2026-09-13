use std::path::PathBuf;

use anyhow::{Result, bail};
use clap::{arg, command, value_parser};

fn main() -> Result<()> {
    let matches = command!()
        .arg(
            arg!(
                <FILE> "Input file"
            )
            .value_parser(value_parser!(PathBuf))
        )
        .get_matches();

    let Some(file) = matches.get_one::<PathBuf>("FILE") else {
        bail!("Input file is required");
    };

    Ok(())
}
