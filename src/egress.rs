use std::io::{BufWriter, Stdout};

pub(crate) fn stdout_csv_egress() -> std::io::Result<csv::Writer<BufWriter<Stdout>>> {
    // NOTE(juf): The buffer size can/should be adjusted based on the use-case.
    // NOTE(juf): csv already comes with buffered writes. This is just an example how buffered
    // output could look like if csv would not have it builtin
    let buf = BufWriter::new(std::io::stdout());
    let writer = csv::WriterBuilder::new().from_writer(buf);
    Ok(writer)
}
