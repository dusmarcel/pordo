use std::{fs::File, path::PathBuf};

use anyhow::{Result, bail};
use clap::{arg, command, value_parser};
use lopdf::{self, Document};

fn main() -> Result<()> {
    let matches = command!()
        .arg(
            arg!(
                <FILE> "Input file"
            )
            .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let Some(file) = matches.get_one::<PathBuf>("FILE") else {
        bail!("Input file is required");
    };

    let doc = Document::load(file)?;
    let pages = doc.get_pages();
    println!("File {} has {} pages", file.display(), pages.len());

    for (page_number, _) in pages.iter() {
        println!("Page {}", page_number);

        let mut pdoc = doc.clone();
        let pages_to_delete = pages
            .keys()
            .filter(|other_page_number| *other_page_number != page_number)
            .copied()
            .collect::<Vec<_>>();
        pdoc.delete_pages(&pages_to_delete);
        pdoc.prune_objects();

        let mut file = File::create(format!("page_{}.pdf", page_number))?;
        pdoc.save_modern(&mut file)?;
    }

    Ok(())
}
