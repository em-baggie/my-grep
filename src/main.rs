use std::env;
use std::fs;

fn main() {
    // read any CL arguments passed to it and collects them into a vector
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    // read contents of file, will panic if error occurs or return Ok value
    // file is automatically closed after reading
    let contents = fs::read_to_string(config.file_path).expect("should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

// 
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    // cloning ensures that the struct has ownership of the data
    Config { 
        query,
        file_path,
    }

}



