use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file.");

    println!("With text:\n{}", contents);
}

struct Config<'a> {
    query: &'a str,
    file_path: &'a str,
}

fn parse_config(args: &[String]) -> Config {
    // Use &str and lifetimes for better performance than `.clone()`
    let query = args[1].as_str();
    let file_path = args[2].as_str();
    Config { query, file_path }
}
