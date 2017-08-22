mod processor;

use tokens;
use tokens::*;
use self::processor::*;

pub struct ArgConfig {
    meta: Meta,
    token_processor: Box<Processor>,
    required_values_specification: Count,

}

pub struct Meta {
    pub name: &'static str,
    pub help: &'static str
}

enum Type {
    OnIndex { index: u32 },
    AsOption { short_name: &'static str, long_name: &'static str }
}

pub struct IndexPair {
    pub physical: u32,
    pub logical: u32
}

impl Clone for IndexPair {
    fn clone(&self) -> Self {
        IndexPair { physical: self.physical, logical: self.physical }
    }
}

impl IndexPair {
    pub fn zero_indeces() -> IndexPair {
        IndexPair { physical: 0, logical: 0 }
    }

    fn with_phys(physical: u32) -> IndexPair {
        IndexPair { physical: physical, logical: 0 }
    }
}

pub struct ArgConfigBuilder {
    built_arg: ArgConfig
}

impl ArgConfigBuilder {
    fn new() -> ArgConfigBuilder {
        ArgConfigBuilder {
            built_arg: ArgConfig {
                meta: Meta { name: "", help: "" },
                token_processor: Box::new(OptionProcessor::new("", "")),
                required_values_specification: Count::Fixed(0),
                matched_values: None
            }
        }
    }

    pub fn build(self) -> ArgConfig {
        self.built_arg
    }

    pub fn with_name(mut self, name: &'static str) -> ArgConfigBuilder {
        self.built_arg.meta.name = name;
        self
    }

    pub fn with_help(mut self, help: &'static str) -> ArgConfigBuilder {
        self.built_arg.meta.help = help;
        self
    }

    pub fn on_index(mut self, index: u32) -> ArgConfigBuilder {
        self.built_arg.token_processor = Box::new(PositionalProcessor::new(index));
        self
    }

    pub fn as_option(mut self, short_name: &'static str, long_name: &'static str) -> ArgConfigBuilder {
        self.built_arg.token_processor = Box::new(OptionProcessor::new(short_name, long_name));
        self
    }

    pub fn takes_one_value(mut self) -> ArgConfigBuilder {
        self.built_arg.required_values_specification = Count::Fixed(1);
        self
    }

    pub fn takes_n_values(mut self, n: u32) -> ArgConfigBuilder {
        self.built_arg.required_values_specification = Count::Fixed(n);
        self
    }

    pub fn takes_min_values(mut self, min: u32) -> ArgConfigBuilder {
        self.built_arg.required_values_specification = Count::Minimum(min);
        self
    }

    pub fn takes_max_values(mut self, max: u32) -> ArgConfigBuilder {
        self.built_arg.required_values_specification = Count::Maximum(max);
        self
    }

    pub fn takes_min_max_values(mut self, min: u32, max: u32) -> ArgConfigBuilder {
        self.built_arg.required_values_specification = Count::Range { min: min, max: max };
        self
    }
}

impl ArgConfig {
    pub fn with_name(name: &'static str) -> ArgConfigBuilder {
        let builder = ArgConfigBuilder::new();
        builder.with_name(name)
    }

    pub fn name(&self) -> String {
        String::from(self.meta.name)
    }

    pub fn help(&self) -> String {
        String::from(self.meta.help)
    }

    pub fn take_tokens_at_index(&mut self, token_stream: &[Token], token_stream_index: &IndexPair) -> Result<IndexPair, &'static str> {
        match self.token_processor.process_tokens_from_index(token_stream, token_stream_index, &self.required_values_specification) {
            Ok((matched_values, resulting_index)) => {
                self.matched_values = Some(matched_values);
                return Ok(resulting_index);
            },
            Err(message) => Err(message)
        }
    }
}

pub struct ArgResult {
    occurences: isize;
    values: Vec<String>
}

impl ArgResult {
    pub fn new() -> ArgResult {
        ArgResult {
            occurences: 0,
            values: Vec::new()
        }
    }

    pub fn merge(&self, other: &ArgResult) -> ArgResult {
        ArgResult {
            occurences: self.occurences + other.occurences,
            values: self.values + other.values
        }
    }
}


#[cfg(test)]
mod test {
    use super::ArgConfig;
    use super::IndexPair;

    fn check_match_result(dut: &ArgConfig, shall_matched_value: &str) {
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
        use super::ArgConfig;
        use super::super::tokens;

        let argument_list = vec!(
            String::from("val1"),
            String::from("val2"));
        let token_stream = tokens::tokenize(&argument_list);

        let mut argument_1st = ArgConfig::with_name("Index 0")
                        .with_help("Index 0")
                        .on_index(0)
                        .takes_one_value()
                        .build();
        let mut argument_2nd = ArgConfig::with_name("Index 1")
                        .with_help("Index 1")
                        .on_index(1)
                        .takes_one_value()
                        .build();
        let resulting_index_1st = argument_1st.take_tokens_at_index(&token_stream, &IndexPair::with_phys(0)).unwrap().physical;
        check_resulting_index(1, resulting_index_1st);
        check_match_result(&argument_1st, "val1");

        let resulting_index_2nd = argument_2nd
            .take_tokens_at_index(&token_stream, &IndexPair { physical: 1, logical: 1 }).unwrap().physical;
        check_resulting_index(2, resulting_index_2nd);
        check_match_result(&argument_2nd, "val2");
    }

