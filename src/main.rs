mod archiver;
mod huffmann;
mod lzw;

use anyhow::{Ok, Result};

fn main() -> Result<()> {
    // huffmann::compress_file("sample_file.txt", "compressed_file");
    // huffmann::decompress_file("compressed_file", "sample_file_restored.txt");
    let input_file_paths = ["inputs/sample1.txt", "inputs/sample2.txt"];
    archiver::archive_files(&input_file_paths, "archive")?;
    archiver::unarchive_files("archive")?;
    Ok(())
}
