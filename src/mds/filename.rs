use super::types::{Bytes, Res};
use nom::{
    combinator::map_res,
    number::complete::{le_u32, le_u8},
    sequence::tuple,
};

#[derive(Debug)]
pub enum NameFormat {
    EightBit,
    SixteenBit,
}

pub struct NameFormatError;

impl TryInto<NameFormat> for u8 {
    type Error = NameFormatError;

    fn try_into(self) -> Result<NameFormat, Self::Error> {
        match self {
            0 => Ok(NameFormat::EightBit),
            1 => Ok(NameFormat::SixteenBit),
            _ => Err(NameFormatError),
        }
    }
}

#[derive(Debug)]
pub struct FilenameBlock {
    pub filename_offset: u32,
    pub filename_format: NameFormat,
}

pub fn filename_block(input: Bytes) -> Res<FilenameBlock> {
    let (rest, (filename_offset, filename_format)) = tuple((le_u32, name_format))(input)?;

    Ok((
        rest,
        FilenameBlock {
            filename_offset,
            filename_format,
        },
    ))
}

fn name_format(input: Bytes) -> Res<NameFormat> {
    map_res(le_u8, |x| x.try_into())(input)
}
