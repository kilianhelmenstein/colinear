use tokens;
use tokens::*;

type InterpreteTokensFn<'a> = Fn(&'a [Token], &usize, &Count)
                            -> Result<(&'a [Token], usize, Option<ArgValue>), &'static str>;

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
    pub name: &'static str,
    pub help: &'static str,
    pub count: Count,
    pub interprete_tokens: InterpreteTokensFn
}

pub struct ArgValue {
    occurences: usize,
    assigned_values: Vec<String>
}

impl ArgDefinition {
    pub fn new(count: &Count, interprete_tokens: &InterpreteTokensFn) -> Self {
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
