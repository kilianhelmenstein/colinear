use args::tokens::*;

pub struct Arg {
    meta: Meta,
    kind_of: Type,
    required_values_specification: Count,
    pub matched_values: Option<Vec<String>>
}

pub struct Meta {
    pub name: &'static str,
    pub help: &'static str
}

enum Type {
    OnIndex { index: u32 },
    AsOption { short_name: &'static str, long_name: &'static str }
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
            kind_of: Type::OnIndex{ index: 0 },
            required_values_specification: Count::Fixed(0),
            matched_values: None
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
        self.kind_of = Type::OnIndex { index: index };
        self
    }

    pub fn as_option(mut self, short_name: &'static str, long_name: &'static str) -> Arg {
        self.kind_of = Type::AsOption { short_name: short_name, long_name: long_name };
        self
    }

    pub fn takes_one_value(mut self) -> Arg {
        self.required_values_specification = Count::Fixed(1);
        self
    }

    pub fn takes_n_values(mut self, n: u32) -> Arg {
        self.required_values_specification = Count::Fixed(n);
        self
    }

    pub fn takes_min_values(mut self, min: u32) -> Arg {
        self.required_values_specification = Count::Minimum(min);
        self
    }

    pub fn takes_max_values(mut self, max: u32) -> Arg {
        self.required_values_specification = Count::Maximum(max);
        self
    }

    pub fn takes_min_max_values(mut self, min: u32, max: u32) -> Arg {
        self.required_values_specification = Count::Range { min: min, max: max };
        self
    }

    pub fn take_tokens_at_index(&mut self, token_stream: &[Token], token_stream_index: &u32) -> Result<u32, &'static str> {
        match self.kind_of {
            Type::OnIndex{..} => return self.match_positional_arg(token_stream_index, token_stream),
            Type::AsOption{..} => return self.match_optional_arg(token_stream_index, token_stream),
        }
    }

    fn match_positional_arg(&mut self, token_stream_index: &u32, token_stream: &[Token]) -> Result<u32, &'static str> {
        if let Type::OnIndex{ index: configured_position } = self.kind_of {
            if *token_stream_index == configured_position {
                return self.extract_values(token_stream_index, token_stream);
            }
        }

        Ok(token_stream_index.clone())
    }

    fn match_optional_arg(&mut self, token_stream_index: &u32, token_stream: &[Token]) -> Result<u32, &'static str> {
        if let Type::AsOption{ short_name: defined_short_name, long_name: defined_long_name} = self.kind_of {
            match token_stream[*token_stream_index as usize] {
                Token::ShortName(ref name) if name == defined_short_name => return self.extract_values(&(token_stream_index+1), &token_stream),
                Token::LongName(ref name) if name == defined_long_name => return self.extract_values(&(token_stream_index+1), &token_stream),
                _ => (),
            }
        }

        Ok(token_stream_index.clone())
    }

    fn extract_values(&mut self, token_stream_index: &u32, token_stream: &[Token]) -> Result<u32, &'static str> {
        use self::Count::*;

        let token_stream_index = *token_stream_index as usize;
        let mut new_token_stream_index = token_stream_index.clone() as u32;

        let available_values = count_available_contigous_values(&token_stream[token_stream_index..]);
        println!("avail. values: {}", available_values);
        match self.required_values_specification {
            Fixed(fixed_count) if available_values >= fixed_count => {
                self.matched_values = Some(copy_contigous_values(&token_stream[token_stream_index..], &fixed_count));
                new_token_stream_index += fixed_count;
            },
            Minimum(min_count) if available_values >= min_count => {
                self.matched_values = Some(copy_all_contigous_values(&token_stream[token_stream_index..]));
                new_token_stream_index += available_values;
            },
            Minimum(_) => return Err("To few arguments"),
            Maximum(max_count) => {
                self.matched_values = Some(copy_contigous_values(&token_stream[token_stream_index..], &max_count));
                if available_values > max_count {
                    new_token_stream_index += max_count;
                } else {
                    new_token_stream_index += available_values;
                }
            },
            Range { min: min_count, max: max_count } if available_values >= min_count && available_values <= max_count => {
                self.matched_values = Some(copy_contigous_values(&token_stream[token_stream_index..], &max_count));
                if available_values > max_count {
                    new_token_stream_index += max_count;
                } else {
                    new_token_stream_index += available_values;
                }
            },
            _ => (),
        }

        Ok(new_token_stream_index)
    }
}

