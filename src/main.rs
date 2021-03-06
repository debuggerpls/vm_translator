use std::env;
use std::process;

use vm_translator::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = vm_translator::run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
