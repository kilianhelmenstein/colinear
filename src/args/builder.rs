use super::ArgDefinition;
use super::Count;


use super::optional_arg_interpreter;

pub struct ArgDefinitionBuilder {
    name: &'static str,
    count: Option<Count>,
    interprete_tokens: Option<Box<for<'a> Fn(&'a [Token], usize, &'static str, &Count) -> Result<(&'a [Token], usize, Option<ArgValue>), &'static str>>>
}

impl ArgDefinitionBuilder {
    fn Called(name: &'static str) -> Self {
        ArgDefinitionBuilder {
            name: name, count: None, interprete_tokens: None
        }
    }

    fn with_count(self, count: Count) -> Self {
        self.count = Some(count);
        self
    }

    fn on_index(self, index: usize) -> Self {
        use super::positional_arg_interpreter::interprete_positional_arg;

        self.interprete_tokens = |stream, current_index, name, count| interprete_positional_arg(name, count, index, stream, current_index);
        self
    }
}
