use std::error::Error;
use std::fs;

pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
}

impl Config<'_> {
    // Rename `new` to `build` as it's expected `new` will never fail
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{}", contents);

    Ok(())
}
