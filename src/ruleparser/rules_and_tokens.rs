use std::collections::HashSet;

////////////////////////////////////////////////////////////////
/// TOKENS
////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
pub struct Sym(pub usize);

impl Sym {
    pub(super) fn to_term(&self) -> Term {
        Term(self.clone())
    }

    pub(super) fn to_non_term(&self) -> NonTerm {
        NonTerm(self.clone())
    }
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Debug)]
pub struct NonTerm(pub Sym);

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Debug)]
pub struct Term(pub Sym);

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
pub(super) struct SimpleProduction {
    pub(super) nt: Sym,
    pub(super) inputs: Vec<Sym>,
}

impl SimpleProduction {
    pub(super) fn to_production(&self, nt_set: &HashSet<usize>) -> Production {
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
    pub(super) nt: String,
    pub(super) inputs: Vec<String>,
}

impl RawProduction {
    pub fn new<T: Into<String>>(nt: impl Into<String>, inputs: Vec<T>) -> Self {
        Self {
            nt: nt.into(),
            inputs: inputs.into_iter().map(|x| x.into()).collect(),
        }
    }
}
