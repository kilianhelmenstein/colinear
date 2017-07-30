#![allow(non_snake_case)]

pub mod args;

#[test]
fn test_tokenize() {
    use args::*;
    use args::tokens::*;

    let argList = vec!(
        String::from("val1"),
        String::from("val2"),
        String::from("-o"),
        String::from("optval1"),
        String::from("--option2"),
        String::from("optval2"));

    let tokenStream = tokens::tokenize(&argList);

    match tokenStream[0] {
        Token::Value(ref val) if *val == String::from("val1") => (),
        _ => panic!("val1 false"),
    }

    match tokenStream[1] {
        Token::Value(ref val) if *val == String::from("val2") => (),
        _ => panic!("val2 false"),
    }

    match tokenStream[2] {
        Token::ShortName(ref name) if *name == String::from("-o") => (),
        _ => panic!("-o false"),
    }

    match tokenStream[3] {
        Token::Value(ref val) if *val == String::from("optval1") => (),
        _ => panic!("optval1 false"),
    }

    match tokenStream[4] {
        Token::LongName(ref name) if *name == String::from("--option2") => (),
        _ => panic!("-o false"),
    }

    match tokenStream[5] {
        Token::Value(ref val) if *val == String::from("optval2") => (),
        _ => panic!("optval2 false"),
    }
}

#[test]
fn test_Arg_takeTokens_index() {
    use args::*;
    use args::tokens::*;

    let argList = vec!(
        String::from("val1"),
        String::from("val2"));

    let tokenStream = tokens::tokenize(&argList);

    let mut argIndex0 = args::Arg::new()
                    .with_name("Index 1")
                    .with_help("Index 1")
                    .on_index(0)
                    .takes_one_value();
    let newIndex = argIndex0.takeTokensAtIndex(&tokenStream, &0);

    if newIndex != 1 {
        panic!("takeTokensAtIndex delivered false resulting index");
    }

    match argIndex0.matchedValues {
        Some(ref matched_values) if matched_values.len() > 0 => {
            if let Some(matched_value) = matched_values.first() {
                if *matched_value == String::from("val1") {
                    println!("Matched right value: {}", matched_value);
                } else {
                    panic!("Matched false value: {}", matched_value)
                }
            } else {
                panic!("Matched invalid value: {}")
            }
        },
        Some(ref matched_values) => panic!("takeTokens matched false value"),
        _ => panic!("takeTokens matched to no value!"),
    }
}

#[test]
fn test_Arg_takeTokens_option() {
    use args::*;
    use args::tokens::*;

    let argList = vec!(
        String::from("val1"),
        String::from("val2"),
        String::from("-o"),
        String::from("optval1"),
        String::from("--option2"),
        String::from("optval2"));

    let tokenStream = tokens::tokenize(&argList);

    let mut argOpt1 = args::Arg::new()
                    .with_name("Opt 1")
                    .with_help("Opt 1")
                    .as_option("-o", "--option")
                    .takes_one_value();
    let newIndex = argOpt1.takeTokensAtIndex(&tokenStream, &2);

    if newIndex != 4 {
        panic!("takeTokensAtIndex delivered false resulting index");
    }

    match argOpt1.matchedValues {
        //Some(ref matched_values) if matched_values.len() > 0 && matched_values[0] == String::from("optval1") => (),
        Some(ref matched_values) if matched_values.len() > 0 => {
            if let Some(matched_value) = matched_values.first() {
                if *matched_value == String::from("optval1") {
                    println!("Matched right value: {}", matched_value);
                } else {
                    panic!("Matched false value: {}", matched_value)
                }
            } else {
                panic!("Matched invalid value: {}")
            }
        },
        Some(ref matched_values) => panic!("takeTokens matched false value"),
        _ => panic!("takeTokens matched to no value!"),
    }
}

#[test]
fn test_Parser_parse() {
    use args::*;
    use args::parser::*;

    let argList = vec!(
        String::from("val1"),
        //String::from("val2"),
        String::from("-o"),
        String::from("optval1"),
        String::from("--option2"),
        String::from("optval2"));
    let tokenStream = tokens::tokenize(&argList);

    let mut parser = Parser::new()
                    .app("Colinear")
                    .with_author("Kilian Helmenstein", "kilian.helmenstein@gmail.com")
                    .with_arg(args::Arg::new()
                                .with_name("Pos 1")
                                .with_help("Pos 1")
                                .on_index(0)
                                .takes_one_value())
                    .with_arg(args::Arg::new()
                                .with_name("Option 1")
                                .with_help("Opt 1")
                                .as_option("-o", "--option")
                                .takes_one_value())
                    .with_arg(args::Arg::new()
                                .with_name("Option 2")
                                .with_help("Opt 2")
                                .as_option("-p", "--option2")
                                .takes_one_value());
    parser.parse_token_stream(&tokenStream);

    if let Some(value1_content) = parser.value("Pos 1") {
        if value1_content != "val1" {
            panic!("Matched, but value is false ({})", "Pos 1");
        }
    } else {
        panic!("Not matched ({})", "Pos 1");
    }

    if let Some(opt1_content) = parser.value("Option 1") {
        if opt1_content != "optval1" {
            panic!("Matched, but value is false ({})", "Option 1");
        }
    } else {
        panic!("Not matched ({})", "Option 1");
    }

    if let Some(opt2_content) = parser.value("Option 2") {
        if opt2_content != "optval2" {
            panic!("Matched, but value is false ({})", "Option 2");
        }
    } else {
        panic!("Not matched ({})", "Option 2");
    }
}
