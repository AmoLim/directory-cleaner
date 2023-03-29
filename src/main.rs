use std::env;
use std::process;

use directory_cleaner::DeleteConfig;

fn main() {
    let config = DeleteConfig::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing argument: {err}");
        process::exit(1);
    });

    if let Err(e) = directory_cleaner::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

}
