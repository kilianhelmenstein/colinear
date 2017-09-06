use args::*;
use args::Count;
use tokens;
use tokens::*;

use std::env;

//fn parse_entire_stream(stream: Vec<Token>, args: Vec<Arg>) -> Result<Vec<Arg>, &'static str> {
    //let (stream_tail, resulting_args) = parse_next_argument(stream, args)?;

    //let tokens_pending = stream_tail.len() > 0;
    //if tokens_pending {
    //    parse_entire_stream(stream_tail, resulting_args);
    //} else {
    //    resulting_args;
    //}
    //Err("")
//}

//fn parse_next_argument<'a>(stream: Vec<Token>, args: &'a[Arg]) -> Result<(Vec<Token>, Vec<Arg<'a>>), &'static str> {
    //if stream.is_empty() || args.is_empty() {
    //    return Err("Invalid input");
    //}

    //match process_token_stream
    //Err("")
//}

fn parse_next_argument_with_defintion<'a, S=Iterator<Item=Token>>(stream: &S, logical_index: usize, arg_definition: ArgDefinition)
    -> Result<(&S, usize, Option<ArgValue>), &'static str>
    where S: Iterator<Item=Token> {
    (arg_definition.interprete_tokens)(stream, logical_index, &arg_definition.count)
}

#[cfg(test)]
mod test {
    use tokens;
    use tokens::*;
    use args::*;

    fn interprete_tokens_increments_logical_index(stream: Iterator<Item=Token>, logical_index: &usize, count: &Count)
        -> Result<(Vec<Token>, usize, Option<ArgValue>), &'static str> {
        let arg_value = ArgValue::new(1, vec!(String::new()));

        Ok(stream, logical_index+1, Some(arg_value))
    }

    #[test]
    fn parse_next_argument_with_defintion() {
        let token_stream = tokens::tokenize(vec!["1", "2", "3", "4", "5"]);
        let arg = ArgDefinition::new(Count::Fixed(1), &interprete_tokens_increments_logical_index);

        let (arg_value, logical_index) = parse_next_argument_with_defintion(token_stream.into_iter(), 0, &arg).unwrap().unwrap();

    }
}
