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
