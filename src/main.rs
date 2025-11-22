use std::{env::args, error::Error, fs::OpenOptions};

pub(crate) mod deserialize;
pub(crate) mod objects;
pub(crate) mod processor;

fn main() -> Result<(), Box<dyn Error>> {
    let input_file_name = if let Some(name) = args().next() {
        name
    } else {
        // TODO(juf): Add nicer error
        panic!("Requires input file")
    };
    let mut f = OpenOptions::new().read(true).open(&input_file_name)?;
    // TODO(juf): Remove trim from underlying Deserializer
    let mut reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_reader(&mut f);

    Ok(())
}
