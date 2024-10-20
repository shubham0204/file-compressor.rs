mod archiver;
mod huffmann;
mod lzw;
use anyhow::{Ok, Result};
use clap::arg;
use clap::Command;

fn cli() -> Command {
    Command::new("file-compressor")
        .about("File compressor")
        .author("Shubham Panchal")
        .version("0.0.1")
        .subcommand(
            Command::new("compress").about("compress a file").args(&[
                arg!([FILEPATH] "path to the file to compress").required(true),
                arg!([COMPRESSED_FILEPATH] "path to write the compressed file").required(true),
                arg!([METHOD] "Compression method")
                    .required(false)
                    .value_parser(["lzw", "huffmann"])
                    .default_value("huffmann")
                    .default_missing_value("huffmann"),
            ]),
        )
        .subcommand(
            Command::new("decompress")
                .about("de-compress a file")
                .args(&[
                    arg!([COMPRESSED_FILEPATH] "path to the compressed file").required(true),
                    arg!([DECOMPRESSED_FILEPATH] "path to write the decompressed file")
                        .required(true),
                ]),
        )
        .subcommand(
            Command::new("archive")
                .about("combine multiple files into a single file")
                .args(&[
                    arg!([ARCHIVE_FILEPATH] "path to write the archive file").required(true),
                    arg!([FILEPATHS] "paths to the files to archive")
                        .required(true)
                        .num_args(1..)
                        .value_delimiter(' '),
                ]),
        )
        .subcommand(
            Command::new("unarchive")
                .about("extract files from an archive")
                .args(&[
                    arg!([ARCHIVE_FILEPATH] "path to the archive file").required(true),
                    arg!([TARGET_DIR] "path to write the extracted files").required(true),
                ]),
        )
}

fn main() -> Result<()> {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("compress", sub_matches)) => {
            let input_filepath = sub_matches
                .get_one::<String>("FILEPATH")
                .map(|s| s.to_string())
                .unwrap();
            let output_filepath = sub_matches
                .get_one::<String>("COMPRESSED_FILEPATH")
                .map(|s| s.to_string())
                .unwrap();
            let method = sub_matches
                .get_one::<String>("METHOD")
                .map(|s| s.to_string())
                .unwrap();
            match method.as_str() {
                "lzw" => lzw::compress_file(&input_filepath, &output_filepath)?,
                "huffmann" => huffmann::compress_file(&input_filepath, &output_filepath)?,
                _ => unreachable!(),
            }
        }
        Some(("decompress", sub_matches)) => {
            let input_filepath = sub_matches
                .get_one::<String>("COMPRESSED_FILEPATH")
                .map(|s| s.to_string())
                .unwrap();
            let output_filepath = sub_matches
                .get_one::<String>("DECOMPRESSED_FILEPATH")
                .map(|s| s.to_string())
                .unwrap();
            huffmann::decompress_file(&input_filepath, &output_filepath)?;
        }
        Some(("archive", sub_matches)) => {
            let archive_filepath = sub_matches
                .get_one::<String>("ARCHIVE_FILEPATH")
                .map(|s| s.to_string())
                .unwrap();
            let filepaths: Vec<&String> = sub_matches.get_many("FILEPATHS").unwrap().collect();
            let filepaths_str: Vec<&str> = filepaths.into_iter().map(|s| s.as_str()).collect();
            archiver::archive_files(filepaths_str.as_slice(), &archive_filepath)?;
        }
        Some(("unarhive", sub_matches)) => {
            let archive_filepath = sub_matches
                .get_one::<String>("ARCHIVE_FILEPATH")
                .map(|s| s.to_string())
                .unwrap();
            let target_dir = sub_matches
                .get_one::<String>("TARGET_DIR")
                .map(|s| s.to_string())
                .unwrap();
            archiver::unarchive_files(&archive_filepath, &target_dir)?;
        }
        _ => unreachable!(),
    }

    Ok(())
}
