use std::{ env, process };

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(
        &args
    ).unwrap_or_else(|err| {
        // using eprintln to avoid bloating
        // the output of stdout (thus using
        // stderr for errors).
        eprintln!(
            "Problem parsing arguments: {err}"
        );
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}