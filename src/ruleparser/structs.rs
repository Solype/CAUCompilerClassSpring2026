use std::collections::{HashMap, HashSet};

use super::rules_and_tokens::*;
use indexmap::IndexSet;

////////////////////////////////////////////////////////////////
/// HANDLER
////////////////////////////////////////////////////////////////

pub struct TokenManager {
    pub token: HashMap<String, usize>,
    pub non_terminal_token: HashSet<usize>,
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

    /**
     * Resolves a token name into its internal parser representation.
     *
     * Returns:
     *   - `Token::Term`     for terminal symbols
     *   - `Token::NonTerm`  for non-terminal symbols
     *
     * Fails if the token name is unknown.
     */
    pub fn get_token(&self, str: &String) -> Result<Token, String> {
        if let Some(id) = self.token.get(str) {
            if self.non_terminal_token.get(&id).is_some() {
                return Ok(Token::NonTerm(Sym(*id).to_non_term()));
            }
            return Ok(Token::Term(Sym(*id).to_term()));
        }
        return Err(format!("the token '{}' does not exists", str));
    }

    /**
     * Returns the textual name associated with a token ID.
     *
     * If the ID does not exist in the token table,
     * a placeholder string is returned instead.
     */
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

    /**
     * Registers a new terminal token if it does not already exist.
     *
     * Token IDs are assigned incrementally based on insertion order.
     */
    pub fn add_token(&mut self, str: &String) -> &mut Self {
        let default_val = self.token.len();
        let _ = *self.token.entry(str.to_string()).or_insert(default_val);
        return self;
    }

    /**
     * Converts a raw grammar production into its internal parser representation.
     *
     * Every token name is resolved into its corresponding grammar symbol.
     *
     * Returns an error if:
     *   - the left-side non-terminal is unknown
     *   - one of the production inputs does not exist
     */
    fn create_production(&self, prod: &RawProduction) -> Result<SimpleProduction, String> {
        let nt = self
            .get_token(&prod.nt)
            .map_err(|_| format!("Unknown token '{}' in production left side", prod.nt))?
            .sym();

        let mut inputs = Vec::new();

        for token in &prod.inputs {
            let sym = self
                .get_token(token)
                .map_err(|_| {
                    format!(
                        "Unknown token '{}' in production '{} -> {}'",
                        token,
                        prod.nt,
                        prod.inputs.join(" ")
                    )
                })?
                .sym();

            inputs.push(sym);
        }

        Ok(SimpleProduction { nt, inputs })
    }

    /**
     * Registers a new grammar production into the parser.
     *
     * This function:
     *   - creates missing tokens
     *   - marks the left-side token as a non-terminal
     *   - converts the raw production into parser symbols
     *   - inserts the production into the grammar set
     *
     * Returns an error if symbol resolution fails.
     */
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

    /**
     * Registers multiple grammar productions into the parser.
     *
     * Productions are inserted sequentially using `add_production`.
     *
     * Stops and returns the first encountered error.
     */
    pub fn add_productions(&mut self, prod: &Vec<RawProduction>) -> Result<&mut Self, String> {
        for elem in prod.iter() {
            self.add_production(elem)?;
        }
        Ok(self)
    }

    /**
     * Builds and returns the finalized parser production set.
     *
     * Internal grammar representations are converted into parser-ready
     * productions using the current non-terminal table.
     */
    pub fn get_production(&self) -> IndexSet<Production> {
        self.productions
            .iter()
            .map(|x| x.to_production(&self.non_terminal_token))
            .collect::<IndexSet<Production>>()
    }

    /**
     * Splits a whitespace-separated token stream into raw tokens.
     *
     * Each extracted token is associated with positional metadata
     * including line number, column, original lexeme, and source line.
     *
     * This scanner assumes tokens are already separated by spaces or tabs.
     */
    pub fn scan_tokens(&self, buffer: &String) -> Result<Vec<(String, TokenMetadata)>, String> {
        let mut tokens: Vec<(String, TokenMetadata)> = vec![];

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
                    line: line_buffer.to_string(),
                };

                tokens.push((raw_token.to_string(), metadata));

                col += raw_token.len() + 1;
            }
        }

        Ok(tokens)
    }

    /**
     * Converts a raw token into a parser terminal symbol.
     *
     * The token name is resolved using the token table and validated
     * to ensure it is a terminal symbol.
     *
     * Returns a formatted lexer/parser diagnostic if:
     *   - the token does not exist
     *   - the token is not terminal
     */
    fn wrap_single_token(
        &self,
        unwrapped_token: &(String, TokenMetadata),
    ) -> Result<(Term, TokenMetadata), String> {
        let token = self.get_token(&unwrapped_token.0).map_err(|_| {
            file_error(
                &"Tokens".to_string(),
                unwrapped_token.1.span.0,
                unwrapped_token.1.span.1,
                unwrapped_token.1.str.len(),
                &unwrapped_token.1.line,
                &format!(
                    "Unknown token '{}', it can be either the regex or your typing",
                    unwrapped_token.0
                ),
            )
        })?;

        let Token::Term(term) = token else {
            return Err(file_error(
                &"Tokens".to_string(),
                unwrapped_token.1.span.0,
                unwrapped_token.1.span.1,
                unwrapped_token.1.str.len(),
                &unwrapped_token.1.str,
                &format!("'{}' is not a terminal token", unwrapped_token.0),
            ));
        };
        return Ok((term, unwrapped_token.1.clone()));
    }

    /**
     * Converts lexer tokens into parser terminal symbols.
     *
     * Every token is validated and resolved against the parser token table.
     * An explicit END (`$`) token is automatically appended to the stream.
     *
     * The END token metadata is positioned immediately after the last token
     * of the input, or at `(1, 1)` for an empty stream.
     *
     * Returns the first encountered token conversion error.
     */
    pub fn wrap_cooked_token(
        &self,
        cooked_tokens: &Vec<(String, TokenMetadata)>,
    ) -> Result<Vec<(Term, TokenMetadata)>, String> {
        let mut wrapped_token = cooked_tokens
            .iter()
            .map(|x| self.wrap_single_token(x))
            .collect::<Result<Vec<_>, _>>()?;

        let end_metadata = if let Some((_, last_meta)) = wrapped_token.last() {
            TokenMetadata {
                span: (last_meta.span.0, last_meta.span.1 + last_meta.str.len()),
                str: "$".to_string(),
                line: last_meta.line.clone(),
            }
        } else {
            TokenMetadata {
                span: (1, 1),
                str: "$".to_string(),
                line: "".to_string(),
            }
        };

        wrapped_token.push((END, end_metadata));

        Ok(wrapped_token)
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
