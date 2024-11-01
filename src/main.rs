use std::env;
use std::fs;

fn main() {
    // read any CL arguments passed to it and collects them into a vector
    let args: Vec<String> = env::args().collect();
    // first argument is path of binary file (like in C)
    // save values of two CL arguments

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    // read contents of file, will panic if error occurs or return Ok value
    // file is automatically closed after reading
    let contents = fs::read_to_string(file_path)
        .expect("should have been able to read the file");

    println!("With text:\n{contents}");
}

