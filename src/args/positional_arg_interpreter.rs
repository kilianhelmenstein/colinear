use super::*;
use super::super::tokens;

pub fn interprete_positional_arg(
    mut stream: Box<Iterator<Item=Token>>,
    actual_logical_index: usize,
    name: &'static str,
    defined_count: &Count,
    defined_logical_index: usize) -> Result<(Box<Iterator<Item=Token>>, usize, Option<ArgValue>), &'static str> {

    let unfitting_logical_index = actual_logical_index != defined_logical_index;
    if unfitting_logical_index {
        return Ok((stream, actual_logical_index, None));
    }

    let arg_values = match defined_count {
        Fixed(fixed_count) =>,
        Minimum(min_count) =>,
        Maximum(max_count) =>,
        Range { min: min_count, max: max_count } =>,
    }

    Err("asfd")
}

fn n_following_values(mut stream: Box<Iterator<Item=Token>>, n: usize) -> Result<(Box<Iterator<Item=Token>>, Vec<String>), &'static str> {
    let (stream, values, _) = append_n_following_values(stream, Vec::new(), n)?;
    Ok((stream, values))
}

fn append_n_following_values(
    mut stream: Box<Iterator<Item=Token>>,
    mut appended: Vec<String>,
    n: usize) -> Result<(Box<Iterator<Item=Token>>, Vec<String>, usize), &'static str> {

    if n <= 0 {
        return Ok((stream, appended, 0))
    }

    if let Some(Token::Value(next_value)) = stream.next() {
        appended.push(next_value);
        append_n_following_values(stream, appended, n-1)
    } else {
        Err("No value left")
    }
}
