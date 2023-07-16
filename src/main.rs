mod mds;

use mds::{mds, Mds};
use std::{
    fs::{read, File},
    io::{BufReader, BufWriter, Read, Result, Write},
    path::Path,
};

fn main() {
    let mut args = std::env::args();
    let prog_name = args.next().unwrap();
    let file = args.next().expect(&format!("Usage: {prog_name} <file>"));

    if !file.ends_with(".mds") {
        eprintln!("Must be given an .mds file to read");
        std::process::exit(1);
    }

    let outfile = file.replace(".mds", ".iso");
    let mdffile = file.replace(".mds", ".mdf");

    let bytes = read(file).unwrap();
    let mds = mds(&bytes).unwrap().1;
    mds_to_iso(&mds, &mdffile, &outfile).unwrap();
}

fn mds_to_iso<P: AsRef<Path>>(mds: &Mds, mdfpath: P, outpath: P) -> Result<()> {
    let mut data_tracks = mds.sessions().flat_map(|session| session.data_tracks());

    let track = data_tracks
        .next()
        .expect("There are no data tracks in this MDS");

    assert!(
        data_tracks.next().is_none(),
        "Multi track conversion not yet supported"
    );

    let sector_size = track.sector_size();
    let data_size = track.sector_data_size();
    let num_sectors = track.num_sectors();

    let file = File::create(outpath)?;
    let mut writer = BufWriter::new(file);

    let file = File::open(mdfpath)?;
    let mut reader = BufReader::new(file);

    let mut buf = Vec::with_capacity(sector_size);

    for _ in 0..num_sectors {
        reader.read_exact(&mut buf)?;
        writer.write_all(&buf[0..data_size])?;
    }

    Ok(())
}
