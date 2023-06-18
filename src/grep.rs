use std::env;
use anyhow::*;
//use std::error::Error;
/*
    Box<dyn Error> means a function will return a type that implements the Error trait,
    but we don’t have to specify what particular type the return value will be.
    This gives us flexibility to return error values that may be of different
    types in different error cases. The dyn keyword is short for “dynamic.”
 */

pub struct GrepArgs {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}
impl GrepArgs {
    pub fn build(args: &Vec<String>) -> Result<GrepArgs> {
        match args.get(1) {
            Some(query) => match args.get(2) {
                Some(file_path) => {
                    let ignore_case = env::var("IGNORE_CASE").is_ok();
                    Ok(
                        /*
                        While to_lowercase will handle basic Unicode, it won’t be 100% accurate.
                        If we were writing a real application, we’d want to do a bit more work here,
                        but this is about environment variables, not Unicode, so we’ll leave it for now.
                     */
                        GrepArgs {
                            query: if ignore_case { query.to_lowercase() } else { query.to_owned() },
                            file_path: file_path.to_owned(),
                            ignore_case
                        }
                    )
                },
                _ => Err(anyhow!("Please provide the file to be searched as the second argument."))
            },
            _ => Err(anyhow!("Please provide the text you are looking for as the first argument."))
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
                GrepArgs::build(&args).is_ok()
            )
        )
    }

    #[test]
    fn missing_first_argument() -> Result<()> {
        let args = vec!["".to_string()];
        Ok(
            assert!(
                GrepArgs::build(&args).is_err()
            )
        )
    }

    #[test]
    fn missing_second_argument() -> Result<()> {
        let args = vec!["".to_string(), "you".to_string()];
        Ok(
            assert!(
                GrepArgs::build(&args).is_err()
            )
        )
    }
}
