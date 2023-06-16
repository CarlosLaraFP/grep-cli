use std::fs;
use std::io::{self, BufRead, BufReader};


pub struct GrepArgs<'a> {
    query: &'a str,
    file_path: &'a str
}
impl<'a> GrepArgs<'a> {
    pub fn new(query: &'a str, file_path: &'a str) -> Self {
        GrepArgs {
            query,
            file_path
        }
    }

    pub fn show(&self) {
        println!("Searching for \"{}\" in file \"{}\"...", self.query, self.file_path);
    }

    pub fn find_string_in_file(&self) -> Result<u32, io::Error> {
        /*
            Returns the number of lines where the string is found while printing them.
            Time complexity: O(N) in file length due to inevitable traversal.
            Space complexity: O(1) because each line variable is dropped at the end of each iteration.
         */
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
    #[test]
    fn string_found() {
        match super::GrepArgs::new("banish", "poem.txt")
            .find_string_in_file() {
                Ok(count) => assert_eq!(count, 1),
                Err(error) => panic!("{}", error)
            }
    }

    #[test]
    fn file_does_not_exist() {
        assert!(
            super::GrepArgs::new("banish", "missing.txt")
                .find_string_in_file()
                .is_err()
        )
    }

    #[test]
    fn string_not_found() {
        match super::GrepArgs::new("quantum", "poem.txt")
            .find_string_in_file() {
            Ok(count) => assert_eq!(count, 0),
            Err(error) => panic!("{}", error)
        }
    }
}