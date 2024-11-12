use std::{ env, process };

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) if arg == "helpme" => {
                Config::print_help();
                process::exit(0);
            },
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let mut ignore_case = false;

        if env::var("IGNORE_CASE") == Ok("1".to_string()) {
            ignore_case = true;

        }

        if let Some(arg) = args.next() {
            if arg == "-i" {
                ignore_case = true;
            }
        }
    
        Ok(Config {
            query,
            file_path,
            ignore_case
        })
        
    }

    pub fn print_help() {
        println!("Mygrep help:");
        println!("1. To search for a query in a file: cargo run -- <query> <file_path>");
        println!("2. To enable case insensitive search, either use environmental variable or command line argument:
        - to enable case insensitive search, set the environmental variable: export IGNORE_CASE=1
        - to disable case insensitive search, unset the environmental variable: unset IGNORE_CASE
        - to use the command line argument to enable case insensitive search: cargo run -- <query> <file_path> -i");
        println!("4. To display this help message: cargo run -- helpme");
    }
}