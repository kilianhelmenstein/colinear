use args::tokens::*;

pub struct Arg {
    meta: Meta,
    kindOf: Type,
    required_values_specification: Count,
    pub matchedValues: Option<Vec<String>>
}

pub struct Meta {
    pub name: &'static str,
    pub help: &'static str
}

enum Type {
    OnIndex { index: u32 },
    AsOption { shortName: &'static str, longName: &'static str }
}

enum Count {
    Fixed(u32),
    Minimum(u32),
    Maximum(u32),
    Range { min: u32, max: u32 }
}

impl Arg {
    pub fn new() -> Arg {
        Arg {
            meta: Meta { name: "", help: "" },
            kindOf: Type::OnIndex{ index: 0 },
            required_values_specification: Count::Fixed(0),
            matchedValues: None
        }
    }

    pub fn name(&self) -> String {
        String::from(self.meta.name)
    }

    pub fn with_name(mut self, name: &'static str) -> Arg {
        self.meta.name = name;
        self
    }

    pub fn help(&self) -> String {
        String::from(self.meta.help)
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
        self.required_values_specification = Count::Fixed(1);
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
        match self.required_values_specification {
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

#[cfg(test)]
mod test {
    use super::Arg;

    fn check_match_result(dut: &Arg, shall_matched_value: &str) {
        match dut.matchedValues {
            Some(ref matched_values) if matched_values.len() > 0 => {
                if let Some(matched_value) = matched_values.first() {
                    if *matched_value == String::from(shall_matched_value) {
                        println!("Matched right value: {}", matched_value);
                    } else {
                        panic!("Matched value is '{}', but should be '{}'", matched_value, shall_matched_value)
                    }
                } else {
                    panic!("Matched invalid value: {}")
                }
            },
            Some(ref matched_values) => panic!("Matched, but match count is zero"),
            _ => panic!("Matched no value"),
        }
    }

    fn check_resulting_index(shall: u32, is: u32) {
        if shall != is {
            panic!("Delivered false resulting index (Shall be '{}', but is '{}')", shall, is);
        }
    }

    #[test]
    fn takeTokensAtIndex_onindex_matches() {
        use super::Arg;
        use super::super::tokens;

        let argument_list = vec!(
            String::from("val1"),
            String::from("val2"));
        let token_stream = tokens::tokenize(&argument_list);

        let mut argument_1st = Arg::new()
                        .with_name("Index 0")
                        .with_help("Index 0")
                        .on_index(0)
                        .takes_one_value();
        let mut argument_2nd = Arg::new()
                        .with_name("Index 1")
                        .with_help("Index 1")
                        .on_index(1)
                        .takes_one_value();
        let resulting_index_1st = argument_1st.takeTokensAtIndex(&token_stream, &0);
        check_resulting_index(1, resulting_index_1st);
        check_match_result(&argument_1st, "val1");

        let resulting_index_2nd = argument_2nd.takeTokensAtIndex(&token_stream, &1);
        check_resulting_index(2, resulting_index_2nd);
        check_match_result(&argument_2nd, "val2");
    }

    #[test]
    fn takeTokensAtIndex_asoption_matches() {
        use super::Arg;
        use super::super::tokens;

        let argList = vec!(
            String::from("val1"),
            String::from("val2"),
            String::from("-o"),
            String::from("optval1"),
            String::from("--option2"),
            String::from("optval2"));

        let tokenStream = tokens::tokenize(&argList);

        let mut argument_1st_option = Arg::new()
                        .with_name("Opt 1")
                        .with_help("Opt 1")
                        .as_option("-o", "--option")
                        .takes_one_value();
        let mut argument_2nd_option = Arg::new()
                        .with_name("Opt 2")
                        .with_help("Opt 2")
                        .as_option("-p", "--option2")
                        .takes_one_value();

        let resulting_index = argument_1st_option.takeTokensAtIndex(&tokenStream, &2);
        check_resulting_index(4, resulting_index);
        check_match_result(&argument_1st_option, "optval1");

        let resulting_index = argument_2nd_option.takeTokensAtIndex(&tokenStream, &4);
        check_resulting_index(6, resulting_index);
        check_match_result(&argument_2nd_option, "optval2");
    }
}
