mod mds;

use byteorder::{LittleEndian, ReadBytesExt};
use mds::mds;
use std::{
    fs::read,
    io::{Cursor, Seek, SeekFrom},
};

fn main() {
    let mut args = std::env::args();
    let prog_name = args.next().unwrap();
    let file = args.next().expect(&format!("Usage: {prog_name} <file>"));

    if !file.ends_with(".mds") {
        eprintln!("Must be given an .mds file to read");
        std::process::exit(1);
    }

    let bytes = read(file).unwrap();
    let mds = mds(&bytes).unwrap().1;

    let mut cursor = Cursor::new(&bytes);
    cursor.seek(SeekFrom::Start(0x5C)).unwrap();
    let num_sectors = cursor.read_u32::<LittleEndian>().unwrap();

    let num_entries = bytes[0x62];
    let num_leadin = bytes[0x63];
    let num_sessions = bytes[0x64];
    let num_tracks = bytes[0x66];

    println!("sectors: {num_sectors}, entries: {num_entries}, leadin: {num_leadin}, sessions: {num_sessions}, tracks: {num_tracks}");

    println!("{:#X?}", mds);
}
