use boyer_moore_magiclen::BMByte;

pub struct SearchResults<'a> {
    pub lines: Vec<(usize, &'a str)>,
    pub count: usize,

}

pub fn search<'a>(query: &str, contents: &'a str) -> Result<SearchResults<'a>, String> {
    // inputting word and contents of file, want to output search results

    let search = BMByte::from(query).ok_or_else(|| "Failed to create search pattern")?;

    let lines: Vec<(usize, &'a str)> = contents
        .lines() // make contents into iterator where each item is a line
        .enumerate() // transforms iterator into (index, line)
        .filter(|(_, line)| search.find_full_in(line, 0).len() > 0) // filters the enumerated lines, retains only those containing query substring
        .map(|(index, line)| (index + 1, line)) // creates tuple for each line that includes the line itself and its indecx incremented by 1
        .collect();

    let count = lines.len();

    Ok(SearchResults { lines: lines, count: count })
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Result<SearchResults<'a>, String> {
    let query_lower = query.to_lowercase();
    let search = BMByte::from(query_lower).ok_or_else(|| "Failed to create search pattern")?;

    let lines: Vec<(usize, &'a str)> = contents
        .lines() // make contents into iterator where each item is a line
        .enumerate() // transforms iterator into (index, line)
        .filter(|(_, line)| search.find_full_in(line.to_lowercase(), 0).len() > 0) // filters the enumerated lines, retains only those containing query substring
        .map(|(index, line)| (index + 1, line)) // creates tuple for each line that includes the line itself and its indecx incremented by 1
        .collect();

    let count = lines.len();

    Ok(SearchResults { lines: lines, count: count })
    
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
            vec![(2, "safe, fast, productive.")], 
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
            vec![(1, "Rust:"), (4, "Trust me.")], 
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


