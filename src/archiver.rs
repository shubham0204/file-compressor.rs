use anyhow::Ok;
use anyhow::Result;
use std::fs::File;
use std::io::copy;
use std::io::Read;
use std::io::Write;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::vec;

fn read_u64(file: &mut File) -> Result<u64> {
    let mut buffer = [0u8; 8];
    file.read_exact(&mut buffer)?;
    Ok(u64::from_be_bytes(buffer))
}

pub fn archive_files(paths: &[&str], archive_file_path: &str) -> Result<()> {
    let mut archive_file = File::create_new(archive_file_path)?;
    archive_file.write(&(paths.len() as u64).to_be_bytes())?;
    for path in paths.into_iter() {
        let mut file = File::open(path)?;
        let file_path = Path::new(path);
        let file_name = file_path.file_name().unwrap();
        let file_size = file.metadata().unwrap().len() as u64;
        archive_file.write(&(file_name.len() as u64).to_be_bytes())?;
        archive_file.write(file_name.as_bytes())?;
        archive_file.write(&(file_size.to_be_bytes()))?;
        copy(&mut file, &mut archive_file)?;
    }
    Ok(())
}

pub fn unarchive_files(archive_file_path: &str) -> Result<()> {
    let mut archive_file = File::open(archive_file_path)?;
    let num_files = read_u64(&mut archive_file)?;
    for _ in 0..num_files {
        // read length of file-name
        let file_name_len = read_u64(&mut archive_file)? as usize;
        let mut file_name_bytes = vec![0u8; file_name_len];
        archive_file.read_exact(&mut file_name_bytes)?;

        // read file-size and file-contents
        let file_size = read_u64(&mut archive_file)? as usize;
        let mut file_bytes = vec![0u8; file_size];
        archive_file.read_exact(&mut file_bytes)?;

        // create a new file and write file_bytes
        let mut file = File::create_new(String::from_utf8(file_name_bytes)?)?;
        file.write(&file_bytes)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use anyhow::Ok;

    use super::archive_files;
    use super::unarchive_files;
    use anyhow::Result;
    use std::fs::remove_file;
    use std::fs::File;
    use std::io::Write;

    const FILE_CONTENTS: &str = "Rust is a general-purpose programming language emphasizing performance, type safety, and concurrency. It enforces memory safety, meaning that all references point to valid memory. It does so without a traditional garbage collector; instead, both memory safety errors and data races are prevented by the \"borrow checker\", which tracks the object lifetime of references at compile time.

    Rust does not enforce a programming paradigm, but was influenced by ideas from functional programming, including immutability, higher-order functions, algebraic data types, and pattern matching. It also supports object-oriented programming via structs, enums, traits, and methods. It is popular for systems programming.[13][14][15]
    
    Software developer Graydon Hoare created Rust as a personal project while working at Mozilla Research in 2006. Mozilla officially sponsored the project in 2009. In the years following the first stable release in May 2015, Rust was adopted by companies including Amazon, Discord, Dropbox, Google (Alphabet), Meta, and Microsoft. In December 2022, it became the first language other than C and assembly to be supported in the development of the Linux kernel.
    
    Rust has been noted for its rapid adoption, and has been studied in programming language theory research.
    
    History";

    fn get_file_size_bytes(file_path: &str) -> Result<u64> {
        Ok(File::open(file_path)?.metadata()?.len())
    }

    // TODO: enable test
    // #[test]
    fn test_archive_files() -> Result<()> {
        let mut file1 = File::create_new("sample1.txt")?;
        file1.write(FILE_CONTENTS.as_bytes())?;
        let mut file2 = File::create_new("sample2.txt")?;
        file2.write(FILE_CONTENTS.as_bytes())?;

        let input_file_paths = ["sample1.txt", "sample2.txt"];
        archive_files(&input_file_paths, "archive")?;

        assert!(
            get_file_size_bytes("archive")?
                > get_file_size_bytes("sample1.txt")? + get_file_size_bytes("sample2.txt")?
        );

        remove_file("sample1.txt")?;
        remove_file("sample2.txt")?;
        remove_file("archive")?;
        Ok(())
    }

    // TODO: enable test
    // #[test]
    fn test_unarchive_files() -> Result<()> {
        let mut file1 = File::create_new("sample1.txt")?;
        file1.write(FILE_CONTENTS.as_bytes())?;
        let mut file2 = File::create_new("sample2.txt")?;
        file2.write(FILE_CONTENTS.as_bytes())?;

        let input_file_paths = ["sample1.txt", "sample2.txt"];
        archive_files(&input_file_paths, "archive")?;

        remove_file("sample1.txt")?;
        remove_file("sample2.txt")?;

        unarchive_files("archive")?;

        assert!(get_file_size_bytes("sample1.txt")? > 0);
        assert!(get_file_size_bytes("sample2.txt")? > 0);

        remove_file("sample1.txt")?;
        remove_file("sample2.txt")?;
        remove_file("archive")?;
        Ok(())
    }
}
