mod token;
mod tree;
mod parser;
use token::*;

use crate::parser::{Parser, Rule};

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
    ODECL
}


fn get_rules() -> Vec<Rule<Expression>>
{
    vec![
        // 01
        Rule::new(Expression::CODE, vec![Expression::VDECL, Expression::CODE]),
        Rule::new(Expression::CODE, vec![Expression::FDECL, Expression::CODE]),
        Rule::new(Expression::CODE, vec![Expression::CDECL, Expression::CODE]),
        Rule::new(Expression::CODE, vec![]),

        // 02
        Rule::new(Expression::VDECL, vec![
            Expression::Token(Token::Vtype),
            Expression::Token(Token::Id),
            Expression::Token(Token::Ponctuation(Ponctuation::Semi))
        ]),
        Rule::new(Expression::VDECL, vec![
            Expression::Token(Token::Vtype),
            Expression::ASSIGN,
            Expression::Token(Token::Ponctuation(Ponctuation::Semi))
        ]),

        // 03
        Rule::new(Expression::ASSIGN, vec![
            Expression::Token(Token::Id),
            Expression::Token(Token::Assign),
            Expression::RHS
        ]),

        // 04
        Rule::new(Expression::RHS, vec![Expression::EXPR]),
        Rule::new(Expression::RHS, vec![Expression::Token(Token::Literal)]),
        Rule::new(Expression::RHS, vec![Expression::Token(Token::Character)]),
        Rule::new(Expression::RHS, vec![Expression::Token(Token::Boolstr)]),

        // 05
        Rule::new(Expression::EXPR, vec![
            Expression::EXPR,
            Expression::Token(Token::Addsub),
            Expression::EXPR
        ]),
        Rule::new(Expression::EXPR, vec![
            Expression::EXPR,
            Expression::Token(Token::Multdiv),
            Expression::EXPR
        ]),

        // 06
        Rule::new(Expression::EXPR, vec![
            Expression::Token(Token::Nesting(Nesting::Lparen)),
            Expression::EXPR,
            Expression::Token(Token::Nesting(Nesting::Rparen)),
        ]),
        Rule::new(Expression::EXPR, vec![Expression::Token(Token::Id)]),
        Rule::new(Expression::EXPR, vec![Expression::Token(Token::Id)]),

        // 07
        Rule::new(Expression::FDECL, vec![
            Expression::Token(Token::Vtype),
            Expression::Token(Token::Nesting(Nesting::Lparen)),
            Expression::ARG,
            Expression::Token(Token::Nesting(Nesting::Rparen)),
            Expression::Token(Token::Nesting(Nesting::Lbrace)),
            Expression::BLOCK,
            Expression::RETURN,
            Expression::Token(Token::Nesting(Nesting::Rbrace))
        ]),

        // 08
        Rule::new(Expression::ARG, vec![
            Expression::Token(Token::Vtype),
            Expression::Token(Token::Vtype),
            Expression::MOREARGS
        ]),
        Rule::new(Expression::ARG, vec![]),

        // 09
        Rule::new(Expression::MOREARGS, vec![
            Expression::Token(Token::Ponctuation(Ponctuation::Comma)),
            Expression::Token(Token::Vtype),
            Expression::Token(Token::Vtype),
            Expression::MOREARGS
        ]),
        Rule::new(Expression::MOREARGS, vec![]),

        // 10
        Rule::new(Expression::BLOCK, vec![
            Expression::STMT,
            Expression::BLOCK
        ]),
        Rule::new(Expression::BLOCK, vec![]),


        // 11
        Rule::new(Expression::STMT, vec![Expression::VDECL]),
        Rule::new(Expression::STMT, vec![
            Expression::ASSIGN,
            Expression::Token(Token::Ponctuation(Ponctuation::Semi))
        ]),

        // 12
        Rule::new(Expression::STMT, vec![
            Expression::Token(Token::Branchs(Branch::If)),
            Expression::Token(Token::Nesting(Nesting::Lparen)),
            Expression::COND,
            Expression::Token(Token::Nesting(Nesting::Rparen)),
            Expression::Token(Token::Nesting(Nesting::Lbrace)),
            Expression::BLOCK,
            Expression::Token(Token::Nesting(Nesting::Rbrace)),
            Expression::ELSE
        ]),

        // 13
        Rule::new(Expression::STMT, vec![
            Expression::Token(Token::Branchs(Branch::While)),
            Expression::Token(Token::Nesting(Nesting::Lparen)),
            Expression::COND,
            Expression::Token(Token::Nesting(Nesting::Rparen)),
            Expression::Token(Token::Nesting(Nesting::Lbrace)),
            Expression::BLOCK,
            Expression::Token(Token::Nesting(Nesting::Rbrace))
        ]),

        // 14
        Rule::new(Expression::COND, vec![
            Expression::COND,
            Expression::Token(Token::Comp),
            Expression::COND
        ]),
        Rule::new(Expression::COND, vec![Expression::Token(Token::Boolstr)]),

        // 15
        Rule::new(Expression::ELSE, vec![
            Expression::Token(Token::Branchs(Branch::Else)),
            Expression::Token(Token::Nesting(Nesting::Lbrace)),
            Expression::BLOCK,
            Expression::Token(Token::Nesting(Nesting::Rbrace))
        ]),
        Rule::new(Expression::ELSE, vec![]),


        // 16
        Rule::new(Expression::RETURN, vec![
            Expression::Token(Token::Branchs(Branch::Return)),
            Expression::RHS,
            Expression::Token(Token::Ponctuation(Ponctuation::Semi))
        ]),

        // 17
        Rule::new(Expression::CDECL, vec![
            Expression::Token(Token::Class),
            Expression::Token(Token::Vtype),
            Expression::Token(Token::Nesting(Nesting::Lbrace)),
            Expression::ODECL,
            Expression::Token(Token::Nesting(Nesting::Rbrace))
        ]),

        // 18
        Rule::new(Expression::ODECL, vec![
            Expression::VDECL,
            Expression::ODECL
        ]),
        Rule::new(Expression::ODECL, vec![
            Expression::FDECL,
            Expression::ODECL
        ]),
        Rule::new(Expression::ODECL, vec![]),
    ]
}

fn expression1() -> Vec<Expression>
{
    return vec![
        Expression::Token(Token::Vtype),
        Expression::Token(Token::Id),
        Expression::Token(Token::Ponctuation(Ponctuation::Semi)),
    ]
}

fn expression2() -> Vec<Expression>
{
    return vec![
        Expression::Token(Token::Vtype),
        Expression::Token(Token::Id),
        Expression::Token(Token::Assign),
        Expression::Token(Token::Num),
        Expression::Token(Token::Ponctuation(Ponctuation::Semi)),
    ]
}

fn main() {
    let mut parser = Parser::<Expression>::new();

    parser.add_rules(get_rules());
    let mut expr = expression1();
    parser.parse_sequence(&mut expr);
    println!("{:?}", expr);
    expr = expression2();
    parser.parse_sequence(&mut expr);
    println!("{:?}", expr);
    // for expr in expression() {
    //     parser.push_expr(expr);
    // }
    // println!("{}", tree);
}