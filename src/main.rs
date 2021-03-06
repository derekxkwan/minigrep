use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(
        |err| {
            println!("error encountered: {}", err);
            process::exit(1);
        });

    if let Err(e) = minigrep::run(config) {
        println!("App oopsie: {}", e);

        process::exit(1);
    }

}

