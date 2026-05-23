use std::fmt::{self, Debug};

#[derive(Clone, Debug)]
pub struct TreeNode<T> {
    pub value: T,
    pub children: Vec<TreeNode<T>>,
}

impl<T> fmt::Display for TreeNode<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n\\---", self.value)?;
        for node in &self.children {
            write!(f, "\t{}\t", node)?;
        }
        Ok(())
    }
}
pub struct Tree<T> {
    pub root: TreeNode<T>,
}

impl<T> fmt::Display for Tree<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.root)?;

        Ok(())
    }
}
