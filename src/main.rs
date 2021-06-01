use std::{env, fs};

/*
Excert about what main.rs should do, from https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html

The responsibilities that remain in the main function after this process should be limited to the following:
    Calling the command line parsing logic with the argument values
    Setting up any other configuration
    Calling a run function in lib.rs
    Handling the error if run returns an error
*/
fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for '{}'", query);
    println!("In file '{}'", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
