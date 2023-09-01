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
    #[error("term_lib threw an error")]
    TermError(#[from] term_lib::Error),

    #[error("part not found by id: {part_id}")]
    PartNotFoundId { part_id: u32 },
    #[error("part not found by name: {name}")]
    PartNotFoundName { name: String },
    #[error("part already exists in database: {part_id}")]
    PartAlreadyExists { part_id: u32 },

    #[error("command {cmd} cannot be executed in mode {mode}")]
    CmdModeMismatch { cmd: String, mode: String },

    #[error("escape was pressed")]
    Escape,
    #[error("signal to quit program was sent")]
    Quit,
}

// impl From<std::io::Error> for Error {
//     fn from(err: std::io::Error) -> Self {
//         Error::TermError(term_lib::Error::IOError(err))
//     }
// }

pub type Result<T> = std::result::Result<T, Error>;
