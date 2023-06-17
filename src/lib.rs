pub mod grep;

use crate::grep::GrepArgs;
use std::fs;
use std::io::{self, BufRead, BufReader, Result};


pub fn find_string_in_file(args: &GrepArgs) -> Result<Vec<String>> {
    /*
        Returns a vector of owned String instances (size = # of lines found).
        Time complexity: O(N) in file length due to inevitable traversal.
        Space complexity: O(1) because each line variable is dropped at the end of each iteration.
     */
    let reader = BufReader::new(
        fs::File::open(&args.file_path)?
    );

    let mut vector = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?; // moved into line, not cloned
        if line.contains(&args.query) {
            vector.push(line); // line moved into vector, who now owns it
        }
    }

    Ok(vector)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_found() -> Result<()> {
        let args = vec!["".to_string(), "you".to_string(), "poem.txt".to_string()];
        let args = GrepArgs::new(&args)?;
        let vector = find_string_in_file(&args)?;
        Ok(
            assert_eq!(vector.len(), 4)
        )
    }

    #[test]
    fn file_does_not_exist() -> Result<()> {
        let args = vec!["".to_string(), "banish".to_string(), "missing.txt".to_string()];
        let args = GrepArgs::new(&args)?;
        Ok(
            assert!(
                find_string_in_file(&args).is_err()
            )
        )
    }

    #[test]
    fn string_not_found() -> Result<()> {
        let args = vec!["".to_string(), "quantum".to_string(), "poem.txt".to_string()];
        let args = GrepArgs::new(&args)?;
        let vector = find_string_in_file(&args)?;
        Ok(
            assert_eq!(vector.len(), 0)
        )
    }
}
