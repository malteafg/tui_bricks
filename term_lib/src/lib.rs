use thiserror::Error;

pub mod cmd;
pub mod display;
pub mod input;

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

    #[error("escape was pressed")]
    Escape,
    #[error("signal to quit program was sent")]
    Quit,
}

pub type Result<T> = std::result::Result<T, Error>;
