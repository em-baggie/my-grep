use std::{error::Error, fs, env};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

pub struct SearchResults<'a> {
    pub lines: Vec<&'a str>,
    pub count: usize,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();
    
        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results.lines {
        println!("{line}");
    }
    println!("Line count: {}", results.count);

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> SearchResults<'a> {
    let mut results = Vec::new();
    let mut count = 0;

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
            count += 1;
        }
    }
    SearchResults { lines: results, count: count }
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> SearchResults<'a> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    let mut count = 0;

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
            count += 1;
        }
    }
    
    SearchResults { lines: results, count: count }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        let results = search(query, contents);
        assert_eq!(
            vec!["safe, fast, productive."], 
            results.lines
        );
        assert_eq!(results.count, 1);
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        let results = search_case_insensitive(query, contents);
        assert_eq!(
            vec!["Rust:", "Trust me."], 
            results.lines
        );
        assert_eq!(results.count, 2);
    }

}


