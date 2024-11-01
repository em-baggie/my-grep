use std::env;

fn main() {
    // below allows pogram to read any CL arguments passed to it and collects them into a vector
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}

