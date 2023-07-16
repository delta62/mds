use nom::{
    bytes::complete::{tag, take},
    combinator::{map, map_res},
    multi::count,
    number::complete::{le_i32, le_u16, le_u32, le_u64, le_u8},
    sequence::tuple,
    IResult,
};

type Bytes<'a> = &'a [u8];
type Res<'a, T> = IResult<Bytes<'a>, T>;
type Version = [u8; 2];

const SESSION_SIZE: usize = 0x18;
const DATA_BLOCK_SIZE: usize = 0x50;

#[derive(Debug)]
pub enum MediaType {
    CdRom,
    CdR,
    CdRw,
    DvdRom,
    DcdR,
    Other(u16),
}

impl Into<MediaType> for u16 {
    fn into(self) -> MediaType {
        use MediaType::*;

        match self {
            0x00 => CdRom,
            0x01 => CdR,
            0x02 => CdRw,
            0x10 => DvdRom,
            0x12 => DcdR,
            x => Other(x),
        }
    }
}

#[derive(Debug)]
pub enum TrackMode {
    None,
    Audio,
    Mode1,
    Mode2,
    Mode2Form1,
    Mode2Form2,
    Unknown(u8),
}

impl Into<TrackMode> for u8 {
    fn into(self) -> TrackMode {
        use TrackMode::*;

        match self {
            0x00 => None,
            0xA9 => Audio,
            0xAA => Mode1,
            0xAB => Mode2,
            0xAC => Mode2Form1,
            0xAD => Mode2Form2,
            0xEC => Mode2,
            x => Unknown(x),
        }
    }
}

impl Default for TrackMode {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug)]
pub struct Mds {
    header: Header,
    sessions: Vec<Session>,
}

#[derive(Debug)]
pub struct Header {
    version: Version,
    media_type: MediaType,
    num_sessions: u16,
    session_offset: u32,
}

#[derive(Debug)]
pub struct Session {
    start_sector: i32,
    end_sector: i32,
    session_number: u16,
    num_data_blocks: u8,
    num_lead_in_data_blocks: u8,
    first_track_num: u16,
    last_track_num: u16,
    first_block_offset: u32,
    tracks: Vec<Track>,
}

#[derive(Debug, Default)]
pub struct Track {
    track_mode: TrackMode,
    num_subchannels: u8,
    adr: u8,
    track_number: u8,
    point: u8,
    minute: u8,
    second: u8,
    frame: u8,
    index_block_offset: u32,
    sector_size: u16,
    track_start_sector: i32,
    track_start_offset: u64,
    num_filenames: u32,
    filename_offset: u32,
}

fn id(input: Bytes) -> Res<Bytes> {
    tag("MEDIA DESCRIPTOR")(input)
}

fn version(input: Bytes) -> Res<[u8; 2]> {
    map_res(count(le_u8, 2), |x| x.try_into())(input)
}

fn media_type(input: Bytes) -> Res<MediaType> {
    map(le_u16, |x| x.into())(input)
}

fn num_sessions(input: Bytes) -> Res<u16> {
    le_u16(input)
}

fn dvd_padding(input: Bytes) -> Res<Bytes> {
    take(0x3Ausize)(input)
}

fn session_offset(input: Bytes) -> Res<u32> {
    le_u32(input)
}

fn header(input: Bytes) -> Res<Header> {
    let (input, (_, version, media_type, num_sessions, _, session_offset)) = tuple((
        id,
        version,
        media_type,
        num_sessions,
        dvd_padding,
        session_offset,
    ))(input)?;

    let header = Header {
        version,
        media_type,
        num_sessions,
        session_offset,
    };

    Ok((input, header))
}

