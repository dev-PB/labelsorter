use std::{fs::File, path::PathBuf, process};

use args::{Args, Commands};
use clap::Parser;

mod args;
mod csv_io;
mod entry;

use entry::Entry;

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Sort { path } => sort(path),
    }
}

fn sort(path: PathBuf) {
    let extension = match path.extension() {
        Some(ext) => ext,
        None => {
            println!("File has no extension.");
            process::exit(1)
        }
    };

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error while opening file!\n{}", err);
            process::exit(1);
        }
    };

    let mut entries: Vec<Entry> = match extension.to_str() {
        Some("csv") => csv_io::get_csv(&file),
        _ => {
            eprintln!("The filetype of the specified file is unsupported.");
            process::exit(1);
        }
    };

    entries.sort_by(|a, b| {
        a.value
            .partial_cmp(&b.value)
            .expect("Error while sorting file.")
    });

    csv_io::export_csv(String::from("output"), entries);
}
