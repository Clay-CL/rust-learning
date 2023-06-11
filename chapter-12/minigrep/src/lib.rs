use colored::Colorize;
use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub show_line_numbers: bool
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, String> {
        if (args.len() - 1) < 2 {
            let err_msg = format!(
                "Insufficient arguments. Expected 2, received {}",
                args.len() - 1
            );
            return Err(err_msg);
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        let show_line_numbers = env::var("SHOW_LINE_NUMBERS").is_ok();
        Ok(Self {
            query,
            file_path,
            ignore_case,
            show_line_numbers
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(config.file_path).expect("Problem encountered when reading file");
    // println!("Contents :\n{}", contents);
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, contents, true, config.show_line_numbers)
    } else {
        search(&config.query, contents, true, config.show_line_numbers)
    };

    if results.is_empty() {
        println!("\n{}", "No matches found".yellow());
        return Ok(());
    }

    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub fn search(query: &str, contents: String, colored: bool, show_line_numbers: bool) -> Vec<String> {
    let mut results = Vec::new();
    for (index, line) in contents.lines().enumerate() {
        if line.contains(query) {
            // modified the example to highlight the found text
            let updated_string = if colored {
                let changed = line.replace(query, query.green().to_string().as_str());
                changed
            } else {
                line.trim().to_string()
            };
            let updated_string = if show_line_numbers {
                String::from(format!("{} : {}", index, updated_string))
            } else {
                updated_string
            };
            results.push(updated_string)
        }
    }

    results
}

pub fn search_case_insensitive(query: &str, contents: String, colored: bool, show_line_numbers: bool) -> Vec<String> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for (index, line) in contents.lines().enumerate() {
        if line.to_lowercase().contains(&query) {
            let updated_string = if colored {
                let idx = line.to_lowercase().find(&query).unwrap();
                let original: String = line.chars().skip(idx).take(query.len()).collect();
                let original_colored = original.green().to_string();
                let modified = line.replace(original.as_str(), original_colored.as_str());
                modified
            } else {
                line.trim().to_string()
            };
            
            let updated_string = if show_line_numbers {
                String::from(format!("{} : {}", index, updated_string))
            } else {
                updated_string
            };
            results.push(updated_string)
        }
    }

    results
}

// tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = String::from(
            "\
Rust:
safe, fast, productive.
Pick three.",
        );

        let results = vec![format!("safe, fast, pro{}ive.", "duct".green())];
        assert_eq!(results, search(query, contents, /* colored */ true, false));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = String::from(
            "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.",
        );

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, /* colored */ false, false)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = String::from(
            "\
        Rust:
        safe, fast, productive.
        Pick three.
        Trust me.",
        );

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents, /* colored */ false, false)
        );
    }
}
