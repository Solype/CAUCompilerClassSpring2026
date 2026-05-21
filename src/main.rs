mod input;
mod parser;
mod token;
mod tree;

use token::*;

use crate::{
    input::read_input,
    parser::{parser::Parser, Rule},
};

#[derive(PartialEq, Eq, Debug, Default, Clone, Hash)]
pub enum Expression {
    Token(Token),
    #[default]
    CODE,
    VDECL,
    ASSIGN,
    RHS,
    EXPR,
    EXPRPRIME,
    EXPRDOUBLEPRIME,
    HIGHOP,
    LOWOP,
    OPERAND,
    FDECL,
    ARG,
    MOREARGS,
    BLOCK,
    STMT,
    COND,
    CONDPRIME,
    RCOND,
    ELSE,
    RETURN,
    CDECL,
    DECL,
    ODECL,
}

fn get_rules() -> Vec<Rule<Expression>> {
    vec![
        Rule::new(
            Expression::CODE,
            vec![
                Expression::VDECL,
                Expression::CODE,
            ],
        ),

        Rule::new(
            Expression::CODE,
            vec![
                Expression::FDECL,
                Expression::CODE,
            ],
        ),

        Rule::new(
            Expression::CODE,
            vec![
                Expression::CDECL,
                Expression::CODE,
            ],
        ),

        Rule::new(
            Expression::CODE,
            vec![],
        ),

        Rule::new(
            Expression::VDECL,
            vec![
                Expression::Token(Token::from(TokenType::Vtype)),
                Expression::Token(Token::from(TokenType::Id)),
                Expression::Token(Token::from(TokenType::Ponctuation(Ponctuation::Semi))),
            ],
        ),

        Rule::new(
            Expression::VDECL,
            vec![
                Expression::Token(Token::from(TokenType::Vtype)),
                Expression::ASSIGN,
                Expression::Token(Token::from(TokenType::Ponctuation(Ponctuation::Semi))),
            ],
        ),

        Rule::new(
            Expression::ASSIGN,
            vec![
                Expression::Token(Token::from(TokenType::Id)),
                Expression::Token(Token::from(TokenType::Assign)),
                Expression::RHS,
            ],
        ),

        Rule::new(
            Expression::RHS,
            vec![
                Expression::HIGHOP,
            ],
        ),

        Rule::new(
            Expression::RHS,
            vec![
                Expression::Token(Token::from(TokenType::Literal)),
            ],
        ),

        Rule::new(
            Expression::RHS,
            vec![
                Expression::Token(Token::from(TokenType::Character)),
            ],
        ),

        Rule::new(
            Expression::RHS,
            vec![
                Expression::Token(Token::from(TokenType::Boolstr)),
            ],
        ),

        Rule::new(
            Expression::HIGHOP,
            vec![
                Expression::HIGHOP,
                Expression::Token(Token::from(TokenType::Addsub)),
                Expression::LOWOP,
            ],
        ),

        Rule::new(
            Expression::HIGHOP,
            vec![
                Expression::LOWOP,
            ],
        ),

        Rule::new(
            Expression::LOWOP,
            vec![
                Expression::LOWOP,
                Expression::Token(Token::from(TokenType::Multdiv)),
                Expression::OPERAND,
            ],
        ),

        Rule::new(
            Expression::LOWOP,
            vec![
                Expression::OPERAND,
            ],
        ),

        Rule::new(
            Expression::OPERAND,
            vec![
                Expression::Token(Token::from(TokenType::Nesting(Nesting::Lparen))),
                Expression::HIGHOP,
                Expression::Token(Token::from(TokenType::Nesting(Nesting::Rparen))),
            ],
        ),

        Rule::new(
            Expression::OPERAND,
            vec![
                Expression::Token(Token::from(TokenType::Id)),
            ],
        ),

        Rule::new(
            Expression::OPERAND,
            vec![
                Expression::Token(Token::from(TokenType::Num)),
            ],
        ),

        Rule::new(
            Expression::FDECL,
            vec![
                Expression::Token(Token::from(TokenType::Vtype)),
                Expression::Token(Token::from(TokenType::Id)),
                Expression::Token(Token::from(TokenType::Nesting(Nesting::Lparen))),
                Expression::ARG,
                Expression::Token(Token::from(TokenType::Nesting(Nesting::Rparen))),
                Expression::Token(Token::from(TokenType::Nesting(Nesting::Lbrace))),
                Expression::BLOCK,
                Expression::RETURN,
                Expression::Token(Token::from(TokenType::Nesting(Nesting::Rbrace))),
            ],
        ),

        Rule::new(
            Expression::ARG,
            vec![
                Expression::Token(Token::from(TokenType::Vtype)),
                Expression::Token(Token::from(TokenType::Id)),
                Expression::MOREARGS,
            ],
        ),

        Rule::new(
            Expression::ARG,
            vec![],
        ),

        Rule::new(
            Expression::MOREARGS,
            vec![
                Expression::Token(Token::from(TokenType::Ponctuation(Ponctuation::Comma))),
                Expression::Token(Token::from(TokenType::Vtype)),
                Expression::Token(Token::from(TokenType::Id)),
                Expression::MOREARGS,
            ],
        ),

        Rule::new(
            Expression::MOREARGS,
            vec![],
        ),

        Rule::new(
            Expression::BLOCK,
            vec![
                Expression::STMT,
                Expression::BLOCK,
            ],
        ),

        Rule::new(
            Expression::BLOCK,
            vec![],
        ),

        Rule::new(
            Expression::STMT,
            vec![
                Expression::VDECL,
            ],
        ),

        Rule::new(
            Expression::STMT,
            vec![
                Expression::ASSIGN,
                Expression::Token(Token::from(TokenType::Ponctuation(Ponctuation::Semi))),
            ],
        ),

        Rule::new(
            Expression::STMT,
            vec![
                Expression::Token(Token::from(TokenType::Branchs(Branch::If))),
                Expression::Token(Token::from(TokenType::Nesting(Nesting::Lparen))),
                Expression::COND,
                Expression::Token(Token::from(TokenType::Nesting(Nesting::Rparen))),
                Expression::Token(Token::from(TokenType::Nesting(Nesting::Lbrace))),
                Expression::BLOCK,
                Expression::Token(Token::from(TokenType::Nesting(Nesting::Rbrace))),
                Expression::ELSE,
            ],
        ),

        Rule::new(
            Expression::STMT,
            vec![
                Expression::Token(Token::from(TokenType::Branchs(Branch::While))),
                Expression::Token(Token::from(TokenType::Nesting(Nesting::Lparen))),
                Expression::COND,
                Expression::Token(Token::from(TokenType::Nesting(Nesting::Rparen))),
                Expression::Token(Token::from(TokenType::Nesting(Nesting::Lbrace))),
                Expression::BLOCK,
                Expression::Token(Token::from(TokenType::Nesting(Nesting::Rbrace))),
            ],
        ),

        Rule::new(
            Expression::COND,
            vec![
                Expression::COND,
                Expression::Token(Token::from(TokenType::Comp)),
                Expression::RCOND,
            ],
        ),

        Rule::new(
            Expression::COND,
            vec![
                Expression::RCOND,
            ],
        ),

        Rule::new(
            Expression::RCOND,
            vec![
                Expression::Token(Token::from(TokenType::Boolstr)),
            ],
        ),

        Rule::new(
            Expression::ELSE,
            vec![
                Expression::Token(Token::from(TokenType::Branchs(Branch::Else))),
                Expression::Token(Token::from(TokenType::Nesting(Nesting::Lbrace))),
                Expression::BLOCK,
                Expression::Token(Token::from(TokenType::Nesting(Nesting::Rbrace))),
            ],
        ),

        Rule::new(
            Expression::ELSE,
            vec![],
        ),

        Rule::new(
            Expression::RETURN,
            vec![
                Expression::Token(Token::from(TokenType::Branchs(Branch::Return))),
                Expression::RHS,
                Expression::Token(Token::from(TokenType::Ponctuation(Ponctuation::Semi))),
            ],
        ),

        Rule::new(
            Expression::CDECL,
            vec![
                Expression::Token(Token::from(TokenType::Class)),
                Expression::Token(Token::from(TokenType::Id)),
                Expression::Token(Token::from(TokenType::Nesting(Nesting::Lbrace))),
                Expression::ODECL,
                Expression::Token(Token::from(TokenType::Nesting(Nesting::Rbrace))),
            ],
        ),

        Rule::new(
            Expression::ODECL,
            vec![
                Expression::VDECL,
                Expression::ODECL,
            ],
        ),

        Rule::new(
            Expression::ODECL,
            vec![
                Expression::FDECL,
                Expression::ODECL,
            ],
        ),

        Rule::new(
            Expression::ODECL,
            vec![],
        ),
    ]
}
fn get_rules_test() -> Vec<Rule<Expression>> {
    vec![
        // 01
        Rule::new(
            Expression::CODE,
            vec![
                Expression::STMT,
                Expression::Token(Token::from(TokenType::Vtype)),
                Expression::Token(Token::from(TokenType::Id)),
                Expression::Token(Token::from(TokenType::Ponctuation(Ponctuation::Semi))),
            ],
        ),
        Rule::new(
            Expression::STMT,
            vec![Expression::Token(Token::from(TokenType::Comp))],
        ),
    ]
}

fn main() {
    let buffer = read_input();
    let tokens = parse_token(&buffer);
    let sequence = tokens.iter().map(|t| Expression::Token(t.clone())).collect();

    let mut parser = Parser::<Expression>::new();
    parser.add_rules(get_rules());
    parser.display_rules();

    println!("before {:?}", sequence);
    parser.parse_sequence(sequence);
}
