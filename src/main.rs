use byteorder::{LittleEndian, ReadBytesExt};
use std::{
    fs::read,
    io::{Cursor, Seek, SeekFrom},
};

const MDS_SIGNATURE: &[u8; 16] = b"MEDIA DESCRIPTOR";

fn main() {
    let mut args = std::env::args();
    let prog_name = args.next().unwrap();
    let file = args.next().expect(&format!("Usage: {prog_name} <file>"));

    if !file.ends_with(".mds") {
        eprintln!("Must be given an .mds file to read");
        std::process::exit(1);
    }

    let bytes = read(file).unwrap();

    if bytes.len() < 0x54 {
        panic!("invalid mds");
    }

    if !memcmp(&bytes, MDS_SIGNATURE) {
        panic!("invalid signature");
    }

    let mut cur = Cursor::new(&bytes);
    cur.seek(SeekFrom::Start(0x50)).unwrap();

    let session_offset = cur.read_i32::<LittleEndian>().unwrap();

    cur.seek(SeekFrom::Start(session_offset as u64 + 14))
        .unwrap();
    let track_count = cur.read_i16::<LittleEndian>().unwrap();
    cur.seek(SeekFrom::Start(session_offset as u64 + 20))
        .unwrap();
    let track_offset = cur.read_u32::<LittleEndian>().unwrap();

    if track_count > 99 || track_offset > bytes.len().try_into().unwrap() {
        panic!("invalid track count or block offset");
    }

    println!("MDS size: {}", bytes.len());
    println!("track count: {track_count}");
    println!("session offset: {track_offset}");
}

fn memcmp(actual: &[u8], expected: &[u8]) -> bool {
    for i in 0..expected.len() {
        if actual[i] != expected[i] {
            return false;
        }
    }

    true
}
