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


fn main() -> Result<(), io::Error> {
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
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for \"{query}\" in file \"{file_path}\"...");

    /*
        Time complexity: O(N) in file length due to inevitable traversal.
        Space complexity: Is it possible to avoid reading the file into a String and simply use pointers to disk?
            Not sure because this is a disk I/O operation that needs to operate on RAM data?
     */

    let file = fs::File::open(file_path)?;
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = &line_result?;
        if line.contains(query.as_str()) {
            println!("{}", line);
        }
    }
    
    // let lines = fs::read_to_string(&file_path)?;
    // for sentence in lines.split("\n") {
    //     if sentence.contains(query.as_str()) {
    //         println!("{sentence}");
    //     }
    // }

    Ok(())
}
