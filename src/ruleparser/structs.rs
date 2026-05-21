use std::collections::{HashMap, HashSet};

use indexmap::IndexSet;


////////////////////////////////////////////////////////////////
/// TOKENS
////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Debug)]
pub struct Sym(usize);

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Debug)]
pub enum Token {
    Term(Sym),
    NonTerm(Sym)
}

impl Token {
    pub fn id(&self) -> usize
    {
        match self {
            Token::Term(t) => t.0,
            Token::NonTerm(t) => t.0,
        }
    }
}

////////////////////////////////////////////////////////////////
/// PRODUCTION
////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Debug)]
pub struct Production {
    nt: Token,
    inputs: Vec<Token>,
}

pub struct RawProduction {
    nt: String,
    inputs: Vec<String>,
}

impl RawProduction {
    pub fn new<T: Into<String>>(nt: impl Into<String>, inputs: Vec<T>) -> Self {
        Self {
            nt: nt.into(),
            inputs: inputs.into_iter().map(|x| x.into()).collect(),
        }
    }
}

////////////////////////////////////////////////////////////////
/// HANDLER
////////////////////////////////////////////////////////////////

#[derive(Default, Debug)]
pub struct TokenManager {
    token: HashMap<String, usize>,
    non_terminal_token: HashSet<usize>,
    productions: IndexSet<Production>,
}


impl TokenManager {
    pub fn new() -> Self
    {
        Self::default()
    }

    pub fn get_token(&self, str: &String) -> Token
    {
        if let Some(id) = self.token.get(str) {
            if self.non_terminal_token.get(&id).is_some() {
                return Token::NonTerm(Sym(*id));
            }
            return  Token::Term(Sym(*id));
        }
        panic!("The token cannot be retrieved, it does not exists");
    }

    pub fn add_token(&mut self, str: &String) -> &mut Self
    {
        let default_val = self.token.len();
        let _ = *self.token.entry(str.to_string()).or_insert(default_val);
        return self;
    }

    fn create_production(&self, prod: &RawProduction) -> Production
    {
        let prod = Production {
            nt: self.get_token(&prod.nt),
            inputs: prod.inputs.iter().map(|x| {
                self.get_token(x)
            }).collect()
        };
        return prod;
    }

    pub fn add_production(&mut self, prod: &RawProduction) -> &mut Self
    {
        self.add_token(&prod.nt);
        for elem in prod.inputs.iter() {
            self.add_token(elem);
        }
        self.non_terminal_token.insert(self.get_token(&prod.nt).id());
        let new_prod = self.create_production(prod);
        self.productions.insert(new_prod);
        return self
    }

    pub fn add_productions(&mut self, prod: &Vec<RawProduction>) -> &mut Self
    {
        for elem in prod.iter() {
            self.add_production(elem);
        }
        self
    }
}


////////////////////////////////////////////////////////////////
/// DISPLAY
////////////////////////////////////////////////////////////////


use std::fmt;

impl fmt::Display for TokenManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "TokenManager {{")?;

        writeln!(f, "  tokens:")?;
        for (name, id) in &self.token {
            let kind = if self.non_terminal_token.contains(id) {
                "non-terminal"
            } else {
                "terminal"
            };

            writeln!(f, "    [{id}] {name} ({kind})")?;
        }

        writeln!(f, "  productions:")?;
        for (i, prod) in self.productions.iter().enumerate() {
            writeln!(f, "    ({i}) {prod}")?;
        }

        write!(f, "}}")
    }
}

impl fmt::Display for Production {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ->", self.nt)?;

        for tok in &self.inputs {
            write!(f, " {}", tok)?;
        }

        Ok(())
    }
}

impl fmt::Display for Sym {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Term(t) => write!(f, "T{t}"),
            Token::NonTerm(nt) => write!(f, "N{nt}"),
        }
    }
}
