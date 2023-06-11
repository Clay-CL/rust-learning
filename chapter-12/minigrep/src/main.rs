use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Query : \"{}\";\nFile path : {}", config.query, config.file_path);

    if let Err(e) = minigrep::run(config) {
      eprintln!("Runtime Error : {e}");
      process::exit(1);
    }
}