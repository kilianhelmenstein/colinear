use super::*;
use super::extract_values::*;

pub fn interprete_optional_arg<'a>(
    name: &'static str,
    defined_count: &Count,
    defined_short_name: &String,
    defined_long_name: &String,
    stream: &'a [Token]) -> Result<(&'a [Token], Option<ArgValue>), &'static str> {

    let names_matches = match stream[0] {
        Token::ShortName(ref short_name) => short_name == defined_short_name,
        Token::LongName(ref long_name) => long_name == defined_long_name,
        Token::Value(_) => false,
    };

    if !names_matches {
        return Ok((stream, None));
    };

    let (min, max) = match defined_count {
        &Count::Fixed(fixed_count) => (fixed_count, fixed_count),
        &Count::Minimum(min_count) => (min_count, 100 as usize),
        &Count::Maximum(max_count) => (0 as usize, max_count),
        &Count::Range { min, max } => (min, max),
    };

    let (stream, values) = n_following_values(&stream[1..], min, max)?;
    Ok((stream, Some(ArgValue::new(name, 1, values))))
}

#[cfg(test)]
mod test {
    use super::super::super::tokens;
    use super::super::super::tokens::*;
    use super::super::*;

    #[test]
    fn interprete_optional_arg__shortname_and_two_values__captures_two_values() {
        let token_stream = tokens::tokenize(&vec![String::from("-o"), String::from("1"), String::from("2"), String::from("3")]);

        let (stream, maybe_value) = super::interprete_optional_arg("opional_arg", &Count::Fixed(2), &String::from("-o"), &String::from("--optional"), &token_stream).unwrap();
        let arg_value = maybe_value.unwrap();

        assert!(arg_value.occurences == 1);
        assert!(arg_value.name == String::from("opional_arg"));
        assert!(arg_value.assigned_values == vec![String::from("1"), String::from("2")]);
    }

    #[test]
    fn interprete_optional_arg__longname_and_two_values__captures_two_values() {
        let token_stream = tokens::tokenize(&vec![String::from("--optional"), String::from("1"), String::from("2"), String::from("3")]);

        let (stream, maybe_value) = super::interprete_optional_arg("opional_arg", &Count::Fixed(2), &String::from("-o"), &String::from("--optional"), &token_stream).unwrap();
        let arg_value = maybe_value.unwrap();

        assert!(arg_value.occurences == 1);
        assert!(arg_value.name == String::from("opional_arg"));
        assert!(arg_value.assigned_values == vec![String::from("1"), String::from("2")]);
    }
}
