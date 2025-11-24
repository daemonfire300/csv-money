use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("csv::Error {0}")]
    FailedDeserialization(#[from] csv::Error),
    #[error("std::io:error {0}")]
    StdIO(#[from] std::io::Error),
    #[error("cli requires input argument to read transactions from")]
    MissingArgument,
}
