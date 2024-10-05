// TODO: implement lzw compression algorithm

pub fn compress_file(file_path: &str, compressed_file_path: &str) {
    !todo!()
}

pub fn decompress_file(compressed_file_path: &str, restored_file_path: &str) {
    !todo!()
}

#[cfg(test)]
mod tests {

    use super::compress_file;
    use super::decompress_file;
    use std::fs::remove_file;
    use std::fs::File;
    use std::io::Write;

    const FILE_CONTENTS: &str = "Rust is a general-purpose programming language emphasizing performance, type safety, and concurrency. It enforces memory safety, meaning that all references point to valid memory. It does so without a traditional garbage collector; instead, both memory safety errors and data races are prevented by the \"borrow checker\", which tracks the object lifetime of references at compile time.

    Rust does not enforce a programming paradigm, but was influenced by ideas from functional programming, including immutability, higher-order functions, algebraic data types, and pattern matching. It also supports object-oriented programming via structs, enums, traits, and methods. It is popular for systems programming.[13][14][15]
    
    Software developer Graydon Hoare created Rust as a personal project while working at Mozilla Research in 2006. Mozilla officially sponsored the project in 2009. In the years following the first stable release in May 2015, Rust was adopted by companies including Amazon, Discord, Dropbox, Google (Alphabet), Meta, and Microsoft. In December 2022, it became the first language other than C and assembly to be supported in the development of the Linux kernel.
    
    Rust has been noted for its rapid adoption, and has been studied in programming language theory research.
    
    History";

    fn setup() {
        let mut file = File::create_new("sample.txt").unwrap();
        file.write(FILE_CONTENTS.as_bytes()).unwrap();
    }

    fn clean() {
        remove_file("sample.txt").unwrap_or_default();
        remove_file("sample_restored.txt").unwrap_or_default();
        remove_file("compressed").unwrap_or_default();
    }

    fn get_file_size_bytes(file_path: &str) -> u64 {
        File::open(file_path).unwrap().metadata().unwrap().len()
    }

    #[test]
    fn test_compress_file() {
        setup();
        compress_file("sample.txt", "compressed");
        assert!(get_file_size_bytes("compressed") > 0);
        assert!(get_file_size_bytes("sample.txt") > get_file_size_bytes("compressed"));
        clean();
    }

    // TODO: check difference between file sizes
    #[test]
    fn test_decompress_file() {
        setup();
        compress_file("sample.txt", "compressed");
        decompress_file("compressed", "sample_restored.txt");
        println!("{}",get_file_size_bytes("sample_restored.txt"));
        println!("{}",get_file_size_bytes("sample.txt"));
        assert!(get_file_size_bytes("compressed") > 0);
        assert!(get_file_size_bytes("sample_restored.txt") == get_file_size_bytes("sample.txt"));
        clean();
    }
}
