use super::args::*;
use super::tokens;

use std::env;

struct AppMeta {
    app_name: &'static str,
    author_name: &'static str,
    author_email: &'static str
}

pub struct Parser {
    meta: AppMeta,
    configured_args: Vec<Arg>
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            meta: AppMeta { app_name: "", author_name: "", author_email: "" },
            configured_args: Vec::new()
        }
    }

    pub fn app(mut self, name: &'static str) -> Parser {
        self.meta.app_name = name;
        self
    }

    pub fn with_author(mut self, name: &'static str, email: &'static str) -> Parser {
        self.meta.author_name = name;
        self.meta.author_email = email;
        self
    }

    pub fn with_arg(mut self, mut argument: Arg) -> Parser {
        self.configured_args.push(argument);
        self
    }

    pub fn parse_token_stream(&mut self, token_stream: &Vec<tokens::Token>) {
        let mut stream_index: u32 = 1;

        while stream_index < token_stream.len() as u32 {
            let mut resulting_stream_index = stream_index;

            for arg in self.configured_args.iter() {
                resulting_stream_index = arg.takeTokensAtIndex(token_stream, &stream_index);
                if resulting_stream_index > stream_index {
                    break;
                }
            }

            let no_argument_matched = resulting_stream_index == stream_index;
            if no_argument_matched {
                panic!("No match for argument token at index {}", stream_index);
            }
        }
    }

    pub fn parse(&mut self) {
        let cl_arguments: Vec<String> = env::args().collect();
        let token_stream = tokens::tokenize(&cl_arguments);
        self.parse_token_stream(&token_stream);
    }
}
