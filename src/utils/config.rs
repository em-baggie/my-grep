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
            Some(arg) if arg == "help" => {
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

        let ignore_case = env::var("IGNORE_CASE").is_ok();
    
        Ok(Config {
            query,
            file_path,
            ignore_case
        })
        
    }

    pub fn print_help() {
        println!("Mygrep help:");
        println!("1. To search for a query in a file: mygrep <query> <file_path>");
        println!("2. To enable case insensitive search, set the environmental variable: export IGNORE_CASE=1");
        println!("3. To disable case insensitive search: unset IGNORE_CASE");
        println!("4. To display this help message: mygrep help");
    }
}