mod conversion;
mod fileio;

use seahorse::{App, Context, Flag, FlagType};
use std::env;
use std::io::Read;
use std::process::exit;
use conversion::{print_ascii, print_hex};
use fileio::read_file;

const GLOBAL_BUFFER_LEN: usize = 16;

fn main() {
    let args: Vec<String> = env::args().collect();

    let app = App::new("hexdumprs")
        .description("Hexdump utility")
        .author("Josh Burns")
        .version("0.0.0")
        .usage("hexdumprs [-f FILE] [OPTIONS]")
        .flag(
            Flag::new("file", FlagType::String)
                .description("Specify target file")
                .alias("f"),
        )
        .flag(
            Flag::new("ascii", FlagType::Bool)
                .description("Only print the ASCII values")
                .alias("a"),
        )
        .flag(
            Flag::new("hex", FlagType::Bool)
                .description("Only print the hexadecimal values")
                .alias("H"),
        )
        .flag(
	    Flag::new("chunksize", FlagType::Int)
		.description("Change chunk size for hex conversion")
		.alias("c"),
	)
        .action(default_action);

    app.run(args);
}



fn default_action(context: &Context) {
    let file_path = match context.string_flag("file") {
        Ok(path) => path,
        Err(_) => {
            println!("Please supply a filepath with --file");
            exit(1);
        }
    };

    let chunksize: usize = match context.uint_flag("chunksize") {
	Ok(us) => us,
	Err(_) => 2
    };

    let mut file = match read_file(&file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("ERROR: {}", err);
            exit(1);
        }
    };

    let mut buffer = [0; GLOBAL_BUFFER_LEN];
    let mut offset = 0;

    loop {
        match file.read(&mut buffer) {
            Ok(0) => break,
            Ok(bytes_read) => {
                if context.bool_flag("hex") {
                    println!("{:40}", print_hex(&buffer[..bytes_read], chunksize));
                } else if context.bool_flag("ascii") {
                    println!("{:10}", print_ascii(&buffer[..bytes_read]));
                } else {
                    println!(
                        "{:08x}: {:40} {:10}",
                        offset,
                        print_hex(&buffer[..bytes_read], chunksize),
                        print_ascii(&buffer[..bytes_read])
                    );
                }
                offset += GLOBAL_BUFFER_LEN;
            }
            Err(err) => {
                eprintln!("ERROR: {}", err);
                break;
            }
        }
    }

    exit(0);
}
