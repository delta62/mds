use crate::{
    error::{Error, Result},
    mds::Mds,
};
use std::{fs::read, path::Path};

/// Create a new Mds by reading the file at `path`
pub fn load_mds<P: AsRef<Path>>(path: P) -> Result<Mds> {
    let bytes = read(path).map_err(Error::Io)?;
    Mds::from_bytes(&bytes)
}
