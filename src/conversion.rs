pub fn print_hex(bytes: &[u8], chunksize: usize) -> String {
    bytes
        .chunks(chunksize)
        .map(|chunk| match chunk.len() {
            2 => format!("{:02x}{:02x}", chunk[0], chunk[1]),
            3 => format!("{:02x}{:02x}{:02x}", chunk[0], chunk[1], chunk[2]),
            4 => format!("{:02x}{:02x}{:02x}{:02x}", chunk[0], chunk[1], chunk[2], chunk[3]),
            _ => format!("{:02x}", chunk[0]),
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn print_ascii(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|&byte| if (32..=126).contains(&byte) { byte as char } else { '.' })
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_hex_single_chunk() {
        let bytes = [0x61, 0x62, 0x63, 0x64]; // "abcd" in ASCII
        let chunksize = 2;
        let hex_string = print_hex(&bytes, chunksize);
        assert_eq!(hex_string, "6162 6364");
    }

    #[test]
    fn test_print_hex_multiple_chunks() {
        let bytes = [0x61, 0x62, 0x63, 0x64, 0x65, 0x66]; // "abcdef" in ASCII
        let chunksize = 3;
        let hex_string = print_hex(&bytes, chunksize);
        assert_eq!(hex_string, "616263 646566");
    }

    #[test]
    fn test_print_ascii() {
        let bytes = [0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64]; // "Hello, world" in ASCII
        let ascii_string = print_ascii(&bytes);
        assert_eq!(ascii_string, "Hello, world");
    }

    #[test]
    fn test_print_ascii_non_printable_characters() {
        let bytes = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C]; // Non-printable characters
        let ascii_string = print_ascii(&bytes);
        assert_eq!(ascii_string, "............");
    }
}
