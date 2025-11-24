use std::{
    fs::{File, OpenOptions},
    io::BufWriter,
    path::Path,
};

pub(crate) fn default_csv_egress(filename: &Path) -> std::io::Result<csv::Writer<BufWriter<File>>> {
    let f = OpenOptions::new().write(true).create(true).open(filename)?;
    // NOTE(juf): The buffer size can/should be adjusted based on the use-case.
    let buf = BufWriter::new(f);
    let writer = csv::WriterBuilder::new().from_writer(buf);
    Ok(writer)
}
