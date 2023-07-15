use nom::{
    bytes::complete::{tag, take},
    combinator::{fail, map, map_res},
    error::dbg_dmp,
    multi::count,
    number::complete::{le_i32, le_u16, le_u32, le_u8},
    sequence::{preceded, tuple},
    IResult,
};

type Bytes<'a> = &'a [u8];
type Res<'a, T> = IResult<Bytes<'a>, T>;
type Version = [u8; 3];
type Version16 = [u16; 3];

#[derive(Debug)]
pub enum PregapCorrection {
    TrackAtOnce,
    DiskAtOnce,
    Other(i32),
}

impl Into<PregapCorrection> for i32 {
    fn into(self) -> PregapCorrection {
        use PregapCorrection::*;

        match self {
            0 => TrackAtOnce,
            -150 => DiskAtOnce,
            x => Other(x),
        }
    }
}

#[derive(Debug)]
pub struct Header {
    version1: Version,
    version2: Version16,
    pregap_correction: PregapCorrection,
    num_sectors: u32,
    num_entries: u8,
    num_leadin: u8,
    num_sessions: u8,
    num_tracks: u8,
    ofs_entries: u32,
}

fn id(input: Bytes) -> Res<Bytes> {
    tag("MEDIA DESCRIPTOR")(input)
}

fn version1(input: Bytes) -> Res<[u8; 3]> {
    preceded(id, map_res(count(le_u8, 3), |x| x.try_into()))(input)
}

fn version2(input: Bytes) -> Res<[u16; 3]> {
    preceded(le_u8, map_res(count(le_u16, 3), |x| x.try_into()))(input)
}

fn pregap_correction(input: Bytes) -> Res<PregapCorrection> {
    preceded(take(48usize), map(le_i32, |x| x.into()))(input)
}

fn num_sectors(input: Bytes) -> Res<u32> {
    println!("{:X?}", &input[..4]);
    dbg_dmp(le_u32, "num_sectors")(input)
}

fn num_entries(input: Bytes) -> Res<u8> {
    preceded(le_u16, le_u8)(input)
}

fn num_leadin(input: Bytes) -> Res<u8> {
    le_u8(input)
}

fn num_sessions(input: Bytes) -> Res<u8> {
    le_u8(input)
}

fn num_tracks(input: Bytes) -> Res<u8> {
    preceded(le_u8, le_u8)(input)
}

fn ofs_entries(input: Bytes) -> Res<u32> {
    preceded(take(5usize), le_u32)(input)
}

pub fn header(input: Bytes) -> Res<Header> {
    let (
        input,
        (
            version1,
            version2,
            pregap_correction,
            num_sectors,
            num_entries,
            num_leadin,
            num_sessions,
            num_tracks,
            ofs_entries,
        ),
    ) = tuple((
        version1,
        version2,
        pregap_correction,
        num_sectors,
        num_entries,
        num_leadin,
        num_sessions,
        num_tracks,
        ofs_entries,
    ))(input)?;
    let header = Header {
        version1,
        version2,
        pregap_correction,
        num_sectors,
        num_entries,
        num_leadin,
        num_sessions,
        num_tracks,
        ofs_entries,
    };

    Ok((input, header))
}
