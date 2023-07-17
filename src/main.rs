mod error;
mod mds;

use error::{Error, Result};
use mds::Mds;
use std::{
    env::Args,
    fs::{read, File},
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
};

fn main() {
    let args = std::env::args();
    let result = try_run(args);

    if let Err(err) = result {
        eprintln!("{err}");
    }
}

fn try_run(args: Args) -> Result<()> {
    let mds_path = mds_path_from_args(args)?;
    let bytes = read(&mds_path).map_err(Error::Io)?;
    let mds = Mds::from_bytes(&bytes)?;
    let (reader, writer) = make_read_write(&mds_path)?;

    mds_to_iso(&mds, reader, writer)
}

/// Attempt to extract the path to an .mds file from the given arguments
fn mds_path_from_args(mut args: Args) -> Result<PathBuf> {
    let prog_name = args.next().unwrap();
    let path = args.next().ok_or_else(|| Error::NoMdsGiven { prog_name })?;
    let path = PathBuf::from(path);

    let ext = path
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();

    if ext != "mds" {
        Err(Error::WrongFileTypeGiven)?;
    }

    Ok(path)
}

/// Given the path to a .mds file, create a buffered reader & writer for the .mdf file to read data
/// from and the .iso file to write data to, respectively
fn make_read_write<P: AsRef<Path>>(mds_path: P) -> Result<(BufReader<File>, BufWriter<File>)> {
    let mds_path = mds_path.as_ref();

    let mut mdf_path = mds_path.to_path_buf();
    mdf_path.set_extension("mdf");
    let mdf_file = File::open(mdf_path).map_err(Error::Io)?;
    let reader = BufReader::new(mdf_file);

    let mut out_path = mds_path.to_path_buf();
    out_path.set_extension("iso");
    let out_file = File::create(out_path).map_err(Error::Io)?;
    let writer = BufWriter::new(out_file);

    Ok((reader, writer))
}

/// Convert a .mdf file (described by a .mds file) into a .iso file
fn mds_to_iso<R, W>(mds: &Mds, mut reader: R, mut writer: W) -> Result<()>
where
    R: Read,
    W: Write,
{
    let mut data_tracks = mds.sessions().flat_map(|session| session.data_tracks());
    let track = data_tracks.next().ok_or(Error::NoDataTracks)?;

    if data_tracks.next().is_some() {
        Err(Error::MultiTrackNotSupported)?;
    }

    let sector_size = track.sector_size();
    let data_size = track.sector_data_size();
    let num_sectors = track.num_sectors();

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
