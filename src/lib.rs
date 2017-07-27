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
fn test_Arg_takeTokens() {
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
    argOpt1.takeTokensAtIndex(&2, &tokenStream);

    match argOpt1.matchedValues {
        //Some(ref matched_values) if matched_values.len() > 0 && matched_values[0] == String::from("optval1") => (),
        Some(matched_values) => println!("Matched value {}", matched_values[0]),
        _ => panic!("takeTokens did not take tokens!"),
    }
}