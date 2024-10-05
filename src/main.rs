mod huffmann;
mod lzw;

fn main() {
    huffmann::compress_file("sample_file.txt", "compressed_file");
    huffmann::decompress_file("compressed_file", "sample_file_restored.txt");
}
