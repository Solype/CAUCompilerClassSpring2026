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
    FDECL,
    ARG,
    MOREARGS,
    BLOCK,
    STMT,
    COND,
    ELSE,
    RETURN,
    CDECL,
    ODECL,
}

fn get_rules() -> Vec<Rule<Expression>> {
    vec![
        // 01
        Rule::new(Expression::CODE, vec![Expression::VDECL, Expression::CODE]),
        Rule::new(Expression::CODE, vec![Expression::FDECL, Expression::CODE]),
        Rule::new(Expression::CODE, vec![Expression::CDECL, Expression::CODE]),
        Rule::new(Expression::CODE, vec![]),
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
        Rule::new(Expression::RHS, vec![Expression::EXPR]),
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
            Expression::EXPR,
            vec![
                Expression::EXPR,
                Expression::Token(Token::from(TokenType::Addsub)),
                Expression::EXPR,
            ],
        ),
        Rule::new(
            Expression::EXPR,
            vec![
                Expression::EXPR,
                Expression::Token(Token::from(TokenType::Multdiv)),
                Expression::EXPR,
            ],
        ),
        // 06
        Rule::new(
            Expression::EXPR,
            vec![
                Expression::Token(Token::from(TokenType::Nesting(Nesting::Lparen))),
                Expression::EXPR,
                Expression::Token(Token::from(TokenType::Nesting(Nesting::Rparen))),
            ],
        ),
        Rule::new(
            Expression::EXPR,
            vec![Expression::Token(Token::from(TokenType::Id))],
        ),
        Rule::new(
            Expression::EXPR,
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
        Rule::new(Expression::ARG, vec![]),
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
        Rule::new(Expression::MOREARGS, vec![]),
        // 10
        Rule::new(Expression::BLOCK, vec![Expression::STMT, Expression::BLOCK]),
        Rule::new(Expression::BLOCK, vec![]),
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
                Expression::COND,
            ],
        ),
        Rule::new(
            Expression::COND,
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
        Rule::new(Expression::ELSE, vec![]),
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
        Rule::new(Expression::ODECL, vec![]),
    ]
}

fn expression1() -> Vec<Expression> {
    return vec![
        Expression::Token(Token::from(TokenType::Vtype)),
        Expression::Token(Token::from(TokenType::Id)),
        Expression::Token(Token::from(TokenType::Ponctuation(Ponctuation::Semi))),
    ];
}

fn expression2() -> Vec<Expression> {
    return vec![
        Expression::Token(Token::from(TokenType::Vtype)),
        Expression::Token(Token::from(TokenType::Id)),
        Expression::Token(Token::from(TokenType::Assign)),
        Expression::Token(Token::from(TokenType::Num)),
        Expression::Token(Token::from(TokenType::Ponctuation(Ponctuation::Semi))),
    ];
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
        Rule::new(Expression::STMT, vec![]),
    ]
}

fn main() {
    let buffer = read_input();

    let mut tokens = parse_token(&buffer);

    let mut parser = Parser::<Expression>::new();
    parser.add_rules(get_rules_test());
    // for token in tokens {
    //     parser.push_expr(Expression::Token(token));
    // }
    let mut sequence = tokens
        .iter_mut()
        .map(|t| Expression::Token(t.clone()))
        .collect();
    println!("before {:?}", sequence);

    parser.parse_sequence(&mut sequence);

    println!("after {:?}", sequence)
    // parser.display_tree();
    // println!("{}", tree);
}
