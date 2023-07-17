use super::track::{track, Track};
use super::types::{Bytes, Res};
use nom::{
    bytes::complete::take,
    number::complete::{le_i32, le_u16, le_u32, le_u8},
    sequence::tuple,
};

const DATA_BLOCK_SIZE: usize = 0x50;

#[derive(Debug)]
pub struct Session {
    start_sector: i32,
    end_sector: i32,
    session_number: u16,
    num_data_blocks: u8,
    num_lead_in_data_blocks: u8,
    first_track_num: u16,
    last_track_num: u16,
    first_track_offset: u32,
    tracks: Vec<Track>,
}

impl Session {
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
        let block_offset = data_blocks_offset + DATA_BLOCK_SIZE * i as usize;
        let (_, track) = track(input, block_offset)?;
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
        first_track_offset,
        tracks,
    };

    let rest_offset = DATA_BLOCK_SIZE * num_data_blocks as usize;
    let rest = &rest[rest_offset..];

    Ok((rest, session))
}
