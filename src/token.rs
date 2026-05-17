
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Branch {
    If,
    Else,
    While,
    Return
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
pub enum Token {
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