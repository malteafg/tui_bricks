use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error")]
    IOError(#[from] std::io::Error),
    #[error("yaml serialization error")]
    SerdeError(#[from] serde_yaml::Error),
    #[error("external cmd error")]
    ExternalCmdError,

    #[error("parsing error")]
    ParsingError(#[from] std::num::ParseIntError),
}

pub type Result<T> = std::result::Result<T, Error>;
