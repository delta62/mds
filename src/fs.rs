use crate::{
    error::{Error, Result},
    mds::Track,
};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
};

pub fn writer_with_extension<P: AsRef<Path>>(
    relative_to: P,
    extension: &str,
) -> Result<BufWriter<File>> {
    let mut out_path = relative_to.as_ref().to_path_buf();
    out_path.set_extension(extension);

    let out_file = File::create(out_path).map_err(Error::Io)?;
    Ok(BufWriter::new(out_file))
}

pub fn reader_for_track<P: AsRef<Path>>(mds_path: P, track: &Track) -> Result<BufReader<File>> {
    let path = track
        .data_filename(mds_path)
        .ok_or(Error::MissingInputFile)?;
    let file = File::open(path).map_err(Error::Io)?;
    Ok(BufReader::new(file))
}
