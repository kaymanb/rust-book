use std::error::Error;
use std::fs;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {

    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
                       
        Ok(Config { query, filename, case_sensitive})
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    
    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in result { 
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_new() {
        let args = [String::from("minigrep"), 
                    String::from("hello"), 
                    String::from("world.txt")
        ];
        let config = Config::new(&args).unwrap();
        assert_eq!(config.query, "hello");
        assert_eq!(config.filename, "world.txt");
    }

    #[test]
    fn config_not_enough_args() {
        let args = [String::from("minigrep")];
        let e = Config::new(&args).unwrap_err();
        assert_eq!(e, "not enough arguments");
    }

    #[test]
    fn search_one_result() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    
    #[test]
    fn search_is_case_sensitive() {
        let query = "duct";
        let contents = "DUCT:\nsafe, fast, productive.\nPick three";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }


    #[test]
    fn search_case_insensitive_works() {
        let query = "lower";
        let contents = "LOWER HERE\nUPPER HERE\nlower again";
        assert_eq!(vec!["LOWER HERE", "lower again"], 
                   search_case_insensitive(query, contents));
    }
}   
