use std::io::{self, Result, Error, ErrorKind};
//use std::error::Error;
/*
    Box<dyn Error> means a function will return a type that implements the Error trait,
    but we don’t have to specify what particular type the return value will be.
    This gives us flexibility to return error values that may be of different
    types in different error cases. The dyn keyword is short for “dynamic.”
 */

pub struct GrepArgs {
    pub query: String,
    pub file_path: String
}
impl GrepArgs {
    pub fn new(args: &Vec<String>) -> Result<GrepArgs> {
        match args.get(1) {
            Some(query) => match args.get(2) {
                Some(file_path) => Ok(
                    GrepArgs {
                        query: query.to_lowercase(),
                        file_path: file_path.to_owned()
                    }
                ),
                _ => Err(Error::new(ErrorKind::NotFound, "Please provide the file to be searched as the second argument."))
            },
            _ => Err(Error::new(ErrorKind::NotFound, "Please provide the string you are looking for as the first argument."))
        }
    }

    pub fn show(&self) {
        println!("Searching for \"{}\" in file \"{}\"...", self.query, self.file_path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arguments_provided() -> Result<()> {
        let args = vec!["".to_string(), "you".to_string(), "poem.txt".to_string()];
        Ok(
            assert!(
                GrepArgs::new(&args).is_ok()
            )
        )
    }

    #[test]
    fn missing_first_argument() -> Result<()> {
        let args = vec!["".to_string()];
        Ok(
            assert!(
                GrepArgs::new(&args).is_err()
            )
        )
    }

    #[test]
    fn missing_second_argument() -> Result<()> {
        let args = vec!["".to_string(), "you".to_string()];
        Ok(
            assert!(
                GrepArgs::new(&args).is_err()
            )
        )
    }
}
