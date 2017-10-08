use args::*;
use tokens::*;

pub fn parse(stream: &[Token], args: Vec<ArgDefinition>) -> Result<Vec<ArgValue>, &'static str> {
    parse_entire_stream(stream, args, Vec::new(), 0)
}

fn parse_entire_stream(stream: &[Token], args: Vec<ArgDefinition>, results_yet: Vec<ArgValue>, logical_index: usize)
    -> Result<Vec<ArgValue>, &'static str> {

    match parse_next_argument(stream, logical_index, &args) {
        Ok((pending_stream, logical_index, maybe_a_result)) => match maybe_a_result {
            Some(new_arg_result) => parse_entire_stream(pending_stream, args, merged_args(results_yet, new_arg_result), logical_index),
            None => Ok(results_yet),
        },
        Err(error_message) => Err(error_message),
    }
}

fn parse_next_argument<'a, 'b>(stream: &'a [Token], logical_index: usize, pending_args: &'b [ArgDefinition])
    -> Result<(&'a [Token], usize, Option<ArgValue>), &'static str> {

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

fn parse_next_argument_with_defintion<'a>(stream: &'a [Token], logical_index: usize, arg_definition: &ArgDefinition)
    -> Result<(&'a [Token], usize, Option<ArgValue>), &'static str> {
    (arg_definition.interprete_tokens)(stream, logical_index, arg_definition.name, &arg_definition.count)
}

#[cfg(test)]
mod test {
    use tokens;
    use tokens::*;
    use args::*;

    fn interprete_tokens_by_capturing_one_value<'a>(stream: &'a [Token], logical_index: usize, name: &'static str, count: &Count)
        -> Result<(&'a [Token], usize, Option<ArgValue>), &'static str> {

        if stream.len() == 0 {
            return Ok((stream, logical_index, None));
        }

        if let Token::Value(ref val) = stream[0] {
            Ok((&stream[1..], logical_index+1, Some(ArgValue::new(name, 1, vec![val.clone()]))))
        } else {
            Ok((&stream[1..], logical_index+1, None))
        }
    }

    #[test]
    fn parse_next_argument_with_defintion__test() {
        let argument_string = vec![String::from("1"), String::from("2"), String::from("3"), String::from("4")];
        let token_stream = tokens::tokenize(&argument_string);

        let arg_def = ArgDefinition::new("first", Count::Fixed(1), Box::new(interprete_tokens_by_capturing_one_value));

        let (pending_stream, logical_index, maybe_value) = super::parse_next_argument_with_defintion(&token_stream, 0, &arg_def).unwrap();

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

        let (pending_stream, logical_index, maybe_value) = super::parse_next_argument(&token_stream, 0, &arg_defs).unwrap();

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

        let values = super::parse_entire_stream(&token_stream, arg_defs, Vec::new(), 0).unwrap();

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

        let result = super::parse(&token_stream, arg_defs).unwrap();

        print!("{}", result.len());
        assert!(result.len() == 4);
    }
}
