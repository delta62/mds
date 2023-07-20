mod filename;
mod header;
mod index;
mod mds;
mod session;
mod track;
mod types;

// Code for parsing .mds files
// These files are in a proprietary format. The best source of information I've been able to find
// out there is at https://psx-spx.consoledev.net/cdromdrive/#cdrom-disk-images-mdsmdf-alcohol-120.
// Beware some sites which have completely incorrect information listed!

pub use header::{header, Header, Version};
pub use mds::Mds;
pub use session::{session, Session};
pub use track::{Track, TrackMode};
