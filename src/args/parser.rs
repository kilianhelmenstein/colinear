use super::args::*;
use super::tokens;

use std::env;

pub struct Parser {
    meta: AppMeta,
    configured_args: Vec<Arg>
}

struct AppMeta {
    app_name: &'static str,
    author_name: &'static str,
    author_email: &'static str
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

    pub fn with_arg(mut self, argument: Arg) -> Parser {
        self.configured_args.push(argument);
        self
    }

    fn find_matched_values(&self, arg_name: &str) -> Option<&Vec<String>> {
        for arg in &self.configured_args {
            if arg.name() == arg_name {
                return match arg.matchedValues {
                    Some(ref matched_values) => Some(matched_values),
                    _ => None,
                }
            }
        }
        None
    }

    pub fn value(&self, arg_name: &str) -> Option<String> {
        return match self.find_matched_values(arg_name) {
            Some(matched_values) if matched_values.len() > 0 => Some(matched_values[0].clone()),
            _ => None
        }
    }

    pub fn values(&self, arg_name: &str) -> Option<Vec<String>> {
        return match self.find_matched_values(arg_name) {
            Some(matched_values) => Some(matched_values.clone()),
            _ => None
        }
    }

    pub fn parse_token_stream(&mut self, token_stream: &[tokens::Token]) {
        let mut stream_index: u32 = 0;

        while stream_index < token_stream.len() as u32 {
            let mut resulting_stream_index = stream_index;

            for arg in &mut self.configured_args {
                resulting_stream_index = arg.takeTokensAtIndex(token_stream, &stream_index);
                if resulting_stream_index > stream_index {
                    break;
                }
            }

            let no_argument_matched = resulting_stream_index == stream_index;
            if no_argument_matched {
                panic!("No match for argument token at index {}", stream_index);
            } else {
                stream_index = resulting_stream_index;
            }
        }
    }

    pub fn parse(&mut self) {
        let cl_arguments: Vec<String> = env::args().collect();
        let token_stream = tokens::tokenize(&cl_arguments);
        self.parse_token_stream(&token_stream[1..]);
    }
}


#[cfg(test)]
mod test {

    #[test]
    fn parse_positionalAndOptionalArguments_allMatch() {
        use super::super::args;
        use super::super::tokens;
        use super::Parser;

        let argument_list = vec!(
            String::from("val1"),
            //String::from("val2"),
            String::from("-o"),
            String::from("optval1"),
            String::from("--option2"),
            String::from("optval2"));
        let token_stream = tokens::tokenize(&argument_list);

        let mut parser = Parser::new()
                        .app("Colinear")
                        .with_author("Kilian Helmenstein", "kilian.helmenstein@gmail.com")
                        .with_arg(args::Arg::new()
                                    .with_name("Pos 1")
                                    .with_help("Pos 1")
                                    .on_index(0)
                                    .takes_one_value())
                        .with_arg(args::Arg::new()
                                    .with_name("Option 1")
                                    .with_help("Opt 1")
                                    .as_option("-o", "--option")
                                    .takes_one_value())
                        .with_arg(args::Arg::new()
                                    .with_name("Option 2")
                                    .with_help("Opt 2")
                                    .as_option("-p", "--option2")
                                    .takes_one_value());
        parser.parse_token_stream(&token_stream);

        if let Some(value1_content) = parser.value("Pos 1") {
            if value1_content != "val1" {
                panic!("Matched, but value is false ({})", "Pos 1");
            }
        } else {
            panic!("Not matched ({})", "Pos 1");
        }

        if let Some(opt1_content) = parser.value("Option 1") {
            if opt1_content != "optval1" {
                panic!("Matched, but value is false ({})", "Option 1");
            }
        } else {
            panic!("Not matched ({})", "Option 1");
        }

        if let Some(opt2_content) = parser.value("Option 2") {
            if opt2_content != "optval2" {
                panic!("Matched, but value is false ({})", "Option 2");
            }
        } else {
            panic!("Not matched ({})", "Option 2");
        }
    }
}
