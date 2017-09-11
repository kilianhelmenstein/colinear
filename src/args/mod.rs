use tokens::Token;

#[derive(Clone)]
pub enum Count {
    Fixed(u32),
    Minimum(u32),
    Maximum(u32),
    Range { min: u32, max: u32 }
}

pub struct Arg {
    definition: ArgDefinition,
    value: ArgValue
}

pub struct ArgDefinition {
    pub count: Count,
    pub interprete_tokens: Box<Fn(Box<Iterator<Item=Token>>, usize, Count) -> (Box<Iterator<Item=Token>>, usize, Option<ArgValue>)>
}

pub struct ArgValue {
    occurences: usize,
    assigned_values: Vec<String>
}

impl ArgDefinition {
    pub fn new(count: Count, interpreter: Box<Fn(Box<Iterator<Item=Token>>, usize, Count) -> (Box<Iterator<Item=Token>>, usize, Option<ArgValue>)>) -> ArgDefinition {
        ArgDefinition { count: count, interprete_tokens: interpreter }
    }

}

impl ArgValue {
    pub fn new(occurences: usize, assigned_values: Vec<String>) -> Self {
        ArgValue { occurences: occurences, assigned_values: assigned_values }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}
