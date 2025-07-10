use std::{fs::File, path::PathBuf, process};

use clap::{Parser, Subcommand};

fn main() {
    println!("Hello, world!");

    let args = Args::parse();

    match args.command {
        Commands::Sort { path } => sort(path),
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Sort { path: PathBuf },
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
