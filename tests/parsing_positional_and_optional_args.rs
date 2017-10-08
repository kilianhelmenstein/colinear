extern crate colinear;

use colinear::args::*;
use colinear::args::builder::*;
use colinear::tokens;
use colinear::parsing::*;

#[test]
fn parsing_one_() {
    let mut args = Vec::new();
    args.push(an_arg_called("debug-output")
        .with_count(Count::Fixed(0))
        .as_option("-d", "--debug")
        .assembled());
    args.push(an_arg_called("first-value")
        .with_count(Count::Fixed(1))
        .on_index(0)
        .assembled());

    let argument_string = vec![String::from("-d"), String::from("-d"), String::from("-d"), String::from("4")];
    let token_stream = tokens::tokenize(&argument_string);

    let parsed_args = parse(&token_stream, args).unwrap();

    print!("Result's length: {}\n", parsed_args.len());
    assert!(parsed_args.len() == 1);

}
