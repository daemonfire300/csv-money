use std::{env::args, path::Path};

use crate::{
    ingest::default_csv_ingest,
    objects::transactions::Transaction,
    processor::Processor,
};

pub(crate) mod deserialize;
pub(crate) mod egress;
pub(crate) mod error;
pub(crate) mod ingest;
pub(crate) mod objects;
pub(crate) mod processor;
pub(crate) mod serialize;

fn main() -> Result<(), error::Error> {
    let input_file_name = if let Some(name) = args().next() {
        name
    } else {
        // TODO(juf): Add nicer error
        return Err(error::Error::MissingArgument);
    };
    let mut ingest = default_csv_ingest(Path::new(&input_file_name))?;
    let mut p = Processor::new();
    let iter = ingest.deserialize();
    for row in iter {
        let txn: Transaction = row?;
        p.process_one(txn);
    }

    Ok(())
}
