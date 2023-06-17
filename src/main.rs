// cargo run -- searchstring example-filename.txt
use std::env;
use grep_cli::grep::GrepArgs;
/*
    Note that std::env::args will panic if any argument contains invalid Unicode.
    If your program needs to accept arguments containing invalid Unicode,
    use std::env::args_os instead. That function returns an iterator that produces OsString values
    instead of String values. We’ve chosen to use std::env::args here for simplicity, because
    OsString values differ per platform and are more complex to work with than String values.
 */
/*
    use std::process;
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    If the value is an Err value, this method calls the code in the closure,
    which is an anonymous function we define and pass as an argument to unwrap_or_else.
 */

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
    let args = GrepArgs::new(&args)?;
    args.show();
    let vector = grep_cli::find_string_in_file(&args)?;
    println!("Found in {} lines.", vector.len());
    Ok(())
}
