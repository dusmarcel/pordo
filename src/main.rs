use std::path::PathBuf;

use clap::{arg, command, value_parser};

fn main() {
    let matches = command!()
        .arg(
            arg!(
                <FILE> "Input file"
            )
            .value_parser(value_parser!(PathBuf))
        )
        .get_matches();

    if let Some(file) = matches.get_one::<PathBuf>("FILE") {
            println!("Hello, {}!", file.display());
    }
}
