use std::io::Read;

use crate::cli::{term_ok, WeldArgs};
use crate::error::{PvkrError, Result};
use crate::sha256::Sha256;
use crate::util::Streamable;

pub fn weld(args: WeldArgs) -> Result<()> {
    // Validate the package directory
    if !args.target_dir.exists() || !args.target_dir.is_dir() {
        return PvkrError::not_a_pvkr_package(format!(
            "The directory {} does not exist or is a file",
            args.target_dir.display()
        ));
    }
    let package_file = args.target_dir.join("package.pvkr");
    if !package_file.exists() {
        return PvkrError::not_a_pvkr_package("Directory has no package.pvkr file");
    }

    // open the package file as a stream
    let f = std::fs::File::open(args.target_dir.join("package.pvkr"))?;
    let mut stream = std::io::BufReader::new(f);

    // Read the final sha256 from the package
    let expected_sha256 = Sha256::read_from(&mut stream)?;

    // Read the base_name from the package
    let base_name_len = u128::read_from(&mut stream)? as usize;
    let base_name_bytes = {
        let mut buffer: Vec<u8> = vec![0; base_name_len];
        stream.read_exact(&mut buffer)?;
        buffer
    };
    let base_name = String::from_utf8(base_name_bytes)?;
    term_ok(format!("Processing package: {}", &base_name));

    // Read all chunk sha256 hashes from the package and validate all chunks
    let mut chunks = Vec::new();
    let mut chunk_count = 0u128;
    while let Some(expected_sha256) = Sha256::maybe_read_from(&mut stream)? {
        let chunk_file = args
            .target_dir
            .join(format!("{}-{}", base_name, chunk_count));
        let actual_sha256 = Sha256::from_file(&chunk_file)?;

        if expected_sha256 != actual_sha256 {
            return PvkrError::validation_error(format!(
                "Chunk {} SHA256 mismatch, file corrupted",
                chunk_file.display()
            ));
        }

        chunks.push(actual_sha256);
        chunk_count += 1;
    }
    term_ok(format!("{} chunks validated", chunk_count));

    // Prepare output stream
    let output_path = std::fs::File::create(&args.output)?;
    let mut output_stream = std::io::BufWriter::new(output_path);

    // Stream data
    for i in 0..chunks.len() {
        let chunk_file = args.target_dir.join(format!("{}-{}", base_name, i));
        let mut chunk_file = std::fs::File::open(chunk_file)?;
        std::io::copy(&mut chunk_file, &mut output_stream)?;
        term_ok(format!("Chunk {} written", i));
    }

    // Validate the output file
    let actual_sha256 = Sha256::from_file(&args.output)?;
    if expected_sha256 != actual_sha256 {
        return PvkrError::validation_error("Output file SHA256 mismatch, file corrupted");
    }
    term_ok(format!(
        "Validated output file: {} [SHA256:{}]",
        &args.output.display(),
        actual_sha256
    ));

    Ok(())
}
