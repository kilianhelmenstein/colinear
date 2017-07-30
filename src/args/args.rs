use args::tokens::*;

enum Count {
    Fixed(u32),
    Minimum(u32),
    Maximum(u32),
    Range { min: u32, max: u32 }
}

struct InputValueDef {
    count: Count
}

enum Type {
    OnIndex { index: u32 },
    AsOption { shortName: &'static str, longName: &'static str }
}

pub struct Meta {
    pub name: &'static str,
    pub help: &'static str
}

pub struct Arg {
    pub meta: Meta,
    kindOf: Type,
    requiredValue: InputValueDef,
    pub matchedValues: Option<Vec<String>>
}

impl Arg {
    pub fn new() -> Arg {
        Arg {
            meta: Meta { name: "", help: "" },
            kindOf: Type::OnIndex{ index: 0 },
            requiredValue: InputValueDef { count: Count::Fixed(0) },
            matchedValues: None
        }
    }

    pub fn with_name(mut self, name: &'static str) -> Arg {
        self.meta.name = name;
        self
    }

    pub fn with_help(mut self, help: &'static str) -> Arg {
        self.meta.help = help;
        self
    }

    pub fn on_index(mut self, index: u32) -> Arg {
        self.kindOf = Type::OnIndex { index: index };
        self
    }

    pub fn as_option(mut self, short_name: &'static str, long_name: &'static str) -> Arg {
        self.kindOf = Type::AsOption { shortName: short_name, longName: long_name };
        self
    }

    pub fn takes_one_value(mut self) -> Arg {
        self.requiredValue = InputValueDef { count: Count::Fixed(1) };
        self
    }

    pub fn takeTokensAtIndex(&mut self, tokenStream: &[Token], tokenStreamIndex: &u32) -> u32 {
        match self.kindOf {
            Type::OnIndex{..} => return self.match_positionalArg(tokenStreamIndex, tokenStream),
            Type::AsOption{..} => return self.match_optionArg(tokenStreamIndex, tokenStream),
        }
    }

    fn match_positionalArg(&mut self, tokenStreamIndex: &u32, tokenStream: &[Token]) -> u32 {
        if let Type::OnIndex{ index: configuredPosition } = self.kindOf {
            if *tokenStreamIndex == configuredPosition {
                return self.extract_values(tokenStreamIndex, tokenStream);
            }
        }
        tokenStreamIndex.clone()
    }

    fn match_optionArg(&mut self, tokenStreamIndex: &u32, tokenStream: &[Token]) -> u32 {
        if let Type::AsOption{ shortName: definedShortName, longName: definedLongName} = self.kindOf {
            match tokenStream[*tokenStreamIndex as usize] {
                Token::ShortName(ref name) if name == definedShortName => return self.extract_values(&(tokenStreamIndex+1), &tokenStream),
                Token::LongName(ref name) if name == definedLongName => return self.extract_values(&(tokenStreamIndex+1), &tokenStream),
                _ => (),
            }
        }
        tokenStreamIndex.clone()
    }

    fn extract_values(&mut self, tokenStreamIndex: &u32, tokenStream: &[Token]) -> u32 {
        use self::Count::*;

        let tokenStreamIndex = *tokenStreamIndex as usize;
        let mut newTokenStreamIndex = tokenStreamIndex.clone() as u32;

        let availableValues = countAvailableContigousValues(&tokenStream[tokenStreamIndex..]);
        println!("avail. values: {}", availableValues);
        match self.requiredValue.count {
            Fixed(fixedCount) if availableValues >= fixedCount => {
                self.matchedValues = Some(copyContigousValues(&tokenStream[tokenStreamIndex..], &fixedCount));
                newTokenStreamIndex += fixedCount;
            },
            Minimum(minCount) if availableValues >= minCount => {
                self.matchedValues = Some(copyAllContigousValues(&tokenStream[tokenStreamIndex..]));
                newTokenStreamIndex += minCount;
            },
            Maximum(maxCount) => {
                self.matchedValues = Some(copyContigousValues(&tokenStream[tokenStreamIndex..], &maxCount));
                if availableValues > maxCount {
                    newTokenStreamIndex += maxCount;
                } else {
                    newTokenStreamIndex += availableValues;
                }
            },
            Range { min: minCount, max: maxCount } if availableValues >= minCount && availableValues <= maxCount => {
                self.matchedValues = Some(copyContigousValues(&tokenStream[tokenStreamIndex..], &maxCount));
                if availableValues > maxCount {
                    newTokenStreamIndex += maxCount;
                } else {
                    newTokenStreamIndex += availableValues;
                }
            },
            _ => (),
        }

        newTokenStreamIndex
    }
}
