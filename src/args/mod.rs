use tokens::Token;

mod positional_arg_interpreter;
mod optional_arg_interpreter;

#[derive(Clone)]
pub enum Count {
    Fixed(usize),
    Minimum(usize),
    Maximum(usize),
    Range { min: usize, max: usize }
}

pub struct ArgDefinition {
    pub name: &'static str,
    pub count: Count,
    pub interprete_tokens: Box<Fn(Box<Iterator<Item=Token>>, usize, &'static str, &Count)
        -> Result<(Box<Iterator<Item=Token>>, usize, Option<ArgValue>), &'static str>>
}

pub struct ArgValue {
    pub name: &'static str,
    pub occurences: usize,
    pub assigned_values: Vec<String>
}

impl ArgDefinition {
    pub fn new(
        name: &'static str,
        count: Count,
        interpreter: Box<Fn(Box<Iterator<Item=Token>>, usize, &'static str, &Count)
            -> Result<(Box<Iterator<Item=Token>>, usize, Option<ArgValue>), &'static str>>) -> ArgDefinition {
        ArgDefinition { name: name, count: count, interprete_tokens: interpreter }
    }
}

impl ArgValue {
    pub fn new(name: &'static str, occurences: usize, assigned_values: Vec<String>) -> Self {
        ArgValue { name: name, occurences: occurences, assigned_values: assigned_values }
    }
}

pub fn merged_args(mut arg_values: Vec<ArgValue>, merged_in: ArgValue) -> Vec<ArgValue> {
    arg_values.push(merged_in);
    arg_values
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}
