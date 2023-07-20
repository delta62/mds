use crate::mds::Track;
use std::ops::{Add, AddAssign};

/// A "frame" in CD terms is 75 sectors. That is, if you played an audio CD at 75 frames per second
/// it would sound right. This is the definition of a 1x CD drive.
pub const FRAMES_PER_SECOND: i32 = 75;

/// Various software makes use of timecodes to measure positions within a CD rather than bytes. The
/// `Timecode` struct represents a location in a disc based on time. It can be easily formatted in
/// strings using the `msf` function to get the individual components.
#[derive(Clone, Copy, Debug, Default)]
pub struct Timecode(i32);

impl Timecode {
    /// Create a new timecode with minutes, seconds, and frames set to where a given track starts
    pub fn from_track(track: &Track) -> Self {
        Self::from_msf(track.minute, track.second, track.frame)
    }

    /// Create a new timecode with the given values for minutes, seconds, and frames
    pub fn from_msf<M, S, F>(minutes: M, seconds: S, frames: F) -> Self
    where
        M: Into<i32>,
        S: Into<i32>,
        F: Into<i32>,
    {
        let frames = minutes.into() * 60 * FRAMES_PER_SECOND
            + seconds.into() * FRAMES_PER_SECOND
            + frames.into();
        Self(frames)
    }

    /// Extract the minutes, seconds, and frame components of this timecode
    pub fn msf(&self) -> (i32, i32, i32) {
        let f = self.0 % FRAMES_PER_SECOND;
        let s = (self.0 - f) / FRAMES_PER_SECOND % 60;
        let m = (self.0 - s - f) / 60 / FRAMES_PER_SECOND;

        (m, s, f)
    }
}

impl Add for Timecode {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Timecode(self.0 + rhs.0)
    }
}

impl AddAssign for Timecode {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}