    #[test]
    fn take_tokens_at_index__onindex_mincount__matches() {
        use super::ArgConfig;
        use super::super::tokens;

        let argument_list = vec!(
            String::from("val1"),
            String::from("val2"),
            String::from("val3"));
        let token_stream = tokens::tokenize(&argument_list);

        let mut argument = ArgConfig::with_name("Index 0")
                        .with_help("Index 0")
                        .on_index(0)
                        .takes_min_values(2)
                        .build();
        let resulting_index = argument.take_tokens_at_index(&token_stream, &IndexPair::with_phys(0)).unwrap().physical;
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
        use super::ArgConfig;
        use super::super::tokens;

        let argument_list = vec!(
            String::from("val1"));
        let token_stream = tokens::tokenize(&argument_list);

        let mut argument = ArgConfig::with_name("Index 0")
                        .with_help("Index 0")
                        .on_index(0)
                        .takes_min_values(2)
                        .build();
        argument.take_tokens_at_index(&token_stream, &IndexPair::with_phys(0)).unwrap().physical;
    }

    #[test]
    fn take_tokens_at_index__onindex_maxcount__matches() {
        use super::ArgConfig;
        use super::super::tokens;

        let argument_list = vec!(
            String::from("val1"),
            String::from("val2"),
            String::from("val3"));
        let token_stream = tokens::tokenize(&argument_list);

        let mut argument = ArgConfig::with_name("Index 0")
                        .with_help("Index 0")
                        .on_index(0)
                        .takes_max_values(2)
                        .build();
        let resulting_index = argument.take_tokens_at_index(&token_stream, &IndexPair::with_phys(0)).unwrap().physical;
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
        use super::ArgConfig;
        use super::super::tokens;

        let argument_list = vec!(
            String::from("val1"),
            String::from("val2"),
            String::from("val3"));
        let token_stream = tokens::tokenize(&argument_list);

        let mut argument = ArgConfig::with_name("Index 0")
                        .with_help("Index 0")
                        .on_index(0)
                        .takes_min_max_values(1, 2)
                        .build();
        let resulting_index = argument.take_tokens_at_index(&token_stream, &IndexPair::with_phys(0)).unwrap().physical;
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
        use super::ArgConfig;
        use super::super::tokens;

        let argument_list = vec!(
            String::from("val1"),
            String::from("val2"),
            String::from("val3"),
            String::from("val4"));
        let token_stream = tokens::tokenize(&argument_list);

        let mut argument = ArgConfig::with_name("Index 0")
                        .with_help("Index 0")
                        .on_index(0)
                        .takes_min_max_values(1, 3)
                        .build();
        let resulting_index = argument.take_tokens_at_index(&token_stream, &IndexPair::with_phys(0)).unwrap().physical;
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
        use super::ArgConfig;
        use super::super::tokens;

        let argument_list = vec!(
            String::from("val1"));
        let token_stream = tokens::tokenize(&argument_list);

        let mut argument = ArgConfig::with_name("Index 0")
                        .with_help("Index 0")
                        .on_index(0)
                        .takes_min_max_values(2, 3)
                        .build();
        argument.take_tokens_at_index(&token_stream, &IndexPair::with_phys(0)).unwrap().physical;
    }

    #[test]
    fn take_tokens_at_index__asoption__matches() {
        use super::ArgConfig;
        use super::super::tokens;

        let arg_list = vec!(
            String::from("val1"),
            String::from("val2"),
            String::from("-o"),
            String::from("optval1"),
            String::from("--option2"),
            String::from("optval2"));

        let token_stream = tokens::tokenize(&arg_list);

        let mut argument_1st_option = ArgConfig::with_name("Opt 1")
                        .with_help("Opt 1")
                        .as_option("-o", "--option")
                        .takes_one_value()
                        .build();
        let mut argument_2nd_option = ArgConfig::with_name("Opt 2")
                        .with_help("Opt 2")
                        .as_option("-p", "--option2")
                        .takes_one_value()
                        .build();

        let resulting_index = argument_1st_option
            .take_tokens_at_index(&token_stream, &IndexPair::with_phys(2))
            .unwrap().physical;
        check_resulting_index(4, resulting_index);
        check_match_result(&argument_1st_option, "optval1");

        let resulting_index = argument_2nd_option
            .take_tokens_at_index(&token_stream, &IndexPair::with_phys(4))
            .unwrap().physical;
        check_resulting_index(6, resulting_index);
        check_match_result(&argument_2nd_option, "optval2");
    }
}
