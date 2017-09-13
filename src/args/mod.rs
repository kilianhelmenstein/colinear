use tokens::Token;

#[derive(Clone)]
pub enum Count {
    Fixed(u32),
    Minimum(u32),
    Maximum(u32),
    Range { min: u32, max: u32 }
}

pub struct ArgDefinition {
    pub name: &'static str,
    pub count: Count,
    pub interprete_tokens: Box<Fn(Box<Iterator<Item=Token>>, usize, &'static str, &Count)
        -> Result<(Box<Iterator<Item=Token>>, usize, Option<ArgValue>), &'static str>>
}

pub struct ArgValue {
    pub name: &'static str,
    occurences: usize,
    assigned_values: Vec<String>
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
