mod input;
mod parser;
mod token;
mod tree;

use token::*;

use crate::{
    input::read_input,
    parser::{Parser, Rule},
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
    HIGHOP,
    LOWOP,
    OPERAND,
    FDECL,
    ARG,
    MOREARGS,
    BLOCK,
    STMT,
    COND,
    RCOND,
    ELSE,
    RETURN,
    CDECL,
    ODECL,
}

fn get_rules() -> Vec<Rule<Expression>> {
    vec![
        // epsilon
        Rule::new(Expression::CODE, vec![]),
        Rule::new(Expression::ARG, vec![]),
        Rule::new(Expression::MOREARGS, vec![]),
        Rule::new(Expression::BLOCK, vec![]),
        Rule::new(Expression::ELSE, vec![]),
        Rule::new(Expression::ODECL, vec![]),
        Rule::new(Expression::STMT, vec![]),
        // 01
        Rule::new(Expression::CODE, vec![Expression::VDECL, Expression::CODE]),
        Rule::new(Expression::CODE, vec![Expression::FDECL, Expression::CODE]),
        Rule::new(Expression::CODE, vec![Expression::CDECL, Expression::CODE]),
        // 02
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
        // 03
        Rule::new(
            Expression::ASSIGN,
            vec![
                Expression::Token(Token::from(TokenType::Id)),
                Expression::Token(Token::from(TokenType::Assign)),
                Expression::RHS,
            ],
        ),
        // 04
        Rule::new(Expression::RHS, vec![Expression::HIGHOP]),
        Rule::new(
            Expression::RHS,
            vec![Expression::Token(Token::from(TokenType::Literal))],
        ),
        Rule::new(
            Expression::RHS,
            vec![Expression::Token(Token::from(TokenType::Character))],
        ),
        Rule::new(
            Expression::RHS,
            vec![Expression::Token(Token::from(TokenType::Boolstr))],
        ),
        // 05
        Rule::new(
            Expression::HIGHOP,
            vec![
                Expression::HIGHOP,
                Expression::Token(Token::from(TokenType::Addsub)),
                Expression::LOWOP,
            ],
        ),
        Rule::new(Expression::HIGHOP, vec![Expression::LOWOP]),
        Rule::new(
            Expression::LOWOP,
            vec![
                Expression::LOWOP,
                Expression::Token(Token::from(TokenType::Multdiv)),
                Expression::OPERAND,
            ],
        ),
        // 06
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
            vec![Expression::Token(Token::from(TokenType::Id))],
        ),
        Rule::new(
            Expression::OPERAND,
            vec![Expression::Token(Token::from(TokenType::Id))],
        ),
        // 07
        Rule::new(
            Expression::FDECL,
            vec![
                Expression::Token(Token::from(TokenType::Vtype)),
                Expression::Token(Token::from(TokenType::Nesting(Nesting::Lparen))),
                Expression::ARG,
                Expression::Token(Token::from(TokenType::Nesting(Nesting::Rparen))),
                Expression::Token(Token::from(TokenType::Nesting(Nesting::Lbrace))),
                Expression::BLOCK,
                Expression::RETURN,
                Expression::Token(Token::from(TokenType::Nesting(Nesting::Rbrace))),
            ],
        ),
        // 08
        Rule::new(
            Expression::ARG,
            vec![
                Expression::Token(Token::from(TokenType::Vtype)),
                Expression::Token(Token::from(TokenType::Vtype)),
                Expression::MOREARGS,
            ],
        ),
        // 09
        Rule::new(
            Expression::MOREARGS,
            vec![
                Expression::Token(Token::from(TokenType::Ponctuation(Ponctuation::Comma))),
                Expression::Token(Token::from(TokenType::Vtype)),
                Expression::Token(Token::from(TokenType::Vtype)),
                Expression::MOREARGS,
            ],
        ),
        // 10
        Rule::new(Expression::BLOCK, vec![Expression::STMT, Expression::BLOCK]),
        // 11
        Rule::new(Expression::STMT, vec![Expression::VDECL]),
        Rule::new(
            Expression::STMT,
            vec![
                Expression::ASSIGN,
                Expression::Token(Token::from(TokenType::Ponctuation(Ponctuation::Semi))),
            ],
        ),
        // 12
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
        // 13
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
        // 14
        Rule::new(
            Expression::COND,
            vec![
                Expression::COND,
                Expression::Token(Token::from(TokenType::Comp)),
                Expression::RCOND,
            ],
        ),
        Rule::new(Expression::COND, vec![Expression::RCOND]),
        Rule::new(
            Expression::RCOND,
            vec![Expression::Token(Token::from(TokenType::Boolstr))],
        ),
        // 15
        Rule::new(
            Expression::ELSE,
            vec![
                Expression::Token(Token::from(TokenType::Branchs(Branch::Else))),
                Expression::Token(Token::from(TokenType::Nesting(Nesting::Lbrace))),
                Expression::BLOCK,
                Expression::Token(Token::from(TokenType::Nesting(Nesting::Rbrace))),
            ],
        ),
        // 16
        Rule::new(
            Expression::RETURN,
            vec![
                Expression::Token(Token::from(TokenType::Branchs(Branch::Return))),
                Expression::RHS,
                Expression::Token(Token::from(TokenType::Ponctuation(Ponctuation::Semi))),
            ],
        ),
        // 17
        Rule::new(
            Expression::CDECL,
            vec![
                Expression::Token(Token::from(TokenType::Class)),
                Expression::Token(Token::from(TokenType::Vtype)),
                Expression::Token(Token::from(TokenType::Nesting(Nesting::Lbrace))),
                Expression::ODECL,
                Expression::Token(Token::from(TokenType::Nesting(Nesting::Rbrace))),
            ],
        ),
        // 18
        Rule::new(
            Expression::ODECL,
            vec![Expression::VDECL, Expression::ODECL],
        ),
        Rule::new(
            Expression::ODECL,
            vec![Expression::FDECL, Expression::ODECL],
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
    let parsed_tree = parser.parse_sequence(sequence);
    for elem in parsed_tree {
        elem.display();
    }
}
