use std::{env, error::Error, fs};

// IGNORE_CASE=1 cargo run -- to poem.txt

pub struct Config {
  pub query: String,
  pub file_path: String,
  pub ignore_case: bool,
}

// Реализация без использования итераторов
// impl Config {
//     pub fn build(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }
//         let query = args[1].clone();
//         let file_path = args[2]. clone();

//         let ignore_case = env::var("IGNORE_CASE").is_ok();

//         Ok(Config { query, file_path, ignore_case })
//     }
// }

// Реализация с итераторами
impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next(); // Первая запись в env::args() - имя программы

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contexts = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contexts)
    } else {
        search(&config.query, &contexts)
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let context = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, context));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}

// Реализация без итераторов
// pub fn search<'a>(query: &str, contexts: &'a str) -> Vec<&'a str> {
//     let mut results = Vec::new();
//     for line in contexts.lines() {
//         if line.contains(query) {
//             results.push(line);
//         }
//     }
//     results
// }

// Реализация на итераторах
pub fn search<'a>(query: &str, contexts: &'a str) -> Vec<&'a str> {
    contexts
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();

    // Реализация без итераторов
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }
    // results

    // Реализация с итераторами
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
