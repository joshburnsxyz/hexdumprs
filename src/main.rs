use seahorse::{App, Context, Flag, FlagType};
use std::env;
use std::process::exit;
use std::io::Read;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new("hexdumprs")
        .description("Hexdump utility")
        .author("Josh Burns")
        .version("0.0.0")
        .action(default_action)
        .flag(
	    Flag::new("file", FlagType::String)
		.description("Specify target file")
		.alias("f"),
	);

    app.run(args);
}

fn get_file(filepath: String) -> File {
    match File::open(filepath) {
	Ok(f) => File::from(f),
	Err(e) => {
	    eprintln!("ERROR: {}", e);
	    exit(1);
	}
    }
}

fn get_hex(byte_array: &mut [u8]) -> String {
    let build_string_vec: Vec<String> = byte_array.chunks(2)
        .map(|c| {
	    if c.len() == 2 { format!("{:02x}{:02x}", c[0], c[1]) }
	    else { format!("{:02x}", c[0]) }
	}).collect();

    return build_string_vec.join(" ");
}

// TODO: fn get_ascii(byte_array: &mut [u8]) -> String {}

fn default_action(c: &Context) {
    let _file = match c.string_flag("file") {
	Ok(tf) => tf,
	Err(_) => { exit(1) },
    };

    println!("{}", _file);

    // Exit w/o error code
    exit(0);
}
