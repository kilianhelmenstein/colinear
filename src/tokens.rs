pub enum Token {
    Value(String),
    ShortName(String),
    LongName(String)
}

impl Token {
    pub fn new(content: &str) -> Token {
        use utils;

        let content_string = String::from(content);

        if utils::index_in_string_equals_to(&0, content, &'-') {
            if utils::index_in_string_equals_to(&1, content, &'-') {
                return Token::LongName(content_string);
            } else {
                return Token::ShortName(content_string);
            }
        }
        Token::Value(content_string)
    }
}

pub fn tokenize(arguments: &[String]) -> Vec<Token>
{
    let mut token_stream = Vec::new();
    for arg in arguments {
        token_stream.push(Token::new(arg));
    }
    token_stream
}

pub fn count_available_contigous_values(token_stream: &[Token]) -> u32 {
    let mut contigous_value_tokens_count = 0;

    for token in token_stream {
        if let Token::Value(..) = *token {
            contigous_value_tokens_count += 1;
        }
    }
    contigous_value_tokens_count
}

pub fn copy_contigous_values(token_stream: &[Token], max_count: &u32) -> Vec<String> {
    let mut contigous_values = Vec::new();
    for token in token_stream {
        if let Token::Value(ref value) = *token {
            contigous_values.push(value.clone());
            if contigous_values.len() >= *max_count as usize {
                break;
            }
        }
    }
    contigous_values
}

pub fn copy_all_contigous_values(token_stream: &[Token]) -> Vec<String> {
    copy_contigous_values(token_stream, &(token_stream.len() as u32))
}

#[cfg(test)]
mod test {
    #[test]
    fn tokenize_variantlist_validresult() {
        use super::super::*;
        use super::*;

        let argument_list = vec!(
            String::from("val1"),
            String::from("val2"),
            String::from("-o"),
            String::from("optval1"),
            String::from("--option2"),
            String::from("optval2"));
        let token_stream = tokens::tokenize(&argument_list);

        match token_stream[0] {
            Token::Value(ref val) if *val == String::from("val1") => (),
            _ => panic!("val1 false"),
        }

        match token_stream[1] {
            Token::Value(ref val) if *val == String::from("val2") => (),
            _ => panic!("val2 false"),
        }

        match token_stream[2] {
            Token::ShortName(ref name) if *name == String::from("-o") => (),
            _ => panic!("-o false"),
        }

        match token_stream[3] {
            Token::Value(ref val) if *val == String::from("optval1") => (),
            _ => panic!("optval1 false"),
        }

        match token_stream[4] {
            Token::LongName(ref name) if *name == String::from("--option2") => (),
            _ => panic!("-o false"),
        }

        match token_stream[5] {
            Token::Value(ref val) if *val == String::from("optval2") => (),
            _ => panic!("optval2 false"),
        }
    }
}
