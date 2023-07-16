use super::types::{Bytes, Res};
use nom::{
    bytes::complete::{tag, take},
    combinator::map_res,
    multi::count,
    number::complete::{le_u16, le_u32, le_u8},
    sequence::tuple,
};

pub type Version = [u8; 2];

#[derive(Debug)]
pub struct Header {
    version: Version,
    media_type: MediaType,
    num_sessions: u16,
    session_offset: u32,
}

impl Header {
    pub fn num_sessions(&self) -> usize {
        self.num_sessions.into()
    }

    pub fn session_offset(&self) -> usize {
        self.session_offset.try_into().unwrap()
    }
}

#[derive(Debug)]
pub enum MediaType {
    CdRom,
    CdR,
    CdRw,
    DvdRom,
    DcdR,
}

pub struct UnknownMediaType(pub u16);

impl TryInto<MediaType> for u16 {
    type Error = UnknownMediaType;

    fn try_into(self) -> Result<MediaType, Self::Error> {
        use MediaType::*;

        match self {
            0x00 => Ok(CdRom),
            0x01 => Ok(CdR),
            0x02 => Ok(CdRw),
            0x10 => Ok(DvdRom),
            0x12 => Ok(DcdR),
            x => Err(UnknownMediaType(x)),
        }
    }
}

pub fn header(input: Bytes) -> Res<Header> {
    let (input, (_, version, media_type, num_sessions, _, session_offset)) =
        tuple((id, version, media_type, le_u16, dvd_padding, le_u32))(input)?;

    let header = Header {
        version,
        media_type,
        num_sessions,
        session_offset,
    };

    Ok((input, header))
}

fn id(input: Bytes) -> Res<Bytes> {
    tag("MEDIA DESCRIPTOR")(input)
}

fn version(input: Bytes) -> Res<[u8; 2]> {
    map_res(count(le_u8, 2), |x| x.try_into())(input)
}

fn media_type(input: Bytes) -> Res<MediaType> {
    map_res(le_u16, |x| x.try_into())(input)
}

fn dvd_padding(input: Bytes) -> Res<Bytes> {
    take(0x3Ausize)(input)
}
