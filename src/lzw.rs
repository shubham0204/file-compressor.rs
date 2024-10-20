use anyhow::Ok;
use anyhow::Result;

pub fn compress_file(file_path: &str, compressed_file_path: &str) -> Result<()> {
    Ok(())
}

pub fn decompress_file(compressed_file_path: &str, restored_file_path: &str) -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::compress_file;
    use super::decompress_file;
    use anyhow::Ok;
    use anyhow::Result;
    use std::fs::remove_file;
    use std::fs::File;
    use std::io::Write;

    const FILE_CONTENTS: &str = "Rust is a general-purpose programming language emphasizing performance, type safety, and concurrency. It enforces memory safety, meaning that all references point to valid memory. It does so without a traditional garbage collector; instead, both memory safety errors and data races are prevented by the \"borrow checker\", which tracks the object lifetime of references at compile time.

    Rust does not enforce a programming paradigm, but was influenced by ideas from functional programming, including immutability, higher-order functions, algebraic data types, and pattern matching. It also supports object-oriented programming via structs, enums, traits, and methods. It is popular for systems programming.[13][14][15]
    
    Software developer Graydon Hoare created Rust as a personal project while working at Mozilla Research in 2006. Mozilla officially sponsored the project in 2009. In the years following the first stable release in May 2015, Rust was adopted by companies including Amazon, Discord, Dropbox, Google (Alphabet), Meta, and Microsoft. In December 2022, it became the first language other than C and assembly to be supported in the development of the Linux kernel.
    
    Rust has been noted for its rapid adoption, and has been studied in programming language theory research.
    
    History";

    fn get_file_size_bytes(file_path: &str) -> u64 {
        File::open(file_path).unwrap().metadata().unwrap().len()
    }

    // #[test]
    fn test_lzw() -> Result<()> {
        let mut file = File::create_new("sample.txt").unwrap();
        file.write(FILE_CONTENTS.as_bytes()).unwrap();

        compress_file("sample.txt", "compressed")?;
        assert!(get_file_size_bytes("compressed") > 0);
        assert!(get_file_size_bytes("sample.txt") > get_file_size_bytes("compressed"));

        decompress_file("compressed", "sample_restored.txt")?;
        assert!(get_file_size_bytes("compressed") > 0);
        remove_file("sample.txt")?;
        remove_file("compressed")?;
        remove_file("sample_restored.txt")?;

        Ok(())
    }
}
