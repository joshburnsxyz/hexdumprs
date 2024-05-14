use std::fs::File;
use std::io::{self, Error, ErrorKind};

pub fn read_file(file_path: &str) -> io::Result<File> {
    match File::open(file_path) {
	Ok(file) => Ok(file),
	Err(err) => {
	    let errmsg = format!("Failed to open file '{}': {}", file_path, err);
	    return Err(Error::new(ErrorKind::NotFound, errmsg));
	}
    }
}
