use argtiny::{ArgumentParser, arg};

fn main() {
    let parser = ArgumentParser::new()
        .add_arg(arg!(required: "input", Text))
        .add_arg(arg!(required: "output", Text))
        .add_arg(arg!(optional: "verbose", Boolean = false))
        .add_arg(arg!(optional: "count", "c", Integer = 1));

    let parsed = parser.parse(std::env::args()).unwrap_or_else(|e| {
        eprintln!("error: {e}");
        std::process::exit(1);
    });

    let input: String = parsed.get_as("input");
    let output: String = parsed.get_as("output");
    let verbose: bool = parsed.get_as("verbose");
    let count: i64 = parsed.get_as("count");

    if verbose {
        println!("input:  {input}");
        println!("output: {output}");
        println!("count:  {count}");
    }
}
