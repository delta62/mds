use nom::{number::complete::le_u32, sequence::tuple};

use super::types::{Bytes, Res};

#[derive(Debug)]
pub struct IndexBlock {
    index0_sectors: u32,
    pub index1_sectors: u32,
}

pub fn index_block(input: Bytes) -> Res<IndexBlock> {
    let (input, (index0_sectors, index1_sectors)) = tuple((le_u32, le_u32))(input)?;

    Ok((
        input,
        IndexBlock {
            index0_sectors,
            index1_sectors,
        },
    ))
}
