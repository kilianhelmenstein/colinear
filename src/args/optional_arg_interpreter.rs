use super::*;
use super::extract_values::*;

pub fn interprete_optional_arg<'a>(
    stream: &'a [Token],
    name: &'static str,
    defined_count: &Count) -> Result<(&'a [Token], Option<ArgValue>), &'static str> {

    match stream[0] {
        Token::ShortName(ref short_name) => Err(""),
        Token::LongName(ref long_name) => Err(""),
        Token::Value(_) => Ok((stream, None)),
    }
}
