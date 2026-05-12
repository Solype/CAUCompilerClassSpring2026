#[derive(Debug, Clone)]
pub enum Addsub {
    Add,
    Sub
}

#[derive(Debug, Clone)]
pub enum Multdiv {
    Mult,
    Div
}

#[derive(Debug, Clone)]
pub enum Branch {
    If,
    Else,
    While,
    Return
}

#[derive(Debug, Clone)]
pub enum Vtype {
    Float,
    Integer,
    Other(String)
}

#[derive(Debug, Clone)]
pub enum Ponctuation {
    Semi,
    Comma,
}

#[derive(Debug, Clone)]
pub enum Nesting {
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
}

#[derive(Debug, Clone)]
pub enum Token {
    Vtype(Vtype),
    Literal(String),
    Boolstr(bool),
    Num(i64),
    Character(char),
    Id(String),
    Class(String),
    Addsub(Addsub),
    Multdiv(Multdiv),
    Assign,
    Comp,
    Branchs(Branch),
    Ponctuation(Ponctuation),
    Nesting(Nesting),
}