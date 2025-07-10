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
    let file = match File::open(path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error while opening file!\n{}", err);
            process::exit(1);
        }
    };

    eprintln!("{:?}", file);
}
