mod token;
mod tree;
use tree::Tree;
use token::*;


fn main() {
    let tree = Tree::new(
        Token::Addsub(Addsub::Add),
        Some(Tree::new(Token::Num(1), None, None)),
        Some(Tree::new(
            Token::Multdiv(Multdiv::Mult),
            Some(Tree::new(Token::Num(2), None, None)),
            Some(Tree::new(Token::Num(3), None, None)),
        )),
    );

    println!("{}", tree);
}