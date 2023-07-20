use std::{fmt::Display, io};

use crate::mds::TrackMode;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    MissingInputFile,
    MultiTrackNotSupported,
    NoDataTracks,
    NoSessions,
    ParseError,
    TooManySessions,
    UnknownCueTrackSize(TrackMode, usize),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;

        match self {
            Io(err) => write!(f, "{err}"),
            MissingInputFile => write!(f, "No input file provided to read data from"),
            MultiTrackNotSupported => write!(f, "Multi track conversion not yet supported"),
            NoDataTracks => write!(f, "There are no data tracks in this mdf"),
            NoSessions => write!(f, "There are no sessions in the image"),
            ParseError => write!(f, "Error parsing mds file"),
            TooManySessions => write!(f, "Cannot convert multi-session images"),
            UnknownCueTrackSize(mode, data_size) => {
                write!(f, "Unusual track type: {mode:?} @ {data_size}")
            }
        }
    }
}

impl std::error::Error for Error {}
