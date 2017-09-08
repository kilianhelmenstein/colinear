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

fn parse_next_argument_with_defintion<'a>(stream: &'a Iterator<Item=Token>, logical_index: usize, arg_definition: ArgDefinition)
    -> Result<(&'a Iterator<Item=Token>, usize, Option<ArgValue>), &'static str> {
    (arg_definition.interprete_tokens)(stream, logical_index, arg_definition.count)
}

#[cfg(test)]
mod test {
    use tokens;
    use tokens::*;
    use args::*;

    fn interprete_tokens_increments_logical_index(stream: &Iterator<Item=Token>, logical_index: usize, count: Count, test: usize)
        -> Result<(&Iterator<Item=Token>, usize, Option<ArgValue>), &'static str> {
        let arg_value = ArgValue::new(1, vec!(String::new()));
        println!("Wurde aufgerufen mit {}", test);
        Ok((stream, logical_index+1, Some(arg_value)))
    }

    #[test]
    fn parse_next_argument_with_defintion() {
        let argument_string = vec![String::from("1"), String::from("2"), String::from("3"), String::from("4"), String::from("5")];
        let token_stream = tokens::tokenize(&argument_string);

        let intepreter =
            |stream: &Iterator<Item=Token>, logical_index: usize, count: Count|
            -> Result<(&Iterator<Item=Token>, usize, Option<ArgValue>), &'static str>
            { return interprete_tokens_increments_logical_index(stream, logical_index, count, 42) };
        let arg = ArgDefinition::new(Count::Fixed(1), &intepreter);

        super::parse_next_argument_with_defintion(&token_stream.into_iter(), 0, arg);
        panic!("asdf");
    }
}
