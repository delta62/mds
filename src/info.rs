use crate::{
    args::Args,
    error::{Error, Result},
    mds::Mds,
};
use std::fs::read;

pub fn info(args: &Args) -> Result<()> {
    let bytes = read(&args.mds_file).map_err(Error::Io)?;
    let mds = Mds::from_bytes(&bytes)?;

    let file_size = bytes.len();
    let num_sessions = mds.sessions().count();
    let num_tracks = mds.sessions().flat_map(|sess| sess.data_tracks()).count();

    println!("{}", args.mds_file.as_path().to_str().unwrap_or_default());
    println!("{file_size} bytes, {num_sessions} sessions, {num_tracks} tracks");

    for (i, session) in mds.sessions().enumerate() {
        println!("Session {}", i + 1);

        for (i, track) in session.data_tracks().enumerate() {
            println!("  Track {}", i + 1);
            println!("    Mode:         {:?}", track.mode);
            println!("    Subchannels:  {:?}", track.num_subchannels);
            println!("    Sectors:      {}", track.num_sectors());
            println!("    Sector size:  {}", track.sector_size());
        }
    }

    Ok(())
}
