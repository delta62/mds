# mds

Utilities for reading and converting .mds/.mdf disk image files

This tool converts .mdf/.mds files into .iso or .cue/.bin files. I wrote this
since I found that `mdf2iso` was creating bad images for some discs that I
tried, and the iso file format cannot handle multi-track images at all.

This program reads from `.mds` files, which are binary metadata files that
describe the contents of their accompanying `.mdf` files. This is in contrast to
`mdf2iso`, which attempts to parse the type of disc image out of the mdf data
file itself. That said, you will need the .mds metadata file to use this
program.

## Usage

### Printing mds metadata

Run `mds info <my_file.mds>` to view the contents of an mdf image

```
# mds info my_file.mds
/home/sam/my_file.mds
MDS v1.3 | CD-ROM, 574 bytes, 1 session, 2 tracks
Session 1
  First sector:   -150      (0xFFFFFF6A)
  Last sector:    294066    (0x47CB2)
  Total sectors:  294216    (0x47D48)
  Track 1
    Mode:         Mode2
    Subchannels:  Eight
    Data file:    /home/sam/my_file.mdf
    Time offset:  00:02.000
    First byte:   0         (0x0)
    First sector: 0         (0x0)
    Sectors:      278166    (0x43E96)
    Sector size:  2448      (0x990)
    Approx Size:  680MB
  Track 2
    Mode:         Audio
    Subchannels:  Eight
    Data file:    /home/sam/my_file.mdf
    Time offset:  61:52.916
    First byte:   680950368 (0x28967A60)
    First sector: 278316    (0x43F2C)
    Sectors:      15750     (0x3D86)
    Sector size:  2448      (0x990)
    Approx Size:  38MB
```

### Converting to iso

Run `mds convert --format iso <my_image.mds>` to convert the contents of an mdf to an iso
file. Note that iso files can only contain one track, so if you have a
multi-track mdf you'll need to convert to a different format.

### Converting to bin/cue

Run `mds convert --format cue <my_image.mds>` to convert the contents of an mdf to bin
and cue files. This format does support multiple tracks.
