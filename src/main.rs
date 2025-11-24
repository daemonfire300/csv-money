use std::{env::args, path::Path};

use crate::{
    egress::stdout_csv_egress, ingest::default_csv_ingest, objects::transactions::Transaction,
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
    let input_file_name = if let Some(name) = args().nth(1) {
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
    let mut egress = stdout_csv_egress()?;
    for (_, account) in p.get_account_store_ref().iter() {
        egress.serialize(account)?;
    }

    Ok(())
}
