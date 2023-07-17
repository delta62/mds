use super::{
    index::{index_block, IndexBlock},
    types::{Bytes, Res},
};
use nom::{
    bytes::complete::take,
    combinator::map_res,
    number::complete::{le_i32, le_u16, le_u32, le_u64, le_u8},
    sequence::tuple,
};

#[derive(Debug)]
pub struct Track {
    track_mode: TrackMode,
    num_subchannels: SubChannels,
    adr: u8,
    track_number: u8,
    point: u8,
    minute: u8,
    second: u8,
    frame: u8,
    index: Option<IndexBlock>,
    sector_size: u16,
    track_start_sector: i32,
    track_start_offset: u64,
    num_filenames: u32,
    filename_offset: u32,
}

#[derive(Debug)]
pub enum TrackMode {
    None,
    Audio,
    Mode1,
    Mode2,
    Mode2Form1,
    Mode2Form2,
}

pub struct UnknownTrackMode(pub u8);

impl TryInto<TrackMode> for u8 {
    type Error = UnknownTrackMode;

    fn try_into(self) -> Result<TrackMode, Self::Error> {
        use TrackMode::*;

        match self {
            0x00 => Ok(None),
            0xA9 => Ok(Audio),
            0xAA => Ok(Mode1),
            0xAB => Ok(Mode2),
            0xAC => Ok(Mode2Form1),
            0xAD => Ok(Mode2Form2),
            0xEC => Ok(Mode2),
            x => Err(UnknownTrackMode(x)),
        }
    }
}

#[derive(Debug)]
pub enum SubChannels {
    None,
    Eight,
}

pub struct UnknonwSubChannelFlag(pub u8);

impl TryInto<SubChannels> for u8 {
    type Error = UnknonwSubChannelFlag;

    fn try_into(self) -> Result<SubChannels, Self::Error> {
        match self {
            0x00 => Ok(SubChannels::None),
            0x08 => Ok(SubChannels::Eight),
            x => Err(UnknonwSubChannelFlag(x)),
        }
    }
}

impl Track {
    pub fn number(&self) -> usize {
        self.point.into()
    }

    pub fn sector_size(&self) -> usize {
        self.sector_size.into()
    }

    pub fn sector_data_size(&self) -> usize {
        self.sector_size() - self.sector_subchannel_size()
    }

    pub fn sector_subchannel_size(&self) -> usize {
        match self.num_subchannels {
            SubChannels::None => 0x00,
            SubChannels::Eight => 0x60, // 92 bytes at the end of each sector are devoted to subchannel data
        }
    }

    pub fn num_sectors(&self) -> usize {
        self.index
            .as_ref()
            .map(|idx| idx.index1_sectors as usize)
            .unwrap_or_default()
    }
}

pub fn track(input: Bytes, track_offset: usize) -> Res<Track> {
    let track_input = &input[track_offset..];
    let (
        rest,
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
            track_start_sector,
            track_start_offset,
            num_filenames,
            filename_offset,
            _,
        ),
    ) = tuple((
        track_mode,
        num_subchannels, // num subchannels
        le_u8,           // adr/control
        le_u8,           // track number
        le_u8,           // point
        take(4usize),    // zero
        le_u8,           // minute
        le_u8,           // second
        le_u8,           // frame
        le_u32,          // index block offset
        le_u16,          // sector size
        take(0x12usize), // unknown & zero
        le_i32,          // track start sector
        le_u64,          // track start offset
        le_u32,          // num filenames for this track
        le_u32,          // offset to filename block for this track
        take(0x18usize), // zero
    ))(track_input)?;

    let index = if index_block_offset > 0 {
        let offset = index_block_offset.try_into().unwrap();
        let block_input = &input[offset..];
        Some(index_block(block_input)?.1)
    } else {
        None
    };

    let track = Track {
        track_mode,
        num_subchannels,
        adr,
        track_number,
        point,
        minute,
        second,
        frame,
        index,
        sector_size,
        track_start_sector,
        track_start_offset,
        num_filenames,
        filename_offset,
    };

    Ok((rest, track))
}

fn track_mode(input: Bytes) -> Res<TrackMode> {
    map_res(le_u8, |x| x.try_into())(input)
}

fn num_subchannels(input: Bytes) -> Res<SubChannels> {
    map_res(le_u8, |x| x.try_into())(input)
}
