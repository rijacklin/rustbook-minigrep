use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    run(config);
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file.");

    println!("With text:\n{}", contents);
}

struct Config<'a> {
    query: &'a str,
    file_path: &'a str,
}

impl Config<'_> {
    // Rename `new` to `build` as it's expected `new` will never fail
    fn build(args: &[String]) -> Result<Config, &'static str> {
        // Basic error handling
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // Use &str and lifetimes for better performance than `.clone()`
        let query = args[1].as_str();
        let file_path = args[2].as_str();
        Ok(Config { query, file_path })
    }
}
