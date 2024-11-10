# **my-grep**
A simple command line search tool completed as part of <a href = "https://doc.rust-lang.org/book/ch12-00-an-io-project.html">The Rust Programming Language</a> book. 

---- 

**Extra features compared to the book's example:**

- Counts and displays the number of lines that match the query
- Uses <a href="https://crates.io/crates/boyer-moore-magiclen">Beyer Moore Magiclen</a> as a more efficient search method than linear search
- Displays the line number of each line that matches the query
- Option to display a help command that outputs all the commands and their syntax
- Code refactored into modules

----

**How to run:**

----

**Currently further developing the project compared to the book's example to include:**

- search for whole word matches and output highlighted matches
- control case sensitivity through either a command line argument or an environment variable - decide which takes precedence