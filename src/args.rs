use tokens;
use tokens::*;

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

pub struct IndexPair {
    pub physical_index: u32,
    pub logical_index: u32
}

impl Clone for IndexPair {
    fn clone(&self) -> Self {
        IndexPair { physical_index: self.physical_index, logical_index: self.physical_index }
    }
}

impl IndexPair {
    pub fn zero_indeces() -> IndexPair {
        IndexPair { physical_index: 0, logical_index: 0 }
    }

    fn with_phys(physical_index: u32) -> IndexPair {
        IndexPair { physical_index: physical_index, logical_index: 0 }
    }

    pub fn tokens_left(&self, token_stream: &[tokens::Token]) -> bool {
        self.physical_index < token_stream.len() as u32
    }
}

pub struct ArgBuilder {
    built_arg: Arg
}

impl ArgBuilder {
    fn new() -> ArgBuilder {
        ArgBuilder {
            built_arg: Arg {
                meta: Meta { name: "", help: "" },
                kind_of: Type::OnIndex{ index: 0 },
                required_values_specification: Count::Fixed(0),
                matched_values: None
            }
        }
    }

    pub fn build(self) -> Arg {
        self.built_arg
    }

    pub fn with_name(mut self, name: &'static str) -> ArgBuilder {
        self.built_arg.meta.name = name;
        self
    }

    pub fn with_help(mut self, help: &'static str) -> ArgBuilder {
        self.built_arg.meta.help = help;
        self
    }

    pub fn on_index(mut self, index: u32) -> ArgBuilder {
        self.built_arg.kind_of = Type::OnIndex { index: index };
        self
    }

    pub fn as_option(mut self, short_name: &'static str, long_name: &'static str) -> ArgBuilder {
        self.built_arg.kind_of = Type::AsOption { short_name: short_name, long_name: long_name };
        self
    }

    pub fn takes_one_value(mut self) -> ArgBuilder {
        self.built_arg.required_values_specification = Count::Fixed(1);
        self
    }

    pub fn takes_n_values(mut self, n: u32) -> ArgBuilder {
        self.built_arg.required_values_specification = Count::Fixed(n);
        self
    }

    pub fn takes_min_values(mut self, min: u32) -> ArgBuilder {
        self.built_arg.required_values_specification = Count::Minimum(min);
        self
    }

    pub fn takes_max_values(mut self, max: u32) -> ArgBuilder {
        self.built_arg.required_values_specification = Count::Maximum(max);
        self
    }

    pub fn takes_min_max_values(mut self, min: u32, max: u32) -> ArgBuilder {
        self.built_arg.required_values_specification = Count::Range { min: min, max: max };
        self
    }
}

impl Arg {
    pub fn with_name(name: &'static str) -> ArgBuilder {
        let builder = ArgBuilder::new();
        builder.with_name(name)
    }

    pub fn name(&self) -> String {
        String::from(self.meta.name)
    }

    pub fn help(&self) -> String {
        String::from(self.meta.help)
    }

    pub fn take_tokens_at_index(&mut self, token_stream: &[Token], token_stream_index: &IndexPair) -> Result<IndexPair, &'static str> {
        match self.kind_of {
            Type::OnIndex{..} => return self.match_positional_arg(token_stream_index, token_stream),
            Type::AsOption{..} => return self.match_optional_arg(token_stream_index, token_stream),
        }
    }

    fn match_positional_arg(&mut self, token_stream_index: &IndexPair, token_stream: &[Token]) -> Result<IndexPair, &'static str> {
        if let Type::OnIndex{ index: configured_position } = self.kind_of {
            let stream_is_on_right_position = token_stream_index.logical_index == configured_position;
            if stream_is_on_right_position {
                let mut new_index = token_stream_index.clone();

                new_index.physical_index = self.extract_values(&token_stream_index.physical_index, token_stream)?;
                let took_tokens = new_index.physical_index > token_stream_index.physical_index;
                if took_tokens {
                    new_index.logical_index += 1;
                    return Ok(new_index);
                }
            }
        }

        Ok(token_stream_index.clone())
    }

    fn match_optional_arg(&mut self, token_stream_index: &IndexPair, token_stream: &[Token]) -> Result<IndexPair, &'static str> {
        let mut resulting_index = token_stream_index.clone();

        if let Type::AsOption{ short_name: defined_short_name, long_name: defined_long_name} = self.kind_of {
            let values_begin_index = token_stream_index.physical_index + 1;

            resulting_index.physical_index = match token_stream[token_stream_index.physical_index as usize] {
                Token::ShortName(ref name) if name == defined_short_name => {
                    self.extract_values(&values_begin_index, &token_stream)?
                },
                Token::LongName(ref name) if name == defined_long_name => {
                    self.extract_values(&values_begin_index, &token_stream)?
                },
                _ => token_stream_index.physical_index,
            };
        }

        Ok(resulting_index)
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
            Range { min: min_count, max: max_count } if available_values >= min_count => {
                println!("Range min {} max {}", min_count, max_count);
                self.matched_values = Some(copy_contigous_values(&token_stream[token_stream_index..], &max_count));
                if available_values > max_count {
                    new_token_stream_index += max_count;
                } else {
                    new_token_stream_index += available_values;
                }
            },
            Range {..} => {
                println!("Other range...");
                return Err("To few arguments");
            },
            _ => return Err("Invalid value count specification"),
        }

        Ok(new_token_stream_index)
    }
}

