pub mod grep;

use crate::grep::GrepArgs;
use std::fs;
use std::io::{self, BufRead, BufReader, Result};


pub fn find_string_in_file(args: &GrepArgs) -> Result<u32> {
    /*
        Returns the number of lines where the string is found while printing them.
        Time complexity: O(N) in file length due to inevitable traversal.
        Space complexity: O(1) because each line variable is dropped at the end of each iteration.
     */
    args.show();

    let reader = BufReader::new(
        fs::File::open(&args.file_path)?
    );

    let mut counter: u32 = 0;

    for line_result in reader.lines() {
        let line = &line_result?;
        if line.contains(&args.query) {
            counter += 1;
            println!("{}", line);
        }
    }

    Ok(counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_found() -> Result<()> {
        let args = vec!["".to_string(), "you".to_string(), "poem.txt".to_string()];
        let args = GrepArgs::new(&args)?;
        let count = find_string_in_file(&args)?;
        Ok(
            assert_eq!(count, 4)
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
        let count = find_string_in_file(&args)?;
        Ok(
            assert_eq!(count, 0)
        )
    }
}
