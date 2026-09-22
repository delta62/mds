use super::{
    header::{header, Header, MediaType, Version},
    session::{session, Session, SESSION_SIZE},
    track::Track,
    types::{Bytes, Res},
};
use crate::error::{Error, Result};
use nom::Finish;

#[derive(Debug)]
pub struct Mds {
    header: Header,
    num_bytes: usize,
    sessions: Vec<Session>,
}

impl Mds {
    pub fn sessions(&self) -> impl Iterator<Item = &Session> {
        self.sessions.iter()
    }

    pub fn single_session(&self) -> Result<&Session> {
        if self.sessions.is_empty() {
            Err(Error::NoSessions)?;
        }

        if self.sessions.len() > 1 {
            Err(Error::TooManySessions)?;
        }

        Ok(&self.sessions[0])
    }

    pub fn single_track(&self) -> Result<&Track> {
        self.single_session().and_then(|session| {
            let mut tracks = session.data_tracks();

            let first_track = tracks.next().ok_or(Error::NoDataTracks)?;

            if tracks.next().is_some() {
                Err(Error::MultiTrackNotSupported)?;
            }

            Ok(first_track)
        })
    }

    pub fn version(&self) -> Version {
        self.header.version
    }

    pub fn media_type(&self) -> MediaType {
        self.header.media_type
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        mds(bytes)
            .finish()
            .map(|(_, mds)| mds)
            .map_err(|_| Error::ParseError)
    }

    pub fn byte_len(&self) -> usize {
        self.num_bytes
    }
}

fn mds(input: Bytes) -> Res<Mds> {
    let (mut rest, header) = header(input)?;
    let num_sessions = header.num_sessions();

    let mut sessions = Vec::with_capacity(num_sessions);
    let mut session_offset = header.session_offset();
    let num_bytes = input.len();

    for _ in 0..num_sessions {
        let result = session(input, session_offset)?;

        rest = result.0;
        sessions.push(result.1);

        session_offset += SESSION_SIZE;
    }

    Ok((
        rest,
        Mds {
            num_bytes,
            header,
            sessions,
        },
    ))
}
