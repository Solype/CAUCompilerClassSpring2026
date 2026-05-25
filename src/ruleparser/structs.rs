use std::collections::{HashMap, HashSet};

use indexmap::IndexSet;
use super::rules_and_tokens::*;

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

        new_var.add_token(&"START".to_string());        // 0
        new_var.add_token(&"EPSILON".to_string());      // 1
        new_var.add_token(&"END".to_string());          // 2
        new_var.add_token(&"UNDEFINED".to_string());    // 3

        new_var
    }

    pub fn get_token(&self, str: &String) -> Result<Token, String> {
        if let Some(id) = self.token.get(str) {
            if self.non_terminal_token.get(&id).is_some() {
                return Ok(Token::NonTerm(Sym(*id).to_non_term()));
            }
            return Ok(Token::Term(Sym(*id).to_term()));
        }
        return Err(format!("the token '{}' does not exists", str));
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

    fn create_production(
        &self,
        prod: &RawProduction,
    ) -> Result<SimpleProduction, String> {

        let nt = self.get_token(&prod.nt).map_err(|_| {
                format!("Unknown token '{}' in production left side", prod.nt)
            })?.sym();

        let mut inputs = Vec::new();

        for token in &prod.inputs {
            let sym = self.get_token(token)
                .map_err(|_| {
                    format!( "Unknown token '{}' in production '{} -> {}'", token, prod.nt, prod.inputs.join(" ") )
                })?.sym();

            inputs.push(sym);
        }

        Ok(SimpleProduction {
            nt,
            inputs,
        })
    }

    pub fn add_production(&mut self, prod: &RawProduction) -> Result<&mut Self, String> {
        self.add_token(&prod.nt);
        for elem in prod.inputs.iter() {
            self.add_token(elem);
        }
        self.non_terminal_token
            .insert(self.get_token(&prod.nt)?.id());
        let new_prod = self.create_production(prod)?;
        self.productions.insert(new_prod);
        return Ok(self);
    }

    pub fn add_productions(&mut self, prod: &Vec<RawProduction>) -> Result<&mut Self, String> {
        for elem in prod.iter() {
            self.add_production(elem)?;
        }
        Ok(self)
    }

    pub fn get_production(&self) -> IndexSet<Production> {
        self.productions
            .iter()
            .map(|x| x.to_production(&self.non_terminal_token))
            .collect::<IndexSet<Production>>()
    }

    fn parse_token_in_line(&self, raw_token: &str, vec: &mut Vec<(Term, TokenMetadata)>)
    {
        if raw_token.is_empty() {
            return;
        }
    }

    pub fn scan_tokens(
        &self,
        buffer: &String,
    ) -> Result<Vec<(Term, TokenMetadata)>, String> {

        let mut tokens: Vec<(Term, TokenMetadata)> = vec![];

        let lines: Vec<&str> = buffer.split('\n').collect();

        for (line_idx, line_buffer) in lines.iter().enumerate() {

            let mut col = 1;

            for raw_token in line_buffer.split(&[' ', '\t']) {

                if raw_token.is_empty() {
                    col += 1;
                    continue;
                }

                let metadata = TokenMetadata {
                    span: (line_idx + 1, col),
                    str: raw_token.to_string(),
                    line: (*line_buffer).to_string(),
                };

                let token = self
                    .get_token(&raw_token.to_string())
                    .map_err(|_| {
                        file_error(
                            &"Tokens".to_string(),
                            line_idx + 1,
                            col,
                            raw_token.len(),
                            &line_buffer.to_string(),
                            &format!("Unknown token '{}'", raw_token),
                        )
                    })?;

                let Token::Term(term) = token else {
                    return Err(file_error(
                        &"Tokens".to_string(),
                        line_idx + 1,
                        col,
                        raw_token.len(),
                        &line_buffer.to_string(),
                        &format!("'{}' is not a terminal token", raw_token
                        ),
                    ));
                };

                tokens.push((term, metadata));

                col += raw_token.len() + 1;
            }
        }

        let last_line = lines.last().unwrap_or(&"");

        tokens.push((
            END,
            TokenMetadata {
                span: (lines.len(), last_line.len() + 1),
                str: "$".to_string(),
                line: (*last_line).to_string(),
            },
        ));

        Ok(tokens)
    }
}

////////////////////////////////////////////////////////////////
/// DISPLAY
////////////////////////////////////////////////////////////////
use std::fmt;

use crate::error::file_error;

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
