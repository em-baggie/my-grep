use std:: {env, process};
use minigrep::utils::config::Config;
use minigrep::utils::run;

fn main() {

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // better than unwrap_or_else as do not need to access the Ok value
    if let Err(e) = run::run_program(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}