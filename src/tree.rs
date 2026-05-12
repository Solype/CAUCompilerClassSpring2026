use crate::token::Token;
use std::{
    fmt::{self, Error, Write},
};

#[derive(Debug, Clone)]
pub struct Tree {
    token: Token,
    left : Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}


impl Tree {
    pub fn new(token: Token, left: Option<Tree>, right: Option<Tree>) -> Self {
        Tree {
            token: token,
            left: left.map(Box::new),
            right: right.map(Box::new),
        }
    }

    fn private_display( &self, out: &mut impl Write, nesting: usize, ) -> fmt::Result
    {
        write!(out, "{}{:?}", "  ".repeat(nesting), self.token)?;
        if let Some(ref left) = self.left {
            write!(out, "\n")?;
            left.private_display(out, nesting + 1)?;
        }
        if let Some(ref right) = self.right {
            write!(out, "\n")?;
            right.private_display(out, nesting + 1)?;
        }
        Ok(())
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        if Ok(()) == self.private_display(f, 0) {
            fmt::Result::Ok(())
        } else {
            fmt::Result::Err(Error)
        }
    }
}
