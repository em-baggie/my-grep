# mygrep
A simple command-line search tool which builds upon the project outlined in <a href = "https://doc.rust-lang.org/book/ch12-00-an-io-project.html">Chapter 12 of The Rust Programming Language</a> book. 

## Extra features compared to the book's example:

- [x] Counts and displays the number of lines that match the query
- [x] Uses <a href="https://crates.io/crates/boyer-moore-magiclen">Boyer Moore Magiclen</a> as a more efficient search method than linear search
- [x] Displays the line number of each line that matches the query
- [x] Option to display a help command that outputs all the commands and their syntax
- [x] Refactored into modules
- [x] Control case sensitivity through either a command line argument or an environment variable, with the environment variable taking precedence

## How to run:
1. **Clone the repository:**
   ```
   git clone https://github.com/em-baggie/mygrep.git
   cd mygrep
   ```
2. **Build the application:**
   ```
   cargo build --release
   ```
3. **Create a plain text file** to be searched in the root directory `/mygrep`. `poem.txt` is an example file. <br><br>

4. **Run the application** (from within the root directory `/mygrep`):<br><br>
    To search for a query in a given file path:
    ```
    cargo run -- <query> <file_path>
    ```
    To display all commands and their syntax:
    ```
    cargo run -- helpme
    ```

    **Controlling case sensitivity**<br>
    The default is case-sensitive search. To enable case-insensitive search, either set the environmental variable or use the command line argument '-i' as described below. Note the environmental variable takes precendence.
    
    1. Using the environment variable:<br>
        Set the `IGNORE_CASE` environment variable to `1` before running the application:
        ```
        export IGNORE_CASE=1
        ```
        To subsequently disable case insensitive search, unset the `IGNORE_CASE` environment variable:
        ```
        unset IGNORE_CASE
        ```
        Alternatively, run the application with the preceding environment variable:
        ```
        IGNORE_CASE=1 cargo run -- <query> <file_path>
        ```
    2. Using the command line argument:<br>
        ```
        cargo run -- <query> <file_path> -i
        ```
