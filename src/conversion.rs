pub fn print_hex(bytes: &[u8], chunksize: usize) -> String {
    bytes
        .chunks(chunksize)
        .map(|chunk| match chunk.len() {
            2 => format!("{:02x}{:02x}", chunk[0], chunk[1]),
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
