use std::fs;
use std::io::{self, BufRead, BufReader, Error, ErrorKind, Result};


pub struct GrepArgs<'a> {
    query: &'a str,
    file_path: &'a str
}
impl<'a> GrepArgs<'a> {
    pub fn new(args: &'a [String]) -> Result<GrepArgs> {
        match args.get(1) {
            Some(query) => match args.get(2) {
                Some(file_path) => Ok(
                    GrepArgs {
                        query,
                        file_path
                    }
                ),
                _ => Err(Error::new(ErrorKind::NotFound,"Please provide the file to be searched as the second argument."))
            },
            _ => Err(Error::new(ErrorKind::NotFound, "Please provide the string you are looking for as the first argument."))
        }
    }

    fn show(&self) {
        println!("Searching for \"{}\" in file \"{}\"...", self.query, self.file_path);
    }

    pub fn find_string_in_file(&self) -> Result<u32> {
        /*
            Returns the number of lines where the string is found while printing them.
            Time complexity: O(N) in file length due to inevitable traversal.
            Space complexity: O(1) because each line variable is dropped at the end of each iteration.
         */
        self.show();

        let reader = BufReader::new(
            fs::File::open(&self.file_path)?
        );

        let mut counter: u32 = 0;

        for line_result in reader.lines() {
            let line = &line_result?;
            if line.contains(&self.query) {
                counter += 1;
                println!("{}", line);
            }
        }

        Ok(counter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_found() -> Result<()> {
        let args = vec!["".to_string(), "you".to_string(), "poem.txt".to_string()];
        let count = GrepArgs::new(&args)?
            .find_string_in_file()?;
        Ok(
            assert_eq!(count, 4)
        )
    }

    #[test]
    fn file_does_not_exist() -> Result<()> {
        let args = vec!["".to_string(), "banish".to_string(), "missing.txt".to_string()];
        Ok(
            assert!(
                GrepArgs::new(&args)?
                    .find_string_in_file()
                    .is_err()
            )
        )
    }

    #[test]
    fn string_not_found() -> Result<()> {
        let args = vec!["".to_string(), "quantum".to_string(), "poem.txt".to_string()];
        let count = GrepArgs::new(&args)?
            .find_string_in_file()?;
        Ok(
            assert_eq!(count, 0)
        )
    }
}