use std::{fmt::Display, io};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    MultiTrackNotSupported,
    NoSessions,
    NoDataTracks,
    MissingInputFile,
    ParseError,
    TooManySessions,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;

        match self {
            Io(err) => write!(f, "{err}"),
            MultiTrackNotSupported => write!(f, "Multi track conversion not yet supported"),
            NoSessions => write!(f, "There are no sessions in the image"),
            NoDataTracks => write!(f, "There are no data tracks in this mdf"),
            MissingInputFile => write!(f, "No input file provided to read data from"),
            ParseError => write!(f, "Error parsing mds file"),
            TooManySessions => write!(f, "Cannot convert multi-session images"),
        }
    }
}

impl std::error::Error for Error {}