#[cfg(test)]
mod test {
    use super::Arg;
    use super::IndexPair;

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
            Some(_) => panic!("Matched, but match count is zero"),
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

        let mut argument_1st = Arg::with_name("Index 0")
                        .with_help("Index 0")
                        .on_index(0)
                        .takes_one_value()
                        .build();
        let mut argument_2nd = Arg::with_name("Index 1")
                        .with_help("Index 1")
                        .on_index(1)
                        .takes_one_value()
                        .build();
        let resulting_index_1st = argument_1st.take_tokens_at_index(&token_stream, &IndexPair::with_phys(0)).unwrap().physical_index;
        check_resulting_index(1, resulting_index_1st);
        check_match_result(&argument_1st, "val1");

        let resulting_index_2nd = argument_2nd
            .take_tokens_at_index(&token_stream, &IndexPair { physical_index: 1, logical_index: 1 }).unwrap().physical_index;
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

        let mut argument = Arg::with_name("Index 0")
                        .with_help("Index 0")
                        .on_index(0)
                        .takes_min_values(2)
                        .build();
        let resulting_index = argument.take_tokens_at_index(&token_stream, &IndexPair::with_phys(0)).unwrap().physical_index;
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

        let mut argument = Arg::with_name("Index 0")
                        .with_help("Index 0")
                        .on_index(0)
                        .takes_min_values(2)
                        .build();
        argument.take_tokens_at_index(&token_stream, &IndexPair::with_phys(0)).unwrap().physical_index;
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

        let mut argument = Arg::with_name("Index 0")
                        .with_help("Index 0")
                        .on_index(0)
                        .takes_max_values(2)
                        .build();
        let resulting_index = argument.take_tokens_at_index(&token_stream, &IndexPair::with_phys(0)).unwrap().physical_index;
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
    fn take_tokens_at_index__onindex_minmaxcount__less_avail_than_max__matches() {
        use super::Arg;
        use super::super::tokens;

        let argument_list = vec!(
            String::from("val1"),
            String::from("val2"),
            String::from("val3"));
        let token_stream = tokens::tokenize(&argument_list);

        let mut argument = Arg::with_name("Index 0")
                        .with_help("Index 0")
                        .on_index(0)
                        .takes_min_max_values(1, 2)
                        .build();
        let resulting_index = argument.take_tokens_at_index(&token_stream, &IndexPair::with_phys(0)).unwrap().physical_index;
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
    fn take_tokens_at_index__onindex_minmaxcount__more_avail_than_max__matches() {
        use super::Arg;
        use super::super::tokens;

        let argument_list = vec!(
            String::from("val1"),
            String::from("val2"),
            String::from("val3"),
            String::from("val4"));
        let token_stream = tokens::tokenize(&argument_list);

        let mut argument = Arg::with_name("Index 0")
                        .with_help("Index 0")
                        .on_index(0)
                        .takes_min_max_values(1, 3)
                        .build();
        let resulting_index = argument.take_tokens_at_index(&token_stream, &IndexPair::with_phys(0)).unwrap().physical_index;
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
    fn take_tokens_at_index__onindex_minmaxcount___to_few_avail___panics() {
        use super::Arg;
        use super::super::tokens;

        let argument_list = vec!(
            String::from("val1"));
        let token_stream = tokens::tokenize(&argument_list);

        let mut argument = Arg::with_name("Index 0")
                        .with_help("Index 0")
                        .on_index(0)
                        .takes_min_max_values(2, 3)
                        .build();
        argument.take_tokens_at_index(&token_stream, &IndexPair::with_phys(0)).unwrap().physical_index;
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

        let mut argument_1st_option = Arg::with_name("Opt 1")
                        .with_help("Opt 1")
                        .as_option("-o", "--option")
                        .takes_one_value()
                        .build();
        let mut argument_2nd_option = Arg::with_name("Opt 2")
                        .with_help("Opt 2")
                        .as_option("-p", "--option2")
                        .takes_one_value()
                        .build();

        let resulting_index = argument_1st_option
            .take_tokens_at_index(&token_stream, &IndexPair::with_phys(2))
            .unwrap().physical_index;
        check_resulting_index(4, resulting_index);
        check_match_result(&argument_1st_option, "optval1");

        let resulting_index = argument_2nd_option
            .take_tokens_at_index(&token_stream, &IndexPair::with_phys(4))
            .unwrap().physical_index;
        check_resulting_index(6, resulting_index);
        check_match_result(&argument_2nd_option, "optval2");
    }
}
