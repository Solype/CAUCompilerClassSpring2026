use std::fmt::{self};
use std::hash::Hash;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Branch {
    If,
    Else,
    While,
    Return,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ponctuation {
    Semi,
    Comma,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Nesting {
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    Vtype,
    Literal,
    Boolstr,
    Num,
    Character,
    Id,
    Class,
    Addsub,
    Multdiv,
    Assign,
    Comp,
    Branchs(Branch),
    Ponctuation(Ponctuation),
    Nesting(Nesting),
}

impl FromStr for TokenType {
    type Err = InvalidToken;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "vtype" => Ok(TokenType::Vtype),
            "num" => Ok(TokenType::Num),
            "character" => Ok(TokenType::Character),
            "boolstr" => Ok(TokenType::Boolstr),
            "literal" => Ok(TokenType::Literal),
            "id" => Ok(TokenType::Id),
            "if" => Ok(TokenType::Branchs(Branch::If)),
            "else" => Ok(TokenType::Branchs(Branch::Else)),
            "while" => Ok(TokenType::Branchs(Branch::While)),
            "return" => Ok(TokenType::Branchs(Branch::Return)),
            "class" => Ok(TokenType::Class),
            "addsub" => Ok(TokenType::Addsub),
            "multidiv" => Ok(TokenType::Multdiv),
            "assign" => Ok(TokenType::Assign),
            "comp" => Ok(TokenType::Comp),
            "semi" => Ok(TokenType::Ponctuation(Ponctuation::Semi)),
            "comma" => Ok(TokenType::Ponctuation(Ponctuation::Comma)),
            "lparen" => Ok(TokenType::Nesting(Nesting::Lparen)),
            "rparen" => Ok(TokenType::Nesting(Nesting::Rparen)),
            "lbrace" => Ok(TokenType::Nesting(Nesting::Lbrace)),
            "rbrace" => Ok(TokenType::Nesting(Nesting::Rbrace)),
            invalid => Err(InvalidToken(invalid.to_string())),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Metadata {
    pub span: (usize, usize),
    pub line_str: String,
}
impl fmt::Debug for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("").finish()
    }
}
#[derive(Clone)]
pub struct Token {
    pub typ: TokenType,
    pub metadata: Option<Metadata>,
}
impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.typ)
    }
}
impl From<TokenType> for Token {
    fn from(value: TokenType) -> Self {
        Self {
            typ: value,
            metadata: None,
        }
    }
}
impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.typ == other.typ
    }
}
impl Eq for Token {}
impl Hash for Token {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.typ.hash(state);
    }
}

#[derive(Debug)]
pub struct InvalidToken(pub String);

pub fn parse_token(buffer: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    let mut col;
    for (line, line_buffer) in buffer.split('\n').enumerate() {
        col = 0;
        let mut line_tokens = line_buffer
            .split(&[' ', '\t'])
            .map(|s| {
                col += 1;
                if s == "" {
                    return None;
                }

                let token = Token {
                    typ: TokenType::from_str(s).expect(
                        format!(
                            "Error while parsing token at {} {}:\n{}\n{}^\n",
                            line + 1,
                            col,
                            line_buffer,
                            " ".repeat(col - 1)
                        )
                        .as_str(),
                    ),
                    metadata: Some(Metadata {
                        span: (line + 1, col),
                        line_str: String::from(line_buffer),
                    }),
                };
                col += s.len();
                Some(token)
            })
            .flatten()
            .collect();
        tokens.append(&mut line_tokens);
    }

    tokens
}
