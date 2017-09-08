use tokens::Token;

#[derive(Clone)]
pub enum Count {
    Fixed(u32),
    Minimum(u32),
    Maximum(u32),
    Range { min: u32, max: u32 }
}

pub struct Arg<'a> {
    definition: ArgDefinition<'a>,
    value: ArgValue
}

pub struct ArgDefinition<'a> {
    pub count: Count,
    pub interprete_tokens: &'a for<'c> Fn(&'c Iterator<Item=Token>, usize, Count) -> Result<(&'c Iterator<Item=Token>, usize, Option<ArgValue>), &'static str>
}

pub struct ArgValue {
    occurences: usize,
    assigned_values: Vec<String>
}

impl <'a> ArgDefinition <'a> {
    pub fn new<'r>(count: Count, interprete_tokens: &'r for<'c> Fn(&'c Iterator<Item=Token>, usize, Count) -> Result<(&'c Iterator<Item=Token>, usize, Option<ArgValue>), &'static str>) -> Self {
        ArgDefinition { count: count, interprete_tokens: interprete_tokens }
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
