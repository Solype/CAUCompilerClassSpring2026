use std::fmt::{self, write};
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
#[derive(Debug, Clone)]
pub struct Token {
    pub typ: TokenType,
    pub metadata: Option<Metadata>,
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
