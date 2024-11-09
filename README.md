# **my-grep**
A simple command line search tool completed as part of <a href = "https://doc.rust-lang.org/book/ch12-00-an-io-project.html">The Rust Programming Language</a> book. 

---- 

Extra features compared to the book's example:

- counts and displays the number of lines that match the query
- Uses <a href="https://crates.io/crates/boyer-moore-magiclen">Beyer Moore Magiclen</a> as a more efficient search method than linear search.
- displays the line number of each line that matches the query

----

How to run:

----

Currently further developing the project compared to the book's example to include:

- option to search multiple files and output file name
- limit search to specific file extension
- search for regex matches
- search for whole word matches
- add a help command that outputs all the commands and their syntax

- highlight matches with colour
- control case sensitivity through either a command line argument or an environment variable - decide which takes precedence