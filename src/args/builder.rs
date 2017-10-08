use args::Count;
use args::ArgDefinition;
use args::ArgValue;
use tokens::Token;


pub struct ArgDefinitionBuilder {
    name: &'static str,
    count: Option<Count>,
    interprete_tokens: Option<Box<for<'a> Fn(&'a [Token], usize, &'static str, &Count) -> Result<(&'a [Token], usize, Option<ArgValue>), &'static str>>>
}

impl ArgDefinitionBuilder {
    fn called(name: &'static str) -> Self {
        ArgDefinitionBuilder {
            name: name, count: None, interprete_tokens: None
        }
    }

    fn with_count(mut self, count: Count) -> Self {
        self.count = Some(count);
        self
    }

    fn on_index(mut self, index: usize) -> Self {
        use super::positional_arg_interpreter::interprete_positional_arg;

        self.interprete_tokens = Some(Box::new(
            move |stream, current_index, name, count| interprete_positional_arg(name, count, index.clone(), stream, current_index)));
        self
    }

    fn as_option(mut self, short_name: String, long_name: String) -> Self {
        use super::optional_arg_interpreter::interprete_optional_arg;

        self.interprete_tokens = Some(Box::new(
            move |stream, current_index, name, count| interprete_optional_arg(name, count, &short_name, &long_name, stream, current_index)));
        self
    }

    fn assembled(mut self) -> ArgDefinition {
        ArgDefinition {
            name: self.name,
            count: self.count.unwrap(),
            interprete_tokens: self.interprete_tokens.unwrap()
        }
    }
}
