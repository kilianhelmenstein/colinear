use super::Processor;
use super::super::super::tokens::*;
use super::super::IndexPair;
use super::Count;

pub struct PositionalProcessor {
    logical_index: u32
}

impl PositionalProcessor {
    pub fn new(logical_index: u32) -> PositionalProcessor {
        PositionalProcessor { logical_index: logical_index }
    }
}

impl Processor for PositionalProcessor {
    fn process_tokens_from_index(&mut self, token_stream: &[Token], token_stream_index: &IndexPair, specified_count: &Count) -> Result<(Vec<String>, IndexPair), &'static str> {
        let stream_is_on_right_position = token_stream_index.logical == self.logical_index;

        if stream_is_on_right_position {
            let mut new_index = token_stream_index.clone();

            let (matched_values, new_physical_index) = self.extract_values(&token_stream_index.physical, token_stream, specified_count)?;
            new_index.physical = new_physical_index;

            let took_tokens = new_index.physical > token_stream_index.physical;
            if took_tokens {
                new_index.logical += 1;
                return Ok((matched_values, new_index));
            }
        }

        Ok((Vec::new(), token_stream_index.clone()))
    }
}
