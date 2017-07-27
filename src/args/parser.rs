use super::args::*;

struct AppMeta {
    app_name: &'static str,
    author_name: &'static str,
    author_email: &'static str
}

pub struct Parser {
    meta: AppMeta,
    configured_args: Vec<Arg>
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            meta: AppMeta { app_name: "", author_name: "", author_email: "" },
            configured_args: Vec::new()
        }
    }

    pub fn app(mut self, name: &'static str) -> Parser {
        self.meta.app_name = name;
        self
    }

    pub fn with_author(mut self, name: &'static str, email: &'static str) -> Parser {
        self.meta.author_name = name;
        self.meta.author_email = email;
        self
    }

    pub fn with_arg(mut self, mut argument: Arg) -> Parser {
        self.configured_args.push(argument);
        self
    }

    pub fn parse(&mut self) {
        
    }
}
