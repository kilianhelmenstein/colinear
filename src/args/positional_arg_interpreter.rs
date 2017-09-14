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

    let (min, max) = match defined_count {
        &Count::Fixed(fixed_count) => (fixed_count, fixed_count),
        &Count::Minimum(min_count) => (min_count, 100 as usize),
        &Count::Maximum(max_count) => (0 as usize, max_count),
        &Count::Range { min, max } => (min, max),
    };

    let (stream, values) = n_following_values(stream, &min, &max)?;
    Ok((stream, actual_logical_index+1, Some(ArgValue::new(name, 1, values))))
}

fn n_following_values(mut stream: Box<Iterator<Item=Token>>, min: &usize, max: &usize) -> Result<(Box<Iterator<Item=Token>>, Vec<String>), &'static str> {
    append_n_following_values(stream, Vec::new(), min, max)
}

fn append_n_following_values(
    mut stream: Box<Iterator<Item=Token>>,
    mut appended: Vec<String>,
    min: &usize, max: &usize) -> Result<(Box<Iterator<Item=Token>>, Vec<String>), &'static str> {

    let got_max_number_of_values = *max == 0;
    if got_max_number_of_values {
        return Ok((stream, appended));
    }

    if let Some(Token::Value(next_value)) = stream.next() {
        appended.push(next_value);
        append_n_following_values(stream, appended, &(*min-1), &(*max-1))
    } else {
        let got_min_number_of_values = *min == 0;
        if got_min_number_of_values {
            Ok((stream, appended))
        } else {
            Err("No value left")
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::super::tokens;
    use super::super::super::tokens::*;
    use super::super::*;

    #[test]
    fn interprete_positional_arg__test() {
        let argument_string = vec![String::from("1"), String::from("2"), String::from("3"), String::from("4")];
        let token_stream = tokens::tokenize(&argument_string);

        let stream_iterator = Box::new(token_stream.into_iter());

        let (stream, logical_index, maybe_value) = super::interprete_positional_arg(stream_iterator, 0, "first", &Count::Fixed(2), 0).unwrap();
        let arg_value = maybe_value.unwrap();

        assert!(arg_value.occurences == 1);
        assert!(arg_value.name == String::from("first"));
        assert!(arg_value.assigned_values == vec![String::from("1"), String::from("2")]);
    }
}
