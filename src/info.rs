use crate::{error::Result, loader::load_mds, util::pluralize};
use std::path::Path;

pub fn info<P: AsRef<Path>>(mds_file: P) -> Result<()> {
    let mds = load_mds(&mds_file)?;

    let file_size = mds.byte_len();
    let num_sessions = mds.sessions().count();
    let num_tracks = mds.sessions().map(|sess| sess.data_tracks().count()).sum();
    let version = mds.version();
    let media_type = mds.media_type();

    println!("{}", mds_file.as_ref().to_str().unwrap_or("--none--"));
    println!(
        "MDS v{version} | {media_type}, {file_size} {}, {num_sessions} {}, {num_tracks} {}",
        pluralize("byte", file_size),
        pluralize("session", num_sessions),
        pluralize("track", num_tracks),
    );

    for (i, session) in mds.sessions().enumerate() {
        let first_sector = session.start_sector;
        let last_sector = session.end_sector;
        let total_sectors = last_sector - first_sector;

        println!("Session {}", i + 1);
        println!("  First sector:   {first_sector:<9} (0x{first_sector:X})");
        println!("  Last sector:    {last_sector:<9} (0x{last_sector:X})");
        println!("  Total sectors:  {total_sectors:<9} (0x{total_sectors:X})");

        for (i, track) in session.data_tracks().enumerate() {
            let filename = track
                .data_filename(&mds_file)
                .unwrap_or("--none--".to_owned());

            let num_sectors = track.num_sectors();
            let sector_size = track.sector_size();
            let track_offset = track.track_start_offset;
            let sector_offset = track.track_start_sector;
            let timecode = track.time_str();
            let total_size = sector_size * num_sectors / 1_000_000;

            println!("  Track {}", i + 1);
            println!("    Mode:         {:?}", track.mode);
            println!("    Subchannels:  {:?}", track.num_subchannels);
            println!("    Data file:    {filename}");
            println!("    Time offset:  {timecode}");
            println!("    First byte:   {track_offset:<9} (0x{track_offset:X?})");
            println!("    First sector: {sector_offset:<9} (0x{sector_offset:X?})");
            println!("    Sectors:      {num_sectors:<9} (0x{num_sectors:X?})");
            println!("    Sector size:  {sector_size:<9} (0x{sector_size:X?})");
            println!("    Approx Size:  {total_size}MB");
        }
    }

    Ok(())
}
