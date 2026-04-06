use argparser::{
    Argument, ArgumentParser, ArgumentType::*, OptionalArgument, ParseError, ParsedValue,
};

#[test]
fn full_usage() -> Result<(), Box<dyn std::error::Error>> {
    let parser = ArgumentParser::new()
        .add_arg(Argument::from("input", Text))
        .add_arg(Argument::from("output", Text))
        .add_arg(OptionalArgument::from(
            "verbose",
            None,
            Boolean,
            ParsedValue::Boolean(false),
        ))
        .add_arg(Argument::from("port", Integer))
        .add_arg(OptionalArgument::from(
            "count",
            Some("c"),
            Integer,
            ParsedValue::Integer(1),
        ));
    let args = &[
        "program",
        "input.txt",
        "--verbose",
        "output.txt",
        "8080",
        "-c",
        "42",
    ];
    let parsed = parser.parse(args.map(|s| s.to_string()))?;

    assert_eq!(
        parsed.get("input"),
        &ParsedValue::Text("input.txt".to_string())
    );
    assert_eq!(
        parsed.get_value::<String>("output"),
        String::from("output.txt")
    );
    assert_eq!(parsed.get_value::<i64>("port"), 8080);
    assert_eq!(parsed.get_value::<bool>("verbose"), true);
    assert_eq!(parsed.get_value::<i64>("count"), 42);

    Ok(())
}

#[test]
fn parse_error_is_std_error() {
    let err: Box<dyn std::error::Error> = Box::new(ParseError::TooManyArguments);
    assert!(!err.to_string().is_empty());
}

#[test]
fn error_display_missing_required() {
    let name = "filename";
    let err = ParseError::MissingRequired(name.to_string());
    assert_eq!(
        err.to_string(),
        format!("missing required argument: {name}")
    );
}

#[test]
fn error_display_missing_value() {
    let name = "port";
    let err = ParseError::MissingValue(name.to_string());
    assert_eq!(
        err.to_string(),
        format!("argument '{name}' requires a value but none was given")
    );
}

#[test]
fn error_display_wrong_type() {
    let name = "port";
    let expected = Integer;
    let given = "not_integer";

    let err = ParseError::WrongType {
        name: name.to_string(),
        expected,
        given: given.to_string(),
    };
    assert_eq!(
        err.to_string(),
        format!("argument '{name}' expects value of type {expected:?} given: {given}")
    );
}

#[test]
fn error_display_unknown_argument() {
    let name = "filename";
    let err = ParseError::UnknownArgument(name.to_string());
    assert_eq!(err.to_string(), format!("unknown argument: {name}"));
}

#[test]
fn error_display_duplicate_argument() {
    let name = "port";
    let err = ParseError::DuplicateArgument(name.to_string());
    assert_eq!(
        err.to_string(),
        format!("argument '{name}' was given more than once")
    );
}

#[test]
fn error_display_too_many_arguments() {
    let err = ParseError::TooManyArguments;
    assert_eq!(err.to_string(), "too many positional arguments");
}
