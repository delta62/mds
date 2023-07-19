use crate::{
    error::{Error, Result},
    mds::{Mds, Track},
};
use std::{
    fs::{read, File},
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
};

pub fn convert<P: AsRef<Path>>(mds_file: P) -> Result<()> {
    let bytes = read(&mds_file).map_err(Error::Io)?;
    let mds = Mds::from_bytes(&bytes)?;
    let writer = make_writer(&mds_file)?;

    mds_to_iso(&mds, &mds_file, writer)
}

/// Given the path to a .mds file, create a buffered writer for the .iso file to write data to
fn make_writer<P: AsRef<Path>>(mds_path: P) -> Result<BufWriter<File>>
where
    P: AsRef<Path>,
{
    let mut out_path = mds_path.as_ref().to_path_buf();
    out_path.set_extension("iso");

    let out_file = File::create(out_path).map_err(Error::Io)?;
    Ok(BufWriter::new(out_file))
}

fn make_reader<P: AsRef<Path>>(track: &Track, mds_file_path: P) -> Result<BufReader<File>> {
    let mdf_path = track
        .data_filename(mds_file_path)
        .ok_or(Error::MissingInputFile)?;
    let file = File::open(mdf_path).map_err(Error::Io)?;

    Ok(BufReader::new(file))
}

/// Convert a .mdf file (described by a .mds file) into a .iso file
fn mds_to_iso<P: AsRef<Path>, W: Write>(mds: &Mds, mds_path: P, mut writer: W) -> Result<()> {
    let mut data_tracks = mds.sessions().flat_map(|session| session.data_tracks());
    let track = data_tracks.next().ok_or(Error::NoDataTracks)?;

    if data_tracks.next().is_some() {
        Err(Error::MultiTrackNotSupported)?;
    }

    let sector_size = track.sector_size();
    let data_size = track.sector_data_size();
    let num_sectors = track.num_sectors();
    let mut reader = make_reader(&track, mds_path)?;

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
