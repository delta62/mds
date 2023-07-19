mod filename;
mod header;
mod index;
mod session;
mod track;
mod types;

use crate::error::{Error, Result};
pub use header::{header, Header};
use nom::Finish;
pub use session::{session, Session};
pub use track::{Track, TrackMode};
use types::{Bytes, Res};

use self::header::{MediaType, Version};

const SESSION_SIZE: usize = 0x18;

#[derive(Debug)]
pub struct Mds {
    header: Header,
    sessions: Vec<Session>,
}

impl Mds {
    pub fn sessions(&self) -> impl Iterator<Item = &Session> {
        self.sessions.iter()
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
}

fn mds(input: Bytes) -> Res<Mds> {
    let (mut rest, header) = header(input)?;
    let num_sessions = header.num_sessions();

    let mut sessions = Vec::with_capacity(num_sessions);
    let mut session_offset = header.session_offset();

    for _ in 0..num_sessions {
        let result = session(input, session_offset)?;

        rest = result.0;
        sessions.push(result.1);

        session_offset += SESSION_SIZE;
    }

    Ok((rest, Mds { header, sessions }))
}
