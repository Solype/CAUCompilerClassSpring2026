use std::collections::{HashMap, HashSet};

use indexmap::IndexSet;

////////////////////////////////////////////////////////////////
/// TOKENS
////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
pub struct Sym(usize);

impl Sym {
    fn to_term(&self) -> Term {
        Term(self.clone())
    }

    fn to_non_term(&self) -> NonTerm {
        NonTerm(self.clone())
    }
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Debug)]
pub struct NonTerm(Sym);

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Debug)]
pub struct Term(Sym);

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
pub enum Token {
    Term(Term),
    NonTerm(NonTerm),
}

impl Token {
    pub fn id(&self) -> usize {
        match self {
            Token::Term(t) => t.0.0,
            Token::NonTerm(t) => t.0.0,
        }
    }

    pub fn sym(&self) -> Sym {
        match self {
            Token::Term(t) => t.0.clone(),
            Token::NonTerm(t) => t.0.clone(),
        }
    }

    pub fn is_term(&self) -> bool {
        match self {
            Token::Term(_) => true,
            Token::NonTerm(_) => false,
        }
    }

    pub fn is_non_term(&self) -> bool {
        !self.is_term()
    }
}

#[derive(Clone, Debug)]
pub struct TokenMetadata {
    pub span: (usize, usize),
    pub str: String,
    pub line: String,
}
impl Default for TokenMetadata {
    fn default() -> Self {
        Self {
            span: (1, 1),
            str: "".to_string(),
            line: "".to_string(),
        }
    }
}

////////////////////////////////////////////////////////////////
/// PRODUCTION
////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
struct SimpleProduction {
    nt: Sym,
    inputs: Vec<Sym>,
}

impl SimpleProduction {
    fn to_production(&self, nt_set: &HashSet<usize>) -> Production {
        Production {
            nt: self.nt.to_non_term(),
            inputs: self
                .inputs
                .iter()
                .map(|x| {
                    if nt_set.get(&x.0).is_none() {
                        Token::Term(x.to_term())
                    } else {
                        Token::NonTerm(x.to_non_term())
                    }
                })
                .collect(),
        }
    }
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
pub struct Production {
    pub nt: NonTerm,
    pub inputs: Vec<Token>,
}

impl Production {
    pub fn new(nt: NonTerm, inputs: Vec<Token>) -> Self {
        Self { nt, inputs }
    }
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

pub struct TokenManager {
    token: HashMap<String, usize>,
    non_terminal_token: HashSet<usize>,
    productions: IndexSet<SimpleProduction>,
}

pub const START: NonTerm = NonTerm(Sym(0));
pub const EPSILON: Term = Term(Sym(1));
pub const END: Term = Term(Sym(2));
pub const UNDEFINED: Term = Term(Sym(3));

impl TokenManager {
    pub fn new() -> Self {
        let mut new_var = Self {
            token: HashMap::<String, usize>::new(),
            non_terminal_token: HashSet::<usize>::new(),
            productions: IndexSet::<SimpleProduction>::new(),
        };

        new_var.add_token(&"START".to_string()); // 0
        new_var.add_token(&"EPSILON".to_string()); // 1
        new_var.add_token(&"END".to_string()); // 2
        new_var.add_token(&"UNDEFINED".to_string()); // 3

        new_var
    }

    pub fn get_token(&self, str: &String) -> Token {
        if let Some(id) = self.token.get(str) {
            if self.non_terminal_token.get(&id).is_some() {
                return Token::NonTerm(Sym(*id).to_non_term());
            }
            return Token::Term(Sym(*id).to_term());
        }
        panic!("The token cannot be retrieved, it does not exists");
    }

    pub fn get_token_name(&self, id: usize) -> String {
        self.token
            .iter()
            .find_map(|(name, &value)| {
                if value == id {
                    Some(name.clone())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| format!("<unknown:{id}>"))
    }

    pub fn add_token(&mut self, str: &String) -> &mut Self {
        let default_val = self.token.len();
        let _ = *self.token.entry(str.to_string()).or_insert(default_val);
        return self;
    }

    fn create_production(&self, prod: &RawProduction) -> SimpleProduction {
        let prod = SimpleProduction {
            nt: self.get_token(&prod.nt).sym(),
            inputs: prod
                .inputs
                .iter()
                .map(|x| self.get_token(x).sym())
                .collect(),
        };
        return prod;
    }

    pub fn add_production(&mut self, prod: &RawProduction) -> &mut Self {
        self.add_token(&prod.nt);
        for elem in prod.inputs.iter() {
            self.add_token(elem);
        }
        self.non_terminal_token
            .insert(self.get_token(&prod.nt).id());
        let new_prod = self.create_production(prod);
        self.productions.insert(new_prod);
        return self;
    }

    pub fn add_productions(&mut self, prod: &Vec<RawProduction>) -> &mut Self {
        for elem in prod.iter() {
            self.add_production(elem);
        }
        self
    }

    pub fn get_production(&self) -> IndexSet<Production> {
        self.productions
            .iter()
            .map(|x| x.to_production(&self.non_terminal_token))
            .collect::<IndexSet<Production>>()
    }

    pub fn scan_tokens(&self, buffer: &String) -> Vec<(Term, TokenMetadata)> {
        let mut tokens: Vec<(Term, TokenMetadata)> = vec![];

        let mut col;
        let lines = buffer.split('\n');
        for (line, line_buffer) in lines.clone().enumerate() {
            col = 0;
            let mut line_tokens = line_buffer
                .split(&[' ', '\t'])
                .map(|s| {
                    col += 1;
                    if s == "" {
                        return None;
                    }

                    let metadata = TokenMetadata {
                        span: (line + 1, col),
                        str: s.to_string(),
                        line: String::from(line_buffer),
                    };
                    let Token::Term(term) = self.get_token(&s.to_string()) else {
                        panic!("Only non-term tokens can be used as input.")
                    };
                    col += s.len();

                    Some((term, metadata))
                })
                .flatten()
                .collect();
            tokens.append(&mut line_tokens);
        }

        let last_line = lines.clone().last().unwrap();
        tokens.push((
            END,
            TokenMetadata {
                span: (lines.count(), last_line.len() + 1),
                str: "$".to_string(),
                line: String::from(last_line),
            },
        ));

        tokens
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
            write!(f, "    ({i}) {} ->", self.get_token_name(prod.nt.0))?;
            for tok in prod.inputs.iter() {
                write!(f, " {}", self.get_token_name(tok.0))?;
            }
            writeln!(f, "")?;
        }

        write!(f, "}}")
    }
}

impl fmt::Debug for TokenManager {
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
            writeln!(f, "    ({i}) {:?}", prod)?;
        }

        write!(f, "}}")
    }
}

impl fmt::Debug for Production {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ->", self.nt.0)?;

        for tok in &self.inputs {
            write!(f, " {:?}", tok)?;
        }

        Ok(())
    }
}

impl fmt::Debug for SimpleProduction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ->", self.nt)?;

        for tok in &self.inputs {
            write!(f, " {:?}", tok)?;
        }

        Ok(())
    }
}

impl fmt::Debug for Sym {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Term(t) => write!(f, "T{:?}", t.0),
            Token::NonTerm(nt) => write!(f, "N{:?}", nt.0),
        }
    }
}
