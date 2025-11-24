use std::{
    fs::{File, OpenOptions},
    io::{BufReader, Stdin},
    path::Path,
};

pub(crate) fn default_csv_ingest(filename: &Path) -> std::io::Result<csv::Reader<BufReader<File>>> {
    let f = OpenOptions::new().read(true).open(filename)?;
    // NOTE(juf): The buffer size can/should be adjusted based on the use-case.
    let buf = BufReader::new(f);
    let reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_reader(buf);
    Ok(reader)
}

pub(crate) fn ingest_from_stdin() -> std::io::Result<csv::Reader<BufReader<Stdin>>> {
    // NOTE(juf): The buffer size can/should be adjusted based on the use-case.
    let buf = BufReader::new(std::io::stdin());
    let reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_reader(buf);
    Ok(reader)
}