fn session(input: Bytes, session_offset: usize) -> Res<Session> {
    let (
        rest,
        (
            _,
            start_sector,
            end_sector,
            session_number,
            num_data_blocks,
            num_lead_in_data_blocks,
            first_track_num,
            last_track_num,
            _,
            first_block_offset,
        ),
    ) = tuple((
        take(session_offset),
        session_start_sector,
        session_end_sector,
        session_number,
        num_data_blocks,
        num_lead_in_data_blocks,
        session_first_track_num,
        session_last_track_num,
        le_u32,
        session_first_data_block_offset,
    ))(input)?;

    let data_blocks_offset: usize = first_block_offset.try_into().unwrap();
    let mut tracks = Vec::new();

    for i in 0..num_data_blocks {
        let block_offset = data_blocks_offset + DATA_BLOCK_SIZE * i as usize;
        let (_, track) = track(&input[block_offset..])?;
        tracks.push(track);
    }

    let session = Session {
        start_sector,
        end_sector,
        session_number,
        num_data_blocks,
        num_lead_in_data_blocks,
        first_track_num,
        last_track_num,
        first_block_offset,
        tracks,
    };

    let rest_offset = DATA_BLOCK_SIZE * num_data_blocks as usize;
    let rest = &rest[rest_offset..];

    Ok((rest, session))
}

fn track(input: Bytes) -> Res<Track> {
    let (
        input,
        (
            track_mode,
            num_subchannels,
            adr,
            track_number,
            point,
            _,
            minute,
            second,
            frame,
            index_block_offset,
            sector_size,
            _,
            _,
            track_start_sector,
            track_start_offset,
            num_filenames,
            filename_offset,
            _,
        ),
    ) = tuple((
        track_mode,
        le_u8,           // num subchannels
        le_u8,           // adr/control
        le_u8,           // track number
        le_u8,           // point
        le_u32,          // zero
        le_u8,           // minute
        le_u8,           // second
        le_u8,           // frame
        le_u32,          // index block offset
        le_u16,          // sector size
        le_u8,           // unknown
        take(0x11usize), // zero
        le_i32,          // track start sector
        le_u64,          // track start offset
        le_u32,          // num filenames for this track
        le_u32,          // offset to filename block for this track
        take(0x18usize), // zero
    ))(input)?;

    let b = Track {
        track_mode,
        num_subchannels,
        adr,
        track_number,
        point,
        minute,
        second,
        frame,
        index_block_offset,
        sector_size,
        track_start_sector,
        track_start_offset,
        num_filenames,
        filename_offset,
    };

    println!("{:#?}", b);

    Ok((input, b))
}

fn track_mode(input: Bytes) -> Res<TrackMode> {
    map(le_u8, |x| x.into())(input)
}

fn session_start_sector(input: Bytes) -> Res<i32> {
    le_i32(input)
}

fn session_end_sector(input: Bytes) -> Res<i32> {
    le_i32(input)
}

fn session_number(input: Bytes) -> Res<u16> {
    le_u16(input)
}

fn num_data_blocks(input: Bytes) -> Res<u8> {
    le_u8(input)
}

fn num_lead_in_data_blocks(input: Bytes) -> Res<u8> {
    le_u8(input)
}

fn session_first_track_num(input: Bytes) -> Res<u16> {
    le_u16(input)
}

fn session_last_track_num(input: Bytes) -> Res<u16> {
    le_u16(input)
}

fn session_first_data_block_offset(input: Bytes) -> Res<u32> {
    le_u32(input)
}

pub fn mds(input: Bytes) -> Res<Mds> {
    let (rest, header) = header(input)?;
    let first_session_offset: usize = header.session_offset.try_into().unwrap();
    let num_sessions = header.num_sessions.into();

    let mut sessions = Vec::with_capacity(num_sessions);

    for i in 0..num_sessions {
        let session_offset = first_session_offset + SESSION_SIZE * i;
        let session = session(input, session_offset)?.1;
        sessions.push(session);
    }

    let rest_offset = num_sessions * SESSION_SIZE;
    let rest = &rest[rest_offset..];

    Ok((rest, Mds { header, sessions }))
}
