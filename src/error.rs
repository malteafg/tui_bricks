use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error")]
    IOError(#[from] std::io::Error),
    #[error("yaml serialization error")]
    SerdeError(#[from] serde_yaml::Error),

    #[error("parsing error")]
    ParsingError(#[from] std::num::ParseIntError),

    #[error("part not found: {part_id}")]
    PartNotFound { part_id: u32 },
    #[error("part already exists in database: {part_id}")]
    PartExists { part_id: u32 },

    #[error("command {cmd} cannot be executed in mode {mode}")]
    CmdModeMismatch { cmd: String, mode: String },
}

pub type Result<T> = std::result::Result<T, Error>;
