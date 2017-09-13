pub enum Token {
    Value(String),
    ShortName(String),
    LongName(String)
}

impl Token {
    pub fn from(content: &str) -> Vec<Token> {
        use utils::index_in_string_equals_to;

        let mut resulting_tokens = Vec::new();
        let content_string = String::from(content);

        if index_in_string_equals_to(content, &0, &'-') {
            if index_in_string_equals_to(content, &1, &'-') {
                resulting_tokens.push(Token::LongName(content_string));
            } else {
                let options = &content_string[1..];
                for one_option in options.chars() {
                    let mut option_string = String::from("-");
                    option_string.push(one_option);
                    resulting_tokens.push(Token::ShortName(option_string));
                }
            }
        } else {
            resulting_tokens.push(Token::Value(content_string));
        }

        resulting_tokens
    }
}

pub fn tokenize(arguments: &[String]) -> Vec<Token>
{
    let mut token_stream = Vec::new();
    for arg in arguments {
        token_stream.append(&mut Token::from(arg));
    }
    token_stream
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

    #[test]
    fn tokenize_combinedoptions_validresult() {
        use super::super::*;

        let argument_list = vec!(
            String::from("-abc"));
        let token_stream = tokens::tokenize(&argument_list);

        compare_short_name("-a", &token_stream[0]);
        compare_short_name("-b", &token_stream[1]);
        compare_short_name("-c", &token_stream[2]);
    }

    fn compare_short_name(expected: &str, token: &super::Token) {
        use super::Token;

        match *token {
            Token::ShortName(ref name) if *name == String::from(expected) => (),
            Token::ShortName(ref name) => panic!("Expected {}, got {}", expected, name),
            Token::LongName(_) => panic!("Expected ShortName, got LongName"),
            Token::Value(_) => panic!("Expected ShortName, got Value"),
        }
    }
}
