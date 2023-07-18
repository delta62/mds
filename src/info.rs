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
        let first_sector = session.start_sector;
        let last_sector = session.end_sector;
        let total_sectors = last_sector - first_sector;

        println!("Session {}", i + 1);
        println!("  First sector:    {first_sector:<9} (0x{first_sector:X})");
        println!("  Last sector:     {last_sector:<9} (0x{last_sector:X})");
        println!("  Total sectors:   {total_sectors:<9} (0x{total_sectors:X})");

        for (i, track) in session.data_tracks().enumerate() {
            let filename = track
                .data_filename(&args.mds_file)
                .unwrap_or("--none--".to_owned());

            let num_sectors = track.num_sectors();
            let sector_size = track.sector_size();
            let track_offset = track.track_start_offset;
            let sector_offset = track.track_start_sector;
            let timecode = track.time_str();

            println!("  Track {}", i + 1);
            println!("    Mode:          {:?}", track.mode);
            println!("    Subchannels:   {:?}", track.num_subchannels);
            println!("    Data file:     {filename}");
            println!("    Sectors:       {num_sectors:<9} (0x{num_sectors:X?})");
            println!("    Sector size:   {sector_size:<9} (0x{sector_size:X?})");
            println!("    Time offset:   {timecode}");
            println!("    Track Offset:  {track_offset:<9} (0x{track_offset:X?})");
            println!("    Sector Offset: {sector_offset:<9} (0x{sector_offset:X?})");
        }
    }

    Ok(())
}
