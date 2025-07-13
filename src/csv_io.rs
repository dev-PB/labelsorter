use std::{fs::File, io::BufReader, process};

use csv::Writer;

use crate::Entry;

pub fn get_csv(file: &File) -> Vec<Entry> {
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

pub fn export_csv(file_name: String, entries: Vec<Entry>) {
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
