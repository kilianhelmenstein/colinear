use args::*;
use args::Count;
use tokens;
use tokens::*;

use std::env;

fn parse_entire_stream(stream: Box<Iterator<Item=Token>>, args: Vec<ArgDefinition>) -> Result<Vec<ArgValue>, &'static str> {

    match parse_next_argument(stream, 0, args[0..]) {
        Ok(pending_stream, logical_index, maybe_value) => match maybe_value {
            Some(value) => ,
            None => "Could not match",
        },
        Err(error_message) =>
    }
}

fn parse_next_argument<'a>(stream: Box<Iterator<Item=Token>>, logical_index: usize, pending_args: &'a [ArgDefinition])
    -> Result<(Box<Iterator<Item=Token>>, usize, Option<ArgValue>), &'static str> {

    if pending_args.is_empty() {
        return Err("No argument matched");
    }

    match parse_next_argument_with_defintion(stream, logical_index, &pending_args[0]) {
        Ok((pending_stream, resulting_logical_index, maybe_value)) => match maybe_value {
            Some(argument_value) => Ok((pending_stream, resulting_logical_index, Some(argument_value))),
            None => parse_next_argument(pending_stream, resulting_logical_index, &pending_args[1..]),
        },
        Err(error_message) => Err(error_message),
    }
}

fn parse_next_argument_with_defintion(stream: Box<Iterator<Item=Token>>, logical_index: usize, arg_definition: &ArgDefinition)
    -> Result<(Box<Iterator<Item=Token>>, usize, Option<ArgValue>), &'static str> {
    (arg_definition.interprete_tokens)(stream, logical_index, arg_definition.name, &arg_definition.count)
}

#[cfg(test)]
mod test {
    use tokens;
    use tokens::*;
    use args::*;

    fn interprete_tokens_increments_logical_index(stream: Box<Iterator<Item=Token>>, logical_index: usize, name: &'static str, count: &Count, test: usize)
        -> Result<(Box<Iterator<Item=Token>>, usize, Option<ArgValue>), &'static str> {
        let arg_value = ArgValue::new("test", 1, vec!(String::new()));
        println!("Wurde aufgerufen mit {}", test);
        Ok((stream, logical_index+1, Some(arg_value)))
    }

    #[test]
    fn parse_next_argument_with_defintion() {
        let argument_string = vec![String::from("1"), String::from("2"), String::from("3"), String::from("4"), String::from("5")];
        let token_stream = tokens::tokenize(&argument_string);

        //let intepreter;
        let arg = ArgDefinition::new("test", Count::Fixed(1), Box::new(|stream: Box<Iterator<Item=Token>>, logical_index: usize, name: &'static str, count: &Count|
        { interprete_tokens_increments_logical_index(stream, logical_index, name, &count, 4) }));

        let stream_iterator = Box::new(token_stream.into_iter());
        super::parse_next_argument_with_defintion(stream_iterator, 0, &arg);
        panic!("asdf");
    }
}
