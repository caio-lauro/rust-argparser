use argparser::{Argument, ArgumentParser, ArgumentType::*, OptionalArgument, ParsedValue};

fn main() {
    let parser = ArgumentParser::new()
        .add_arg(Argument::from("input", Text))
        .add_arg(Argument::from("output", Text))
        .add_arg(OptionalArgument::from(
            "verbose",
            Some("v"),
            Boolean,
            ParsedValue::Boolean(false),
        ))
        .add_arg(OptionalArgument::from(
            "count",
            Some("c"),
            Integer,
            ParsedValue::Integer(1),
        ));

    let parsed = parser.parse(std::env::args()).unwrap_or_else(|e| {
        eprintln!("error: {e}");
        std::process::exit(1);
    });

    let input = parsed.get_as::<String>("input");
    let output = parsed.get_as::<String>("output");
    let verbose = parsed.get_as::<bool>("verbose");
    let count = parsed.get_as::<i64>("count");

    if verbose {
        println!("input:  {input}");
        println!("output: {output}");
        println!("count:  {count}");
    }
}