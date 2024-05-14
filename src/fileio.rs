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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_existing_file() {
        let file_path = "test_files/test.txt"; // Path to an existing file
        let result = read_file(file_path);
        assert!(result.is_ok(), "Failed to read existing file: {:?}", result);
    }

    #[test]
    fn test_read_nonexistent_file() {
        let file_path = "nonexistent_file.txt"; // Path to a nonexistent file
        let result = read_file(file_path);
        assert!(result.is_err(), "Reading nonexistent file should result in an error");
    }

    #[test]
    fn test_read_directory_instead_of_file() {
        let file_path = "test_files"; 
        let result = read_file(file_path);
        assert!(result.is_err(), "Reading directory instead of file should result in an error");
    }
}
