use std::env::args;
use std::process;

fn main() {
    let config = minigrep::Config::build(args())
        .unwrap_or_else(|e| {
            eprintln!("program failed with error: {e}");
            process::exit(1);
        });

    if let Err(e) = minigrep::run(&config) {
        eprintln!("program failed with error: {e}");
        process::exit(1);
    }
}
