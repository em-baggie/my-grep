# mygrep
A simple command line search tool completed as part of <a href = "https://doc.rust-lang.org/book/ch12-00-an-io-project.html">The Rust Programming Language</a> book. 

---- 

## Extra features compared to the book's example:

- [x] Counts and displays the number of lines that match the query
- [x] Uses <a href="https://crates.io/crates/boyer-moore-magiclen">Boyer Moore Magiclen</a> as a more efficient search method than linear search
- [x] Displays the line number of each line that matches the query
- [x] Option to display a help command that outputs all the commands and their syntax
- [x] Refactored into modules
- [ ] Control case sensitivity through either a command line argument or an environment variable - decide which takes precedence

----

**How to run:**
1. Clone the repository:
   ```
   git clone https://github.com/em-baggie/mygrep.git
   cd mygrep
   ```
2. Build the application:
   ```
   cargo build --release
   ```
3. Create a plain text file to be searched in the root directory `/mygrep`<br><br>

3. Run the application (from within the root directory `/mygrep`):<br><br>
**Commands:**<br>
   - `mygrep <query> <directory>`
   searches for the query in the directory





----

**Currently further developing the project compared to the book's example to include:**
- control case sensitivity through either a command line argument or an environment variable - decide which takes precedence