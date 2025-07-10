use std::path::PathBuf;

use clap::{Parser, Subcommand};

fn main() {
    println!("Hello, world!");

    let args = Args::parse();

    match args.command {
        Commands::Sort { path } => todo!(),
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
