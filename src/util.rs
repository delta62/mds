use crate::{
    error::{Error, Result},
    mds::Track,
};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};

/// Modify the given path by changing the file extension to `extension`
pub fn set_extension<P: AsRef<Path>>(path: P, extension: &str) -> PathBuf {
    let mut path = path.as_ref().to_path_buf();
    path.set_extension(extension);

    path
}

/// Create a new `BufWriter` which will write to the file `relative_to`, but with the extension
/// given by `extension`
pub fn writer_with_extension<P: AsRef<Path>>(
    relative_to: P,
    extension: &str,
) -> Result<BufWriter<File>> {
    let out_path = set_extension(relative_to, extension);
    let out_file = File::create(out_path).map_err(Error::Io)?;

    Ok(BufWriter::new(out_file))
}

/// Create a new `BufReader` which will read track data from an .mdf
pub fn reader_for_track<P: AsRef<Path>>(mds_path: P, track: &Track) -> Result<BufReader<File>> {
    let path = track
        .data_filename(mds_path)
        .ok_or(Error::MissingInputFile)?;
    let file = File::open(path).map_err(Error::Io)?;

    Ok(BufReader::new(file))
}

/// Append an "s" when count != 1
pub fn pluralize(s: &str, count: usize) -> String {
    if count == 1 {
        s.to_owned()
    } else {
        format!("{s}s")
    }
}
