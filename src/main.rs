use serde::Deserialize;
use std::{fs::File, io::BufReader, path::PathBuf, process};

use args::{Args, Commands};
use clap::Parser;
use csv::Writer;

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

    let mut entries: Vec<Entry> = match extension.to_str() {
        Some("csv") => get_csv(&file),
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

    export_csv(String::from("output"), entries);
}

fn get_csv(file: &File) -> Vec<Entry> {
    let reader = BufReader::new(file);
    let mut csv_reader = csv::Reader::from_reader(reader);

    let mut content: Vec<Entry> = vec![];

    for entry in csv_reader.deserialize() {
        let entry: Entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("Error while deserializing csv!\n{}", err);
                process::exit(1);
            }
        };

        content.push(entry);
    }

    content
}

fn export_csv(file_name: String, entries: Vec<Entry>) {
    let mut writer = match Writer::from_path(format!("{}.csv", file_name)) {
        Ok(writer) => writer,
        Err(err) => {
            eprintln!("Error initialising CSV writer!\n{}", err);
            process::exit(1);
        }
    };

    for entry in entries {
        writer
            .write_record(&[entry.name, entry.value.to_string()])
            .unwrap_or_else(|err| {
                eprintln!("Error while writing to output csv:\n{}", err);
                process::exit(1);
            });
    }

    writer.flush().unwrap_or_else(|err| {
        eprintln!("Error while flushing CSV writer:\n{}", err);
        process::exit(1);
    })
}

#[derive(Debug, Deserialize)]
struct Entry {
    name: String,
    value: f64,
}
