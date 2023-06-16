// cargo run -- searchstring example-filename.txt
use std::env;
/*
    Note that std::env::args will panic if any argument contains invalid Unicode.
    If your program needs to accept arguments containing invalid Unicode,
    use std::env::args_os instead. That function returns an iterator that produces OsString values
    instead of String values. We’ve chosen to use std::env::args here for simplicity, because
    OsString values differ per platform and are more complex to work with than String values.
 */
use std::fs;
use std::io::{self, BufRead, BufReader};
use anyhow::*;


fn main() -> anyhow::Result<()> {
    /*
        1. The args function returns an iterator of the command line arguments passed.
        2. Call the collect method on an iterator to turn it into a collection.
        We can use the collect function to create many kinds of collections, so we explicitly
        annotate the type of args to specify that we want a vector of strings. Although we very
        rarely need to annotate types in Rust, collect is one function you do often need to
        annotate because Rust isn’t able to infer the kind of collection you want.
     */
    let args: Vec<String> = env::args().collect();
    //dbg!(&args);
    let args = GrepArgs::new(&args);
    args.show();
    args.find_string_in_file()
}

struct GrepArgs<'a> {
    query: &'a String,
    file_path: &'a String
}
impl<'a> GrepArgs<'a> {
    pub fn new(args: &'a [String]) -> Self {
        GrepArgs {
            query: &args[1],
            file_path: &args[2],
        }
    }

    pub fn show(&self) {
        println!("Searching for \"{}\" in file \"{}\"...", self.query, self.file_path);
    }

    pub fn find_string_in_file(&self) -> anyhow::Result<()> {
        /*
            Time complexity: O(N) in file length due to inevitable traversal.
            Space complexity: O(1) because each line variable is dropped at the end of each iteration.
         */
        let reader = BufReader::new(
            fs::File::open(self.file_path)?
        );

        for line_result in reader.lines() {
            let line = &line_result?;
            if line.contains(self.query) {
                println!("{}", line);
            }
        }

        Ok(())
    }
}
