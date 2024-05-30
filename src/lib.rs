use std::error::Error;
use std::fs::read_to_string;
use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};
use clap::Parser;
use walkdir::WalkDir;

#[derive(Debug, Parser)]
#[command(author, version, about)]
///Yet another one Rust version of `grep`
pub struct Args {
    /// Directory to search
    #[arg(
        short('d'),
        long("dir"),
        value_name = "DIR",
        help = "Directory to search",
        required = true
    )]
    directory: String,

    /// Search term
    #[arg(
        short('s'),
        long("term"),
        value_name = "TERM",
        help = "String to search for",
        required = true
    )]
    search_term: String,

}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let dir = args.directory;

    let search_term = args.search_term;

    search_dir(Path::new(&dir), &search_term)?;

    Ok(())
}

pub fn search_file(path: &Path, search_term: &str) -> Result<usize, Box<dyn Error>> {
    let contents = read_to_string(path)
        .with_context(|| format!("could not read file `{}`", path.display()))?;
    let count = contents.matches(search_term).count();

    Ok(count)
}


pub fn search_dir(dir: &Path, search_term: &str) -> Result<(), Box<dyn Error>> {
    let results = WalkDir::new(dir)
        .into_iter()

        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if !path.is_file() {
                return None;
            }

            Some((path.to_path_buf(), search_term.to_owned()))
        })
        .map(|(path, term)| (path.clone(), search_file(&path, &term)))
        .collect::<Vec<_>>();

    for (key, value) in results {
        writeln!(std::io::stdout(), "{:?}: {}", key, value?)?;
    }


    Ok(())
}
