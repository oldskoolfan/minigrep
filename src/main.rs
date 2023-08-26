use std::{env, process};

use minigrep::run::run;
use minigrep::config::Config;

fn main() {
    if let Err(e) = run(
        get_config_from_args(env::args())
    ) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}

fn get_config_from_args(args: impl Iterator<Item = String>) -> Config {
    Config::build(args)
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        })
}
