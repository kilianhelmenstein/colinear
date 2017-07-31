use args::utils;

pub enum Token {
    Value(String),
    ShortName(String),
    LongName(String)
}

impl Token {
    pub fn new(content: &str) -> Token {
        let contentString = String::from(content);

        if utils::indexInStringEqualsTo(&0, content, &'-') {
            if utils::indexInStringEqualsTo(&1, content, &'-') {
                return Token::LongName(contentString);
            } else {
                return Token::ShortName(contentString);
            }
        }
        Token::Value(contentString)
    }
}

pub fn tokenize(arguments: &[String]) -> Vec<Token>
{
    let mut tokenStream = Vec::new();
    for arg in arguments {
        tokenStream.push(Token::new(arg));
    }
    tokenStream
}

pub fn countAvailableContigousValues(tokenStream: &[Token]) -> u32 {
    let mut contigousValueTokensCount = 0;

    for token in tokenStream {
        if let Token::Value(..) = *token {
            contigousValueTokensCount += 1;
        }
    }
    contigousValueTokensCount
}

pub fn copyContigousValues(tokenStream: &[Token], maxCount: &u32) -> Vec<String> {
    let mut contigousValues = Vec::new();
    for token in tokenStream {
        if let Token::Value(ref value) = *token {
            contigousValues.push(value.clone());
            if contigousValues.len() >= *maxCount as usize {
                break;
            }
        }
    }
    contigousValues
}

pub fn copyAllContigousValues(tokenStream: &[Token]) -> Vec<String> {
    copyContigousValues(tokenStream, &(tokenStream.len() as u32))
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
