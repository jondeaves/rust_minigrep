use std::{env, process};

use minigrep_jondeaves::Config;

/*
Excert about what main.rs should do, from https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html

The responsibilities that remain in the main function after this process should be limited to the following:
    Calling the command line parsing logic with the argument values
    Setting up any other configuration
    Calling a run function in lib.rs
    Handling the error if run returns an error
*/
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep_jondeaves::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
