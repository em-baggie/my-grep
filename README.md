# **my-grep**
A simple command line search tool completed as part of <a href = "https://doc.rust-lang.org/book/ch12-00-an-io-project.html">The Rust Programming Language</a> book. 

---- 

Extra features compared to the book's example:
- counts and displays the number of lines that match the query

----

Currently further developing the project compared to the book's example to include:
- control case sensitivity through either a command line argument or an environment variable - decide which takes precedence
- output the line number of each line that matches the query
- option to search multiple files and output file name
- limit search to specific file extension
- search for regex matches
- search for whole word matches
- highlight matches with colour
- more efficient search method, e.g. recursive search
- add a help command that outputs all the commands and their syntax