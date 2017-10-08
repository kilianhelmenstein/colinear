use super::super::tokens::Token;

pub fn n_following_values<'a>(stream: &'a [Token], min: usize, max: usize) -> Result<(&'a [Token], Vec<String>), &'static str> {
    append_n_following_values(stream, Vec::new(), min as isize, max as isize)
}

pub fn append_n_following_values<'a>(
    stream: &'a [Token],
    mut appended: Vec<String>,
    min: isize, max: isize) -> Result<(&'a [Token], Vec<String>), &'static str> {

    if min > max {
        return Err("Minimum specification is higher than maximum")
    }

    let max_number_of_values_reached = max <= 0;
    if max_number_of_values_reached {
        return Ok((stream, appended));
    }

    if stream.len() > 0 {
        if let Token::Value(ref token_content) = stream[0] {
            appended.push(token_content.clone());
            append_n_following_values(&stream[1..], appended, min-1, max-1)
        } else {
            Ok((stream, appended))
        }
    } else {
        let min_number_of_values_reached = min <= 0;
        if min_number_of_values_reached {
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
    fn append_n_following_values__extract_exact_one_value_extracts_one_value() {
        let argument_string = vec![String::from("1"), String::from("2"), String::from("3"), String::from("4")];
        let token_stream = tokens::tokenize(&argument_string);

        let (left_stream, extracted_values) = super::append_n_following_values(&token_stream, Vec::new(), 1, 1).unwrap();

        assert!(left_stream.len() == 3);
        assert!(extracted_values.len() == 1);
        assert!(*extracted_values.first().unwrap() == String::from("1"));
    }

    #[test]
    fn append_n_following_values__extract_max_two_value__extracts_two_values() {
        let argument_string = vec![String::from("1"), String::from("2"), String::from("3"), String::from("4")];
        let token_stream = tokens::tokenize(&argument_string);

        let (left_stream, extracted_values) = super::append_n_following_values(&token_stream, Vec::new(), 0, 2).unwrap();

        assert!(left_stream.len() == 2);
        assert!(extracted_values.len() == 2);
        assert!(extracted_values[0] == String::from("1"));
        assert!(extracted_values[1] == String::from("2"));
    }

    #[test]
    fn append_n_following_values__extract_min_two_value__extracts_all_available_values() {
        let argument_string = vec![String::from("1"), String::from("2"), String::from("3"), String::from("4")];
        let token_stream = tokens::tokenize(&argument_string);

        let (left_stream, extracted_values) = super::append_n_following_values(&token_stream, Vec::new(), 2, 10).unwrap();

        assert!(left_stream.len() == 0);
        assert!(extracted_values.len() == 4);
        assert!(extracted_values[0] == String::from("1"));
        assert!(extracted_values[1] == String::from("2"));
        assert!(extracted_values[2] == String::from("3"));
        assert!(extracted_values[3] == String::from("4"));
    }

    #[test]
    #[should_panic]
    fn append_n_following_values__extract_min_four_value_but_two_availbe__panics() {
        let argument_string = vec![String::from("1"), String::from("2")];
        let token_stream = tokens::tokenize(&argument_string);

        let (left_stream, extracted_values) = super::append_n_following_values(&token_stream, Vec::new(), 4, 10).unwrap();
    }

    #[test]
    #[should_panic]
    fn append_n_following_values__min_higher_thatn_max__panics() {
        let argument_string = vec![String::from("1"), String::from("2")];
        let token_stream = tokens::tokenize(&argument_string);

        let (left_stream, extracted_values) = super::append_n_following_values(&token_stream, Vec::new(), 2, 1).unwrap();
    }
}
