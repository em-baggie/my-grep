use boyer_moore_magiclen::BMByte;
pub struct SearchResults<'a> {
    pub lines: Vec<(&'a str, usize)>,
    pub count: usize,

}

pub fn search<'a>(query: &str, contents: &'a str) -> Result<SearchResults<'a>, String> {
    let mut results = Vec::new();
    let mut count = 0;

    for (index, line) in contents.lines().enumerate() {
        let search = BMByte::from(query).ok_or_else(|| "Failed to create search pattern")?;
        if search.find_in(line, 0).len() > 0 {
            results.push((line, index + 1));
            count += 1;
        }
    }
    Ok(SearchResults { lines: results, count: count })
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Result<SearchResults<'a>, String> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    let mut count = 0;

    for (index, line) in contents.lines().enumerate() {
        let search = BMByte::from(&query).ok_or_else(|| "Failed to create search pattern")?;
        if search.find_in(line.to_lowercase(), 0).len() > 0 {
            results.push((line, index + 1));
            count += 1;
        }
    }
    
    Ok(SearchResults { lines: results, count: count })
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
        let results = search(query, contents).unwrap();
        assert_eq!(
            vec![("safe, fast, productive.", 2)], 
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
        let results = search_case_insensitive(query, contents).unwrap();
        assert_eq!(
            vec![("Rust:", 1), ("Trust me.", 4)], 
            results.lines
        );
        assert_eq!(results.count, 2);
    }

    #[test]
    fn no_results() {
        let query = "test";
        let contents = "Rust: safe, fast, productive.";
        let results = search(query, contents).unwrap();
        assert_eq!(results.lines.len(), 0);
        assert_eq!(results.count, 0);
    }

}


