use tokens::Token;

pub mod builder;
mod positional_arg_interpreter;
mod optional_arg_interpreter;
mod extract_values;

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
    pub interprete_tokens: Box<for<'a> Fn(&'a [Token], usize, &'static str, &Count) -> Result<(&'a [Token], usize, Option<ArgValue>), &'static str>>
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
        interpreter: Box<for<'a> Fn(&'a [Token], usize, &'static str, &Count)
            -> Result<(&'a [Token], usize, Option<ArgValue>), &'static str>>) -> ArgDefinition {
        ArgDefinition { name: name, count: count, interprete_tokens: interpreter }
    }
}

impl ArgValue {
    pub fn new(name: &'static str, occurences: usize, assigned_values: Vec<String>) -> Self {
        ArgValue { name: name, occurences: occurences, assigned_values: assigned_values }
    }
}

pub fn merged_args(mut arg_values: Vec<ArgValue>, mut merged_in: ArgValue) -> Vec<ArgValue> {
    let already_occured = if let Some(ref mut found_arg) = arg_values.iter_mut().find(|ref arg| arg.name == merged_in.name) {
        found_arg.occurences += 1;
        found_arg.assigned_values.append(&mut merged_in.assigned_values);
        true
    } else {
        false
    };

    if !already_occured {
        arg_values.push(merged_in);
    }

    arg_values
}

use std::fmt;

impl fmt::Debug for ArgDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ArgDefinition {{ called: {} }}", self.name)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}
