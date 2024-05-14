use seahorse::{App, Context, Flag, FlagType};
use std::env;
use std::process::exit;

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

fn default_action(c: &Context) {
    let _file = match c.string_flag("file") {
	Ok(tf) => tf,
	Err(_) => { exit(1) },
    };

    println!("{}", _file);

    // Exit w/o error code
    exit(0);
}
