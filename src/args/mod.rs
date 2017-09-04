use tokens::Token;

type ProcessTokensFn<'a, S> = &'a Fn(S, &usize, &Count) -> Result<(S, usize, Option<ArgValue>), &'static str>;

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
    pub interprete_tokens: ProcessTokensFn<'a, &'a Iterator<Item=Token>>
}

pub struct ArgValue {
    occurences: usize,
    assigned_values: Vec<String>
}

impl <'a> ArgDefinition <'a> {
    pub fn new(count: &Count, interprete_tokens: ProcessTokensFn<'a, &'a Iterator<Item=Token>>) -> Self {
        ArgDefinition { count: count.clone(), interprete_tokens: interprete_tokens }
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
