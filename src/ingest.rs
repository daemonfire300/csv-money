use std::{
    fs::{File, OpenOptions},
    io::Stdin,
    path::Path,
};

pub(crate) fn default_csv_ingest(filename: &Path) -> std::io::Result<csv::Reader<File>> {
    let f = OpenOptions::new().read(true).open(filename)?;
    // NOTE(juf): The buffer size can/should be adjusted based on the use-case.
    // NOTE(juf): csv states input is already buffered, so we opt for not pre-pending another
    // BufWriter here.
    let reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_reader(f);
    Ok(reader)
}
