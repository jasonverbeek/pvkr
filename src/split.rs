use std::cmp::min;
use std::io::{Read, Write};

use crate::cli::{term_ok, term_warn, SplitArgs};
use crate::error::{PvkrError, Result};
use crate::sha256::{Sha256, Sha256Builder};
use crate::util::Streamable;

pub fn split(args: SplitArgs) -> Result<()> {
    // Check if the file exists
    if !args.file.exists() {
        return PvkrError::file_not_found(format!("{:?}", args.file));
    }

    // Open file and check its size
    let file = std::fs::File::open(&args.file)?;
    let stat = file.metadata()?;
    let file_size = stat.len() as u128;

    // Calculate the sha256 of the file
    let sha256 = Sha256::from_file(&args.file)?;
    term_ok(format!(
        "Processing {} [SHA256:{}]",
        &args.file.display(),
        sha256
    ));

    // Validate and prepare the output directory
    if !args.output.exists() {
        std::fs::create_dir_all(&args.output)?;
    }
    if std::fs::read_dir(&args.output)?.count() != 0 {
        if !args.overwrite {
            return PvkrError::validation_error("Output directory is not empty");
        } else {
            term_warn("Output directory is not empty, overwriting because --overwrite flag is set");
            std::fs::remove_dir_all(&args.output)?;
            std::fs::create_dir_all(&args.output)?;
        }
    }

    // Open package.pvkr data stream
    let data_file_path = args.output.join("package.pvkr");
    let data_file = std::fs::File::create(data_file_path)?;
    let mut data_file_stream = std::io::BufWriter::new(data_file);

    // Write the final sha256 to the package
    sha256.write_to(&mut data_file_stream)?;

    // Get base_name based on the output_dir name
    let base_name = match args.output.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => {
            term_warn("Output directory name is not valid, an alternative name will be used for the data files");
            "data".to_string()
        }
    };
    // write base_name info to the package
    let base_name_bytes = base_name.as_bytes();
    let base_name_len = base_name_bytes.len() as u128;
    base_name_len.write_to(&mut data_file_stream)?;
    data_file_stream.write_all(base_name_bytes)?;

    // Split the file into chunks
    let mut input_stream = std::io::BufReader::new(file);
    let mut total_read = 0u128;
    let mut current_chunk = 0u128;
    loop {
        // prepare output stream
        let file_path = args.output.join(format!("{}-{}", base_name, current_chunk));
        let mut output_stream = std::fs::File::create(&file_path)?;

        // remaining size for this chunk
        let mut remaining = args.chunk_size_bytes;

        // calculate the chunk sha256 while its being written
        let mut sha256_builder = Sha256Builder::new();

        loop {
            // calc the num of bytes to read (<= 1024)
            let read_size = min(remaining as usize, 1024);
            // read from input stream
            let mut buffer = vec![0; read_size];
            let count = input_stream.read(&mut buffer)?;
            total_read += count as u128;
            // write to output stream
            output_stream.write_all(&buffer[..count])?;
            // update the sha256_builder
            sha256_builder.update(&buffer[..count]);
            // update the remaining size
            remaining -= count as u128;
            if remaining == 0 || count == 0 {
                break;
            }
        }
        output_stream.flush()?;

        // get the sha256 of the chunk and write it to the package
        let chunk_sha256 = sha256_builder.finish();
        chunk_sha256.write_to(&mut data_file_stream)?;
        term_ok(format!(
            "Written chunk {} [SHA256:{}]",
            current_chunk, chunk_sha256
        ));
        current_chunk += 1;
        // if done, stop the loop
        if total_read == file_size {
            break;
        }
    }

    Ok(())
}
