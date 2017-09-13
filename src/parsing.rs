use args::*;
use args::Count;
use tokens;
use tokens::*;

use std::env;

pub fn parse(mut stream: Box<Iterator<Item=Token>>, args: Vec<ArgDefinition>) -> Result<Vec<ArgValue>, &'static str> {
    parse_entire_stream(stream, args, Vec::new(), 0)
}

fn parse_entire_stream(mut stream: Box<Iterator<Item=Token>>, args: Vec<ArgDefinition>, results_yet: Vec<ArgValue>, logical_index: usize)
    -> Result<Vec<ArgValue>, &'static str> {

    match parse_next_argument(stream, logical_index, &args) {
        Ok((pending_stream, logical_index, maybe_a_result)) => match maybe_a_result {
            Some(new_arg_result) => parse_entire_stream(pending_stream, args, merged_args(results_yet, new_arg_result), logical_index),
            None => Ok(results_yet),
        },
        Err(error_message) => Err(error_message),
    }
}

fn parse_next_argument<'a>(mut stream: Box<Iterator<Item=Token>>, logical_index: usize, pending_args: &'a [ArgDefinition])
    -> Result<(Box<Iterator<Item=Token>>, usize, Option<ArgValue>), &'static str> {

    if pending_args.is_empty() {
        return Ok((stream, logical_index, None));
    }

    match parse_next_argument_with_defintion(stream, logical_index, &pending_args[0]) {
        Ok((pending_stream, resulting_logical_index, maybe_value)) => match maybe_value {
            Some(argument_value) => Ok((pending_stream, resulting_logical_index, Some(argument_value))),
            None => parse_next_argument(pending_stream, resulting_logical_index, &pending_args[1..]),
        },
        Err(error_message) => Err(error_message),
    }
}

fn parse_next_argument_with_defintion(mut stream: Box<Iterator<Item=Token>>, logical_index: usize, arg_definition: &ArgDefinition)
    -> Result<(Box<Iterator<Item=Token>>, usize, Option<ArgValue>), &'static str> {
    (arg_definition.interprete_tokens)(stream, logical_index, arg_definition.name, &arg_definition.count)
}

#[cfg(test)]
mod test {
    use tokens;
    use tokens::*;
    use args::*;

    fn interprete_tokens_by_capturing_one_value(mut stream: Box<Iterator<Item=Token>>, logical_index: usize, name: &'static str, count: &Count)
        -> Result<(Box<Iterator<Item=Token>>, usize, Option<ArgValue>), &'static str> {

        match stream.next() {
            Some(token) => Ok((stream, logical_index+1, Some(ArgValue::new(name, 1, vec![String::from("1")])))),
            None => Ok((stream, logical_index, None)),
        }
    }

    #[test]
    fn parse_next_argument_with_defintion__test() {
        let argument_string = vec![String::from("1"), String::from("2"), String::from("3"), String::from("4")];
        let token_stream = tokens::tokenize(&argument_string);

        let arg_def = ArgDefinition::new("first", Count::Fixed(1), Box::new(interprete_tokens_by_capturing_one_value));

        let stream_iterator = Box::new(token_stream.into_iter());
        let (pending_stream, logical_index, maybe_value) = super::parse_next_argument_with_defintion(stream_iterator, 0, &arg_def).unwrap();

        let value = match maybe_value {
            Some(value) => value,
            None => panic!("Should not be empty"),
        };

        assert!(value.name == String::from("first"));
        assert!(value.occurences == 1);
        assert!(value.assigned_values == vec![String::from("1")]);
    }

    #[test]
    fn parse_next_argument__test() {
        let argument_string = vec![String::from("1"), String::from("2"), String::from("3"), String::from("4")];
        let token_stream = tokens::tokenize(&argument_string);

        let arg_defs = vec![ArgDefinition::new("first", Count::Fixed(1), Box::new(interprete_tokens_by_capturing_one_value))];

        let stream_iterator = Box::new(token_stream.into_iter());
        let (pending_stream, logical_index, maybe_value) = super::parse_next_argument(stream_iterator, 0, &arg_defs).unwrap();

        let value = match maybe_value {
            Some(value) => value,
            None => panic!("Should not be empty"),
        };

        assert!(value.name == String::from("first"));
        assert!(value.occurences == 1);
        assert!(value.assigned_values == vec![String::from("1")]);
    }

    #[test]
    fn parse_entire_stream__test() {
        let argument_string = vec![String::from("1"), String::from("2"), String::from("3"), String::from("4")];
        let token_stream = tokens::tokenize(&argument_string);

        let arg_defs = vec![ArgDefinition::new("first", Count::Fixed(1), Box::new(interprete_tokens_by_capturing_one_value))];

        let stream_iterator = Box::new(token_stream.into_iter());
        let values = super::parse_entire_stream(stream_iterator, arg_defs, Vec::new(), 0).unwrap();

        assert!(values.len() == 4);
        assert!(values[0].name == String::from("first"));
        assert!(values[0].occurences == 1);
        assert!(values[0].assigned_values == vec![String::from("1")]);
    }

    #[test]
    fn parse__test() {
        let argument_string = vec![String::from("1"), String::from("2"), String::from("3"), String::from("4")];
        let token_stream = tokens::tokenize(&argument_string);

        let arg1 = ArgDefinition::new("first", Count::Fixed(1), Box::new(interprete_tokens_by_capturing_one_value));
        let arg_defs = vec![arg1];

        let mut stream_iterator = Box::new(token_stream.into_iter());
        let result = super::parse(stream_iterator, arg_defs).unwrap();

        print!("{}", result.len());
        assert!(result.len() == 4);
    }
}
