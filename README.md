# pvkr

A tool for splitting large files into smaller ones to be transferred with poor network connections

## Installation

```bash
git clone https://github.com/jasonverbeek/pvkr.git
cd pvkr
cargo install --path .
```
## Usage


To split file "large_file.mp4" into 25Mib chunks and save them in the "output_dir" directory:
```bash
pvkr split -f large_file.mp4 -s 25000000 -o output_dir
```

Then to reconstruct the file:
```bash
pvkr weld -f output_dir -o large_file.mp4
```

See `pvkr --help` for more information


## Validation

Splitting a file will create a package.pvkr file in the output directory.

This file is used to validate the integrity of the split and reconstructed files.

All files and chunks are validated using SHA256 checksums.

It is theoretically possible for someone to modify a chunk and update the sha256 in the package.pvkr file. So this tool provides **no extra security**, only validation.

## Issues

- After splitting a file you can not simply rename files in this directory (however you can rename the directory itself).
