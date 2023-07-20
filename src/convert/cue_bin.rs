use crate::{
    error::{Error, Result},
    loader::load_mds,
    mds::{Mds, Track, TrackMode},
    timecode::Timecode,
    util::{reader_for_track, set_extension, writer_with_extension},
};
use std::{
    io::{Read, Seek, SeekFrom, Write},
    path::Path,
};

// Information about the .cue file format can be found at
// https://psx-spx.consoledev.net/cdromdrive/#cuebin-cdrwin, or the original user manual for cdrwin
// can be found on archive.org:
// https://web.archive.org/web/20070614044112/http://www.goldenhawk.com/download/cdrwin.pdf. The
// various cuesheet commands are in appendix A.

pub fn convert<P: AsRef<Path>>(mds_file: P) -> Result<()> {
    let mds = load_mds(&mds_file)?;
    let bin_writer = writer_with_extension(&mds_file, "bin")?;
    let cue_writer = writer_with_extension(&mds_file, "cue")?;

    mds_to_cue(&mds, &mds_file, cue_writer)?;
    mds_to_bin(&mds, &mds_file, bin_writer)
}

/// Generate a .cue file with metadata contained in an .mds file and write it to `writer`
fn mds_to_cue<P, W>(mds: &Mds, mds_path: P, mut writer: W) -> Result<()>
where
    P: AsRef<Path>,
    W: Write,
{
    let bin_path = set_extension(mds_path, "bin");
    let filename = bin_path.file_name().unwrap().to_str().unwrap();
    writeln!(writer, "FILE \"{filename}\" BINARY").map_err(Error::Io)?;

    let session = mds.single_session()?;
    let pregap_correction = session.start_time;
    let tracks = session
        .data_tracks()
        .enumerate()
        .map(|(i, sess)| (i + 1, sess)); // CUE tracks are 1-indexed

    for (i, track) in tracks {
        let mode = cue_media_type(&track)?;
        let addr = cue_address(&track, pregap_correction);

        writeln!(writer, "  TRACK {i} {mode}").map_err(Error::Io)?; // TODO
        writeln!(writer, "    INDEX 01 {addr}").map_err(Error::Io)?;
    }

    Ok(())
}

/// Write all tracks of an .mdf to the given writer
fn mds_to_bin<P, W>(mds: &Mds, mds_path: P, mut writer: W) -> Result<()>
where
    P: AsRef<Path>,
    W: Write,
{
    let session = mds.single_session()?;
    let tracks = session.data_tracks();

    for track in tracks {
        let mut mdf_reader = reader_for_track(&mds_path, track)?;
        mdf_reader
            .seek(SeekFrom::Start(track.track_start_offset))
            .map_err(Error::Io)?;

        let sector_size = track.sector_size();
        let data_size = track.sector_data_size();
        let mut buf = vec![0; sector_size];

        for _ in 0..track.num_sectors() {
            mdf_reader.read_exact(&mut buf).map_err(Error::Io)?;
            writer.write_all(&buf[0..data_size]).map_err(Error::Io)?;
        }
    }

    Ok(())
}

/// Determine the track type that should be printed in a .cue file. There are a few well-known
/// ones, but not every possibility is accounted for. If there are additional possible combinations
/// which have been seen in the wild, add them here.
///
/// Note that the sector sizes in the .cue file (e.g. "Mode2/2352") are the sizes of the *data*
/// region only because they do not include subchannel or error correction bits.
fn cue_media_type(track: &Track) -> Result<&str> {
    use Error::UnknownCueTrackSize;
    use TrackMode::*;

    match (track.mode, track.sector_data_size()) {
        (Audio, 0x930) => Ok("AUDIO"),
        (Mode1, 0x800) => Ok("MODE1/2048"),
        (Mode1, 0x930) => Ok("MODE1/2352"),
        (Mode2, 0x920) => Ok("MODE2/2336"),
        (Mode2, 0x930) => Ok("MODE2/2352"),
        (mode, data_sector_size) => Err(UnknownCueTrackSize(mode, data_sector_size)),
    }
}

/// Format a track's timecode as it should appear in an INDEX directive.
/// Since .cue files do not include pregap information, .mds discs which do have pregap data (most
/// of them) need to offset their track timecodes, because the first INDEX must have a timecode of
/// 00:00:00. However, with a pregap an .mds file would typically start the first track at
/// 00:02:00.
fn cue_address(track: &Track, pregap_correction: Timecode) -> String {
    let timecode = Timecode::from_track(track) + pregap_correction;
    let (m, s, f) = timecode.msf();

    format!("{m:02}:{s:02}:{f:02}")
}
