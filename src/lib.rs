use std::error::Error;
use std::fs;
use std::env;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let q = "fufufu";
        let contents = "\
        hullo tuturu:
This is a test, fufufu
The best test, mumumu";

        assert_eq!(vec!["This is a test, fufufu"], search(q, contents));
    }

    #[test]
    fn case_insensitive() {
        let q = "fUFu";
        let contents = "\
        hullo tuturu:
This is a test, fufufu
The best test, mumumu
Ufufufu";

        assert_eq!(vec!["This is a test, fufufu", "Ufufufu"],
                   search_case_insensitive(q, contents));
    }

}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let q = match args.next() {
            Some(arg) => arg,
            None => return Err("No query string!")
        };

        let fname = match args.next() {
            Some(arg) => arg,
            None => return Err("No filename!")
        };

        let case_sens = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {query: q, filename: fname, case_sensitive: case_sens})
    }
}

pub fn run(conf: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(conf.filename)?;
    
    let res = if conf.case_sensitive {
        search(&conf.query, &contents)
    } else {
        search_case_insensitive(&conf.query, &contents)
    };

    for line in  res {
        println!("Found '{}' in this line: {}", conf.query, line);
    };
    Ok(())
   
}

pub fn search<'a>(q: &str, cur: &'a str) -> Vec<&'a str> {
    cur
        .lines()
        .filter(|line| line.contains(q))
        .collect()
}

pub fn search_case_insensitive<'a>(q: &str, cur: &'a str) -> Vec<&'a str> {
    let q = q.to_lowercase();
    
    cur
        .lines()
        .filter(|line| line.to_lowercase().contains(&q))
        .collect()
}
