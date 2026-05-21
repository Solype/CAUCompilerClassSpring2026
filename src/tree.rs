#![allow(dead_code)]

use std::fmt::{self, Debug, Error, Write};

#[derive(Debug, Clone)]
pub struct Tree {
    start: Option<TreeNode<Expression>>,
}
#[derive(Debug, Clone)]
pub struct TreeNode<T>
where
    T: Debug,
{
    pub value: T,
    pub children: Vec<TreeNode<T>>,
}
impl Default for Tree {
    fn default() -> Self {
        Tree { start: None }
    }
}

impl<T> TreeNode<T>
where
    T: Debug,
{
    pub fn new(value: T, children: Vec<TreeNode<T>>) -> Self {
        TreeNode { value, children }
    }

    fn private_display(&self, out: &mut impl Write, nesting: usize) -> fmt::Result {
        write!(out, "{}{:?}", "  ".repeat(nesting), self.value)?;
        for child in &self.children {
            write!(out, "\n")?;
            child.private_display(out, nesting + 1)?;
        }

        Ok(())
    }
}

impl<T> fmt::Display for TreeNode<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if Ok(()) == self.private_display(f, 0) {
            fmt::Result::Ok(())
        } else {
            fmt::Result::Err(Error)
        }
    }
}
