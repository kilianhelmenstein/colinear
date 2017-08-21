pub use self::option_processor::OptionProcessor;
pub use self::positional_processor::PositionalProcessor;

mod option_processor;
mod positional_processor;

use super::super::tokens::*;
use super::IndexPair;

pub enum Count {
    Fixed(u32),
    Minimum(u32),
    Maximum(u32),
    Range { min: u32, max: u32 }
}

pub trait Processor {
    fn process_tokens_from_index(&mut self, token_stream: &[Token], token_stream_index: &IndexPair, specified_count: &Count) -> Result<(Vec<String>, IndexPair), &'static str>;

    fn extract_values(&mut self, token_stream_index: &u32, token_stream: &[Token], specified_count: &Count) -> Result<(Vec<String>, u32), &'static str> {
        use self::Count::*;

        let mut matched_values = Vec::new();
        let token_stream_index = *token_stream_index as usize;
        let mut new_token_stream_index = token_stream_index.clone() as u32;

        let available_values = count_available_contigous_values(&token_stream[token_stream_index..]);
        println!("avail. values: {}", available_values);
        match *specified_count {
            Fixed(fixed_count) if available_values >= fixed_count => {
                matched_values = copy_contigous_values(&token_stream[token_stream_index..], &fixed_count);
                new_token_stream_index += fixed_count;
            },
            Minimum(min_count) if available_values >= min_count => {
                matched_values = copy_all_contigous_values(&token_stream[token_stream_index..]);
                new_token_stream_index += available_values;
            },
            Minimum(_) => return Err("To few arguments"),
            Maximum(max_count) => {
                matched_values = copy_contigous_values(&token_stream[token_stream_index..], &max_count);
                if available_values > max_count {
                    new_token_stream_index += max_count;
                } else {
                    new_token_stream_index += available_values;
                }
            },
            Range { min: min_count, max: max_count } if available_values >= min_count => {
                println!("Range min {} max {}", min_count, max_count);
                matched_values = copy_contigous_values(&token_stream[token_stream_index..], &max_count);
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

        Ok((matched_values, new_token_stream_index))
    }
}
