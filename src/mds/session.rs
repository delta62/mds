use super::track::{track, Track};
use super::types::{Bytes, Res};
use crate::timecode::{Timecode, FRAMES_PER_SECOND};
use nom::{
    bytes::complete::take,
    number::complete::{le_i32, le_u16, le_u32, le_u8},
    sequence::tuple,
};

/// The number of bytes in an .mds file that a session takes
pub const SESSION_SIZE: usize = 0x18;
/// The number of bytes in an .mds file that a track takes
const TRACK_SIZE: usize = 0x50;

#[derive(Debug)]
pub struct Session {
    /// The location in sectors of the first sector stored in the disc image. This can be negative,
    /// because sector 0 is the first block of user data. An image with pregap data would typically
    /// start at sector -150 (0xFFFFFF6A).
    pub start_sector: i32,

    /// The location of the first track in timecode format. For discs with a pregap, this will
    /// typically be 00:02:00. Otherwise, it would be 00:00:00.
    pub start_time: Timecode,
    pub end_sector: i32,
    _session_number: u16,
    _num_data_blocks: u8,
    _num_lead_in_data_blocks: u8,
    _first_track_num: u16,
    _last_track_num: u16,
    _first_track_offset: u32,
    tracks: Vec<Track>,
}

impl Session {
    /// An iterator over all the data tracks in this disc image. Pregap / postgap tracks are
    /// omitted; only user data tracks are included
    pub fn data_tracks(&self) -> impl Iterator<Item = &Track> {
        self.tracks.iter().filter(|track| track.number() < 0xA0)
    }
}

pub fn session(input: Bytes, session_offset: usize) -> Res<Session> {
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
            first_track_offset,
        ),
    ) = tuple((
        take(session_offset),
        le_i32,
        le_i32,
        le_u16,
        le_u8,
        le_u8,
        le_u16,
        le_u16,
        le_u32,
        le_u32,
    ))(input)?;

    let data_blocks_offset: usize = first_track_offset.try_into().unwrap();
    let mut tracks = Vec::new();

    for i in 0..num_data_blocks {
        let block_offset = data_blocks_offset + TRACK_SIZE * i as usize;
        let (_, track) = track(input, block_offset)?;
        tracks.push(track);
    }

    let f = start_sector % FRAMES_PER_SECOND;
    let s = start_sector / FRAMES_PER_SECOND - f;
    let start_time = Timecode::from_msf(0, s, f);

    let session = Session {
        start_sector,
        start_time,
        end_sector,
        _session_number: session_number,
        _num_data_blocks: num_data_blocks,
        _num_lead_in_data_blocks: num_lead_in_data_blocks,
        _first_track_num: first_track_num,
        _last_track_num: last_track_num,
        _first_track_offset: first_track_offset,
        tracks,
    };

    let rest_offset = TRACK_SIZE * num_data_blocks as usize;
    let rest = &rest[rest_offset..];

    Ok((rest, session))
}
