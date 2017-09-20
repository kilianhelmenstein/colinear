extern crate colinear;

#[test]
fn parsing_one_() {
    let raw_stream = vec![String::from("1"), String::from("2"), String::from("3"), String::from("4")];
    let token_stream = colinear::tokens::tokenize(&raw_stream);


}
