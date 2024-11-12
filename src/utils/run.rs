use std::{error::Error, fs };
use crate::utils::config::Config;
use crate::utils::search::*;

pub fn run_program(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    let results = if config.ignore_case {
        match search_case_insensitive(&config.query, &contents) {
            Ok(results) => results,
            Err(e) => return Err(e.into()),
        }
    } else {
        match search(&config.query, &contents) {
            Ok(results) => results,
            Err(e) => return Err(e.into()),
        }
    };

    print!("\n");
    for (num, line) in results.lines {
        println!("({}) {}", num, line);
    }
    println!("\nLine count: {}\n", results.count);

    Ok(())
}