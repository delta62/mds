use std::{fmt::Display, io};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    MultiTrackNotSupported,
    NoDataTracks,
    ParseError,
    WrongFileTypeGiven,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "{err}"),
            Self::MultiTrackNotSupported => write!(f, "Multi track conversion not yet supported"),
            Self::NoDataTracks => write!(f, "There are no data tracks in this mdf"),
            Self::ParseError => write!(f, "Error parsing mds file"),
            Self::WrongFileTypeGiven => write!(f, "The given file is not an .mds file"),
        }
    }
}

impl std::error::Error for Error {}
