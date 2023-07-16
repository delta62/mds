use nom::IResult;

pub type Bytes<'a> = &'a [u8];
pub type Res<'a, T> = IResult<Bytes<'a>, T>;
