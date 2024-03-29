use std::io::Read;
use std::error::Error;
use std::fs::File;
use std::env;

pub struct Config {
    pub query: String,
    pub file_name: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };
        let file_name = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path")
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Self { query, file_name , case_sensitive})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(&config.file_name)?;
    
    let mut buffer: String = String::new();
    f.read_to_string(&mut buffer)?;

    let results  =  if config.case_sensitive {
        search(&config.query, &buffer)
    } else {
        search_case_insensitive(&config.query, &buffer)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //        results.push(line);
    //     }
    // }
    // results
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let query = query.to_lowercase();
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line)
    //     }
    // }
    // results
    contents
        .lines()
        .filter(|line|  {
            line.to_lowercase().contains(&query)
        })
        .collect()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nPick three\nTrust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}