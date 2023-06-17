pub mod grep;

use crate::grep::GrepArgs;
use std::fs;
use std::io::{self, BufRead, BufReader};
use anyhow::*;


pub fn find_string_in_file(args: &GrepArgs) -> Result<Vec<String>> {
    /*
        Returns a vector of owned String instances (size = # of lines found).
        Time complexity: O(N) in file length due to inevitable traversal.
        Space complexity: O(1) because each line variable is dropped at the end of each iteration.
     */
    let reader = BufReader::new(
        fs::File::open(&args.file_path)?
    );

    let mut results = Vec::new();

    for line_result in reader.lines() {
        // The compiler does not allow returning references to temporary variables
        let line_result = line_result?;
        // new String bound to line
        let line = if !&args.case_sensitive {
            line_result.to_lowercase()
        } else {
            line_result
        };
        if line.contains(&args.query) {
            results.push(line); // line moved into vector, who now owns it
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_found_case_sensitive() -> Result<()> {
        let args = vec!["".to_string(), "Are".to_string(), "poem.txt".to_string(), "true".to_string()];
        let args = GrepArgs::new(&args)?;
        let results = find_string_in_file(&args)?;
        Ok(
            assert_eq!(results.len(), 1)
        )
    }

    #[test]
    fn file_does_not_exist() -> Result<()> {
        let args = vec!["".to_string(), "banish".to_string(), "missing.txt".to_string(), "false".to_string()];
        let args = GrepArgs::new(&args)?;
        Ok(
            assert!(
                find_string_in_file(&args).is_err()
            )
        )
    }

    #[test]
    fn string_not_found() -> Result<()> {
        let args = vec!["".to_string(), "quantum".to_string(), "poem.txt".to_string(), "false".to_string()];
        let args = GrepArgs::new(&args)?;
        let results = find_string_in_file(&args)?;
        Ok(
            assert_eq!(results.len(), 0)
        )
    }

    #[test]
    fn string_found_case_insensitive() -> Result<()> {
        let args = vec!["".to_string(), "Are".to_string(), "poem.txt".to_string(), "false".to_string()];
        let args = GrepArgs::new(&args)?;
        let results = find_string_in_file(&args)?;
        Ok(
            assert_eq!(results.len(), 2)
        )
    }
}
