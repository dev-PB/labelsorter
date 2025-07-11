use std::{fs::File, path::PathBuf, process};

use args::{Args, Commands};
use clap::Parser;

mod args;

fn main() {
    println!("Hello, world!");

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

    match extension.to_str() {
        Some("json") => get_json(&file),
        _ => {
            eprintln!("The filetype of the specified file is unsupported.");
            process::exit(1);
        }
    }
}

fn get_json(file: &File) {
    todo!()
}