#[cfg(test)]
mod test {
    use super::Arg;

    fn check_match_result(dut: &Arg, shall_matched_value: &str) {
        match dut.matched_values {
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
    fn take_tokens_at_index__onindex_fixedcount_is_1__matches() {
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
        let resulting_index_1st = argument_1st.take_tokens_at_index(&token_stream, &0).unwrap();
        check_resulting_index(1, resulting_index_1st);
        check_match_result(&argument_1st, "val1");

        let resulting_index_2nd = argument_2nd.take_tokens_at_index(&token_stream, &1).unwrap();
        check_resulting_index(2, resulting_index_2nd);
        check_match_result(&argument_2nd, "val2");
    }

    #[test]
    fn take_tokens_at_index__onindex_mincount__matches() {
        use super::Arg;
        use super::super::tokens;

        let argument_list = vec!(
            String::from("val1"),
            String::from("val2"),
            String::from("val3"));
        let token_stream = tokens::tokenize(&argument_list);

        let mut argument = Arg::new()
                        .with_name("Index 0")
                        .with_help("Index 0")
                        .on_index(0)
                        .takes_min_values(2);
        let resulting_index = argument.take_tokens_at_index(&token_stream, &0).unwrap();
        check_resulting_index(3, resulting_index);
        let matched_values = argument.matched_values;
        match matched_values {
            Some(ref matched_values) if matched_values.len() == 3 => {
                assert_eq!(&matched_values[0], "val1");
                assert_eq!(&matched_values[1], "val2");
                assert_eq!(&matched_values[2], "val3");
            },
            _ => panic!("Matched no values"),
        }
    }

    #[test]
    #[should_panic]
    fn take_tokens_at_index__onindex_mincount__panics() {
        use super::Arg;
        use super::super::tokens;

        let argument_list = vec!(
            String::from("val1"));
        let token_stream = tokens::tokenize(&argument_list);

        let mut argument = Arg::new()
                        .with_name("Index 0")
                        .with_help("Index 0")
                        .on_index(0)
                        .takes_min_values(2);
        argument.take_tokens_at_index(&token_stream, &0).unwrap();
    }

    #[test]
    fn take_tokens_at_index__onindex_maxcount__matches() {
        use super::Arg;
        use super::super::tokens;

        let argument_list = vec!(
            String::from("val1"),
            String::from("val2"),
            String::from("val3"));
        let token_stream = tokens::tokenize(&argument_list);

        let mut argument = Arg::new()
                        .with_name("Index 0")
                        .with_help("Index 0")
                        .on_index(0)
                        .takes_max_values(2);
        let resulting_index = argument.take_tokens_at_index(&token_stream, &0).unwrap();
        check_resulting_index(2, resulting_index);
        let matched_values = argument.matched_values;
        match matched_values {
            Some(ref matched_values) if matched_values.len() == 2 => {
                assert_eq!(&matched_values[0], "val1");
                assert_eq!(&matched_values[1], "val2");
            },
            _ => panic!("Matched no values"),
        }
    }

    #[test]
    fn take_tokens_at_index__asoption__matches() {
        use super::Arg;
        use super::super::tokens;

        let arg_list = vec!(
            String::from("val1"),
            String::from("val2"),
            String::from("-o"),
            String::from("optval1"),
            String::from("--option2"),
            String::from("optval2"));

        let token_stream = tokens::tokenize(&arg_list);

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

        let resulting_index = argument_1st_option.take_tokens_at_index(&token_stream, &2).unwrap();
        check_resulting_index(4, resulting_index);
        check_match_result(&argument_1st_option, "optval1");

        let resulting_index = argument_2nd_option.take_tokens_at_index(&token_stream, &4).unwrap();
        check_resulting_index(6, resulting_index);
        check_match_result(&argument_2nd_option, "optval2");
    }
}
