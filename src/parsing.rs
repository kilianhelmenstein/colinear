use args::*;
use args::Count;
use tokens;
use tokens::Token;

use std::env;

fn parse_entire_stream(stream: &[Token], args: Vec<Arg>) -> Result<Vec<Arg>, &str> {
    //let (stream_tail, resulting_args) = parse_next_argument(stream, args)?;

    //let tokens_pending = stream_tail.len() > 0;
    //if tokens_pending {
    //    parse_entire_stream(stream_tail, resulting_args);
    //} else {
    //    resulting_args;
    //}
}

fn parse_next_argument(stream: &[Token], args: &[Arg]) -> Result<(&[Token], Vec<Arg>), &str> {
    //if stream.is_empty() || args.is_empty() {
    //    return Err("Invalid input");
    //}

    //match process_token_stream
}

fn try_parse(stream: &[Token], logical_index: &usize, arg_definition: &ArgDefinition) -> Result<(&[Token], usize, Option<ArgValue>), &str> {
    arg_definition.interprete_tokens(stream, logical_index, arg_definition.count)
}

#[cfg(test)]
mod test {
    fn interprete_tokens_increments_logical_index(stream: &[Token], logical_index: &usize, count: &Count) -> Result<(&[Token], usize, Option<ArgValue>), &str> {
        let arg_value = ArgValue::new(1, vec!(String::from(stream[0])));

        (stream[1..], logical_index+1, Some(arg_value));
    }

    #[test]
    fn try_parse() {
        let token_stream = tokens::tokenize(vec!["1", "2", "3", "4", "5"]);
        let arg = ArgDefinition::new(Count::Fixed(1), &interprete_tokens_increments_logical_index);

        let (arg_value, logical_index) = try_parse_with_argument().unwrap().unwrap();

    }
}
