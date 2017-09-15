use super::*;
use super::super::tokens;
use super::extract_values::*;

pub fn interprete_optional_arg(
    mut stream: Box<Iterator<Item=Token>>,
    name: &'static str,
    defined_count: &Count) -> Result<(Box<Iterator<Item=Token>>, usize, Option<ArgValue>), &'static str> {


    Err("")
}
