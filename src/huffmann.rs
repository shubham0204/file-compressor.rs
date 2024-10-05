use bitstream_io::huffman::compile_read_tree;
use bitstream_io::huffman::compile_write_tree;
use bitstream_io::BigEndian;
use bitstream_io::BitRead;
use bitstream_io::BitReader;
use bitstream_io::BitWrite;
use bitstream_io::BitWriter;
use bitstream_io::HuffmanRead;
use bitstream_io::HuffmanWrite;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use std::io::Seek;
use std::io::Write;

struct BinaryTreeNode {
    symbol: u8,
    weight: u32,
    left: Option<Box<BinaryTreeNode>>,
    right: Option<Box<BinaryTreeNode>>,
}

impl Ord for BinaryTreeNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for BinaryTreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.weight.cmp(&self.weight))
    }
}

impl PartialEq for BinaryTreeNode {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for BinaryTreeNode {}

fn construct_min_heap_with_nodes(symbol_table: &HashMap<u8, u32>) -> BinaryHeap<BinaryTreeNode> {
    let mut min_heap: BinaryHeap<BinaryTreeNode> = BinaryHeap::new();
    for (symbol, count) in symbol_table.iter() {
        min_heap.push(BinaryTreeNode {
            symbol: *symbol,
            weight: *count,
            left: None,
            right: None,
        });
    }
    min_heap
}

fn build_huffman_tree(min_heap: &mut BinaryHeap<BinaryTreeNode>) -> BinaryTreeNode {
    while min_heap.len() > 1 {
        let left_node = min_heap.pop().unwrap();
        let right_node = min_heap.pop().unwrap();
        let intermediate_node = BinaryTreeNode {
            symbol: 0, // no symbols in intermediate nodes,
            weight: left_node.weight + right_node.weight,
            left: Some(Box::new(left_node)),
            right: Some(Box::new(right_node)),
        };
        min_heap.push(intermediate_node);
    }
    min_heap.pop().unwrap()
}

fn traverse_huffmann_tree(
    huffmann_tree_node: &BinaryTreeNode,
    code_bits: &mut Vec<u8>,
    codes: &mut HashMap<u8, Vec<u8>>,
) {
    if huffmann_tree_node.symbol != 0u8 {
        codes.insert(huffmann_tree_node.symbol, code_bits.clone());
    }
    if huffmann_tree_node.left.is_some() {
        code_bits.push(0u8);
        traverse_huffmann_tree(huffmann_tree_node.left.as_ref().unwrap(), code_bits, codes);
        code_bits.pop();
    }
    if huffmann_tree_node.right.is_some() {
        code_bits.push(1u8);
        traverse_huffmann_tree(huffmann_tree_node.right.as_ref().unwrap(), code_bits, codes);
        code_bits.pop();
    }
}

fn encode_symbol_table(huffmann_tree_root: &BinaryTreeNode) -> HashMap<u8, Vec<u8>> {
    let mut codes: HashMap<u8, Vec<u8>> = HashMap::new();
    let mut code_bits: Vec<u8> = Vec::new();
    traverse_huffmann_tree(huffmann_tree_root, &mut code_bits, &mut codes);
    codes
}

pub fn compress_file(file_path: &str, compressed_file_path: &str) {
    let mut file =
        File::open(file_path).expect("Unable to open file at `file_path` in compress_file()");

    // construct symbol table
    let mut symbol_table: HashMap<u8, u32> = HashMap::new();
    let mut buffer: [u8; 128] = [0; 128];
    let mut bytes_read = file.read(&mut buffer).expect("Error reading file");
    while bytes_read > 0 {
        for i in 0..bytes_read {
            *symbol_table.entry(buffer[i]).or_insert(0) += 1;
        }
        bytes_read = file.read(&mut buffer).expect("Error reading file");
    }

    // construct huffmann tree
    let mut heap = construct_min_heap_with_nodes(&symbol_table);
    let tree_root_node = build_huffman_tree(&mut heap);

    // encode the symbol tree using the huffmann tree
    // transform symbols to variable-size optimal prefix codes
    let encoded_symbol_table = encode_symbol_table(&tree_root_node);

    let tree_vec: Vec<(u8, Vec<u8>)> = encoded_symbol_table.clone().into_iter().collect();
    let num_pairs = tree_vec.len() as u32;

    let mut compressed_data = Vec::new();
    let mut bit_writer = BitWriter::endian(&mut compressed_data, BigEndian);

    // write the symbol table to `compressed_data`:
    // 1. write number of (symbol, code) pairs in `encoded_symbol_table`
    // 2. for each (symbol, code) pair, write the symbol
    // 2.a. `code` is a vector of bits with variable size; write the size of `code`
    // 2.b. write each bit of the `code` to the file
    bit_writer.write_bytes(&num_pairs.to_be_bytes()).unwrap();
    for (symbol, code) in tree_vec.iter() {
        bit_writer.write_bytes(&[*symbol]).unwrap();
        bit_writer
            .write_bytes(&(code.len() as u32).to_be_bytes())
            .unwrap();
        for code_bit in code.iter() {
            bit_writer.write_bit(*code_bit == 1u8).unwrap();
        }
    }

    // reset file pointer to start reading from the beginning
    // (but this time to encode data)
    file.seek(std::io::SeekFrom::Start(0)).unwrap();

    let tree = compile_write_tree::<BigEndian, u8>(tree_vec)
        .expect("Unable to create huffmann tree with compile_write_tree");
    let mut buffer: [u8; 128] = [0; 128];
    let mut bytes_read = file.read(&mut buffer).expect("Error reading file");
    while bytes_read > 0 {
        for i in 0..bytes_read {
            bit_writer.write_huffman(&tree, buffer[i]).unwrap();
        }
        bytes_read = file.read(&mut buffer).expect("Error reading file");
    }

    let mut compressed_file =
        File::create_new(compressed_file_path).expect("Could not create new compressed file");
    compressed_file.write(compressed_data.as_slice()).unwrap();
}

pub fn decompress_file(compressed_file_path: &str, restored_file_path: &str) {
    // read all contents of `compressed_file` in `compressed_data`
    let mut compressed_file = File::open(compressed_file_path)
        .expect("Unable to open file at `compressed_file_path` in decompress_file()");
    let mut compressed_data: Vec<u8> = Vec::new();
    compressed_file.read_to_end(&mut compressed_data).unwrap();
    let mut bit_reader = BitReader::endian(Cursor::new(&compressed_data), BigEndian);

    // read the symbol table from the file
    let mut codes: Vec<(u8, Vec<u8>)> = Vec::new();
    let num_pairs: u32 = bit_reader.read_as_to::<BigEndian, u32>().unwrap();
    for _ in 0..num_pairs {
        let symbol: u8 = bit_reader.read_as_to::<BigEndian, u8>().unwrap();
        let code_len: u32 = bit_reader.read_as_to::<BigEndian, u32>().unwrap();
        let mut code: Vec<u8> = Vec::new();
        for _ in 0..code_len {
            let code_bit: bool = bit_reader.read_bit().unwrap();
            code.push(code_bit as u8);
        }
        codes.push((symbol, code));
    }

    let tree = compile_read_tree::<BigEndian, u8>(codes).unwrap();
    let mut original_data: Vec<u8> = Vec::new();
    let mut byte = bit_reader.read_huffman(&tree);
    while byte.is_ok() {
        original_data.push(byte.unwrap());
        byte = bit_reader.read_huffman(&tree);
    }

    let mut original_file =
        File::create_new(restored_file_path).expect("Could not create new original file");
    original_file.write(&original_data.as_slice()).unwrap();
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
