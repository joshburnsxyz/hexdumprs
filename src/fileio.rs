use std::fs::File;
use std::io;

pub fn read_file(file_path: &str) -> io::Result<File> {
    File::open(file_path)
}
