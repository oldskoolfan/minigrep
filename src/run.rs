use std::{fs, error::Error};

use crate::config::Config;
use crate::search::{search, search_case_insensitive};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read file
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_file_successfully() {
        // arrange
        let config = Config {
            query: String::from("foo"),
            file_path: String::from("poem.txt"),
            ignore_case: true,
        };
        // act, assert
        assert_eq!(run(config).unwrap(), ());
    }

    #[test]
    fn returns_error_when_file_not_found() {
        // arrange
        let config = Config {
            query: String::from("foo"),
            file_path: String::from("bar.txt"),
            ignore_case: true,
        };
        // act, assert
        assert!(run(config).is_err());
    }
}
