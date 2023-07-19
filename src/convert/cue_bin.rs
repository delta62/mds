use crate::{
    error::{Error, Result},
    mds::{Mds, Track, TrackMode},
};
use std::{
    fs::{read, File},
    io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    path::Path,
};

pub fn convert<P: AsRef<Path>>(mds_file: P) -> Result<()> {
    let bytes = read(mds_file.as_ref()).map_err(Error::Io)?;
    let mds = Mds::from_bytes(&bytes)?;

    let cue_writer = make_file_writer(mds_file.as_ref(), "cue")?;
    mds_to_cue(&mds, mds_file.as_ref(), cue_writer)?;

    let bin_writer = make_file_writer(mds_file.as_ref(), "bin")?;
    mds_to_bin(&mds, mds_file.as_ref(), bin_writer)
}

fn make_file_reader<P: AsRef<Path>>(track: &Track, mds_path: P) -> Result<BufReader<File>> {
    let path = track
        .data_filename(mds_path)
        .ok_or(Error::MissingInputFile)?;
    let file = File::open(path).map_err(Error::Io)?;
    Ok(BufReader::new(file))
}

/// Given the path to a .mds file, create a buffered writer for the .iso file to write data to
fn make_file_writer<P: AsRef<Path>>(mds_path: P, extension: &str) -> Result<BufWriter<File>>
where
    P: AsRef<Path>,
{
    let mut out_path = mds_path.as_ref().to_path_buf();
    out_path.set_extension(extension);

    let out_file = File::create(out_path).map_err(Error::Io)?;
    Ok(BufWriter::new(out_file))
}

fn mds_to_cue<P: AsRef<Path>, W: Write>(mds: &Mds, mds_path: P, mut writer: W) -> Result<()> {
    let mut sessions = mds.sessions();
    let session = sessions.next().ok_or(Error::NoSessions)?;

    if sessions.next().is_some() {
        Err(Error::TooManySessions)?;
    }

    let mut filename = mds_path.as_ref().to_path_buf();
    filename.set_extension("bin");
    let filename = filename.file_name().unwrap().to_str().unwrap();
    writeln!(writer, "FILE \"{filename}\" BINARY").map_err(Error::Io)?;

    // CUE tracks are 1-indexed
    let tracks = session
        .data_tracks()
        .enumerate()
        .map(|(i, sess)| (i + 1, sess));

    let pre_f = session.start_sector % 75;
    let pre_s = session.start_sector / 75 - pre_f;
    let pregap_correction = Timecode::from_msf(0, pre_s, pre_f);

    for (i, track) in tracks {
        let mode = cue_media_type(&track);
        let addr = cue_address(&track, pregap_correction);

        writeln!(writer, "  TRACK {i} {mode}").map_err(Error::Io)?; // TODO
        writeln!(writer, "    INDEX 01 {addr}").map_err(Error::Io)?;
    }

    Ok(())
}

fn mds_to_bin<P, W>(mds: &Mds, mds_path: P, mut writer: W) -> Result<()>
where
    P: AsRef<Path>,
    W: Write,
{
    let session = mds.sessions().next().unwrap();
    let tracks = session.data_tracks();

    for track in tracks {
        let mut mdf_reader = make_file_reader(track, &mds_path)?;
        let track_offset = track.track_start_offset;
        mdf_reader
            .seek(SeekFrom::Start(track_offset))
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

fn cue_media_type(track: &Track) -> &str {
    match track.mode {
        TrackMode::Audio => "AUDIO",
        TrackMode::Mode2 => "MODE2/2352",
        _ => todo!(),
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Timecode(i32);

impl Timecode {
    pub fn from_track(track: &Track) -> Self {
        Self::from_msf(track.minute, track.second, track.frame)
    }

    pub fn from_msf<M, S, F>(minutes: M, seconds: S, frames: F) -> Self
    where
        M: Into<i32>,
        S: Into<i32>,
        F: Into<i32>,
    {
        let frames = minutes.into() * 60 * 75 + seconds.into() * 75 + frames.into();
        Self(frames)
    }

    pub fn add(&mut self, time: Timecode) {
        self.0 += time.0;
    }

    pub fn msf(&self) -> (i32, i32, i32) {
        let f = self.0 % 75;
        let s = (self.0 - f) / 75 % 60;
        let m = (self.0 - s - f) / 60 / 75;

        (m, s, f)
    }
}

fn cue_address(track: &Track, pregap_correction: Timecode) -> String {
    let mut timecode = Timecode::from_track(track);
    timecode.add(pregap_correction);
    let (m, s, f) = timecode.msf();

    format!("{m:02}:{s:02}:{f:02}")
}
