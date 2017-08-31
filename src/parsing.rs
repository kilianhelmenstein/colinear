use args::*;
use args::processor::Count;
use tokens;

use std::env;

struct Arg {
    definition: ArgDefinition,
    value: ArgValue
}

struct ArgDefinition {
    pub name: &'static str,
    pub help: &'static str

    pub short_name: String,
    pub long_name: String,

    interprete_token_stream: Fn(stream: &[Token], logical_index: &usize, count: Count)
                                -> (&[Token], &usize, ArgValue)
}

struct ArgValue {
    occurences: usize,
    assigned_values: Vec<String>
}

fn parse_entire_stream(stream: &[Token], args: Vec<Arg>) -> Result<Vec<Arg>, &str> {
    let (stream_tail, resulting_args) = parse_next_argument(stream, args)?;

    let tokens_pending = stream_tail.len() > 0;
    if tokens_pending {
        parse_entire_stream(stream_tail, resulting_args);
    } else {
        resulting_args;
    }
}

fn parse_next_argument(stream: &[Token], args: &[Arg]) -> Result<(&[Token], Vec<Arg>), &str> {
    if stream.is_empty() || args.is_empty() {
        return Err("Invalid input");
    }

    match process_token_stream
}
