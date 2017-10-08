use args::Count;
use args::ArgDefinition;
use args::ArgValue;
use tokens::Token;


pub struct ArgDefinitionBuilder {
    name: &'static str,
    count: Count,
    interprete_tokens: Option<Box<for<'a> Fn(&'a [Token], usize, &'static str, &Count) -> Result<(&'a [Token], usize, Option<ArgValue>), &'static str>>>
}

pub fn an_arg_called(name: &'static str) -> ArgDefinitionBuilder {
    ArgDefinitionBuilder {
        name: name, count: Count::Fixed(0), interprete_tokens: None
    }
}

impl ArgDefinitionBuilder {
    pub fn with_count(mut self, count: Count) -> Self {
        self.count = count;
        self
    }

    pub fn on_index(mut self, index: usize) -> Self {
        use super::positional_arg_interpreter::interprete_positional_arg;

        self.interprete_tokens = Some(Box::new(
            move |stream, current_index, name, count| interprete_positional_arg(name, count, index.clone(), stream, current_index)));
        self
    }

    pub fn as_option(mut self, short_name: &'static str, long_name: &'static str) -> Self {
        use super::optional_arg_interpreter::interprete_optional_arg;

        self.interprete_tokens = Some(Box::new(
            move |stream, current_index, name, count| interprete_optional_arg(name, count, &String::from(short_name), &String::from(long_name), stream, current_index)));
        self
    }

    pub fn assembled(self) -> ArgDefinition {
        ArgDefinition {
            name: self.name,
            count: self.count,
            interprete_tokens: self.interprete_tokens.unwrap()
        }
    }
}
