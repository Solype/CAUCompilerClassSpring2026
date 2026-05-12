pub enum addsub {
    Add,
    Sub
}

pub enum multdiv {
    Mult,
    Div
}

pub enum branch {
    If,
    Else,
    While,
    Return
}

pub enum vtype {
    Float,
    Integer,
    Other(String)
}

pub enum ponctuation {
    Semi,
    Comma,
}

pub enum nesting {
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
}

pub enum token {
    Vtype(vtype),
    Literal(String),
    Boolstr(bool),
    Num(i64),
    Character(char),
    Id(String),
    Class(String),
    Addsub(addsub),
    Multdiv(multdiv),
    Assign,
    Comp,
    Branchs(branch),
    Ponctuation(ponctuation),
    Nesting(nesting),
}