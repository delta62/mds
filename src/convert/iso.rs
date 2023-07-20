use crate::{
    error::{Error, Result},
    loader::load_mds,
    mds::Track,
    util::{reader_for_track, writer_with_extension},
};
use std::{
    io::{Read, Write},
    path::Path,
};

/// Convert a .mdf file (described by a .mds file) into a .iso file. This is not always possible,
/// because an ISO represents the bytes of one track. If the .mds refers to a multi-track disc,
/// writing to an ISO doesn't make sense. Try using BIN/CUE in that case.
pub fn convert<P: AsRef<Path>>(mds_file: P) -> Result<()> {
    let mds = load_mds(&mds_file)?;
    let track = mds.single_track()?;
    let writer = writer_with_extension(&mds_file, "iso")?;

    track_to_iso(&track, &mds_file, writer)
}

fn track_to_iso<P: AsRef<Path>, W: Write>(track: &Track, mds_path: P, mut writer: W) -> Result<()> {
    let sector_size = track.sector_size();
    let data_size = track.sector_data_size();
    let num_sectors = track.num_sectors();
    let mut reader = reader_for_track(&mds_path, track)?;

    let mut buf = vec![0; sector_size];
    for _ in 0..num_sectors {
        reader.read_exact(&mut buf).map_err(Error::Io)?;

        // In order to convert the .mdf, take only the main track's data from each sector. Each
        // sector may also contain subchannel data which is stored at the end of the sector. ISO
        // files don't store subchannel data, so just discard this.
        writer.write_all(&buf[0..data_size]).map_err(Error::Io)?;
    }

    Ok(())
}
