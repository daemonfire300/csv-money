use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Stdout},
    path::Path,
};

pub(crate) fn default_csv_egress(filename: &Path) -> std::io::Result<csv::Writer<BufWriter<File>>> {
    let f = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename)?;
    // NOTE(juf): The buffer size can/should be adjusted based on the use-case.
    // NOTE(juf): csv already comes with buffered writes. This is just an example how buffered
    // output could look like if csv would not have it builtin
    let buf = BufWriter::new(f);
    let writer = csv::WriterBuilder::new().from_writer(buf);
    Ok(writer)
}

pub(crate) fn stdout_csv_egress() -> std::io::Result<csv::Writer<BufWriter<Stdout>>> {
    // NOTE(juf): The buffer size can/should be adjusted based on the use-case.
    // NOTE(juf): csv already comes with buffered writes. This is just an example how buffered
    // output could look like if csv would not have it builtin
    let buf = BufWriter::new(std::io::stdout());
    let writer = csv::WriterBuilder::new().from_writer(buf);
    Ok(writer)
}
