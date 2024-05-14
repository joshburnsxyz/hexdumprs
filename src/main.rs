use seahorse::{App, Context, Flag, FlagType};
use std::env;
use std::process::exit;
use std::io::Read;
use std::fs::File;

const GLOBAL_BUFFER_LEN: usize = 16;

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

fn get_ascii(byte_array: &mut [u8]) -> String {
    let build_string_vec: Vec<String> = byte_array.iter().map(|num| {
	if *num >= 32 && *num <= 126 { (*num as char).to_string() }
	else { '.'.to_string() }
    }).collect();

    return build_string_vec.join("");
}

fn default_action(c: &Context) {
    // Get target file from args
    let mut _file = match c.string_flag("file") {
	Ok(tf) => get_file(tf),
	Err(_) => { println!("Please supply a filepath with --file"); exit(1) },
    };

    let mut buffer = [0; GLOBAL_BUFFER_LEN];
    let mut offset: usize = 0;

    loop {
	let bytes_read = _file.read(&mut buffer);
	match bytes_read {
	    Ok(number) => {
		if number == 0 {
		    break;
		} else {
		    println!("{:08x}: {:40} {:10}",
			     offset,
			     get_hex(&mut buffer[0..number]),
			     get_ascii(&mut buffer[0..number]));
		    offset += GLOBAL_BUFFER_LEN;
		}
	    },
	    Err(err) => {
		eprintln!("ERROR: {}", err);
		break;
	    }
	}
    }

    // Exit w/o error code
    exit(0);
}
