use super::Processor;
use super::super::super::tokens::*;
use super::super::IndexPair;
use super::Count;

pub struct OptionProcessor {
    short_name: &'static str,
    long_name: &'static str,
}

impl OptionProcessor {
    pub fn new(short_name: &'static str, long_name: &'static str) -> OptionProcessor {
        OptionProcessor { short_name: short_name, long_name: long_name }
    }
}

impl Processor for OptionProcessor {
    fn process_tokens_from_index(&mut self, token_stream: &[Token], token_stream_index: &IndexPair, specified_count: &Count) -> Result<(Vec<String>, IndexPair), &'static str> {
        let mut resulting_index = token_stream_index.clone();
        let values_begin_index = token_stream_index.physical + 1;

        let (matched_values, new_physical_index) = match token_stream[token_stream_index.physical as usize] {
            Token::ShortName(ref name) if name == self.short_name => {
                self.extract_values(&values_begin_index, &token_stream, specified_count)?
            },
            Token::LongName(ref name) if name == self.long_name => {
                self.extract_values(&values_begin_index, &token_stream, specified_count)?
            },
            _ => (Vec::new(), token_stream_index.physical),
        };

        resulting_index.physical = new_physical_index;
        Ok((matched_values, resulting_index))
    }
}
