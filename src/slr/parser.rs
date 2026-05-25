use core::fmt;

use crate::{
    error::token_error, ruleparser::rules_and_tokens::{Term, Token, TokenMetadata}, slr::{
        table::{Action, Goto, LRTable, Productions, StateId},
        tree::{Tree, TreeNode},
    }
};

#[derive(Debug, Clone)]
pub struct TokenWithMetadata {
    pub token: Token,
    pub metadata: TokenMetadata,
}

impl fmt::Display for TokenWithMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.token {
            Token::Term(t) => write!(f, "{:?}", t),
            Token::NonTerm(nt) => write!(f, "{:?}", nt),
        }
    }
}

#[derive(Clone)]
enum StackValue {
    State(StateId),
    Token(TreeNode<TokenWithMetadata>),
}

impl fmt::Debug for StackValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StackValue::State(s) => write!(f, "State({})", s)?,
            StackValue::Token(TreeNode { value: t_m, .. }) => {
                write!(f, "Token({})", t_m.metadata.str)?
            }
        };

        Ok(())
    }
}

pub struct Parser {
    lr_table: LRTable,
    productions: Productions,
    stack: Vec<StackValue>,
}

impl Parser {
    pub fn new(productions: &Productions) -> Self {
        Self {
            lr_table: LRTable::new(productions),
            productions: productions.clone(),
            stack: vec![],
        }
    }

    fn action(&mut self, action: &Action, inputs: &mut Vec<(Term, TokenMetadata)>) {
        match action {
            Action::Shift(state_id) => {
                let (term, metadata) = inputs.pop().unwrap().clone();

                let node = TreeNode {
                    value: TokenWithMetadata {
                        token: Token::Term(term.clone()),
                        metadata,
                    },
                    children: vec![],
                };
                self.stack.push(StackValue::Token(node.clone()));
                self.stack.push(StackValue::State(*state_id));
            }
            Action::Reduce(rule_id) => {
                let production = self.productions.get_index(*rule_id).unwrap();

                let mut to_take = production.inputs.len();
                let mut children = vec![];
                while to_take != 0 {
                    let last = self.stack.pop().unwrap();
                    if let StackValue::Token(t) = last {
                        children.push(t);
                        to_take -= 1;
                    }
                }
                children.reverse();

                let new_metadata = if let Some((first, nexts)) = children.split_first() {
                    let mut t_metadata = TokenMetadata {
                        span: first.value.metadata.span,
                        str: first.value.metadata.str.clone(),
                        line: first.value.metadata.line.clone(),
                    };
                    nexts.iter().for_each(|TreeNode { value: t_m, .. }| {
                        t_metadata
                            .str
                            .push_str(&(" ".to_owned() + &t_m.metadata.str)); //todo space size
                        t_metadata.line = t_metadata.line.to_owned() + t_m.metadata.line.as_str(); //todo line vec
                    });
                    t_metadata
                } else {
                    TokenMetadata::default()
                };
                let new_node = TreeNode {
                    value: TokenWithMetadata {
                        token: Token::NonTerm(production.nt.clone()),
                        metadata: new_metadata,
                    },
                    children,
                };

                self.stack.push(StackValue::Token(new_node));
            }
            Action::Accept => {
                inputs.pop();
            }
        }
    }

    fn goto(&mut self, goto: Goto) {
        self.stack.push(StackValue::State(goto));
    }

    pub fn parse(
        &mut self,
        mut inputs: Vec<(Term, TokenMetadata)>,
    ) -> Result<Tree<TokenWithMetadata>, String> {
        self.stack = Vec::from([StackValue::State(0)]);

        inputs.reverse();
        while !inputs.is_empty() {
            let stack_value = self.stack.last().unwrap().clone();
            match stack_value {
                StackValue::State(state_id) => {
                    let (term, m) = inputs.last().unwrap().clone();

                    let Some(action) = self
                        .lr_table
                        .actions
                        .get(&(state_id, term.clone()))
                        .cloned()
                    else {
                        return Err(token_error(TokenWithMetadata {token: Token::Term(term.clone()), metadata: m, }));
                    };

                    self.action(&action, &mut inputs);
                }
                StackValue::Token(TreeNode {
                    value:
                        TokenWithMetadata {
                            token: Token::NonTerm(nt),
                            ..
                        },
                    ..
                }) => {
                    let StackValue::State(state) = self.stack.iter().rev().nth(1).unwrap() else {
                        panic!("An error occured");
                    };

                    let Some(goto) = self.lr_table.gotos.get(&(*state, nt.clone())).cloned() else {
                        let (term, m) = inputs.last().unwrap().clone();

                        return Err(token_error(TokenWithMetadata { token: Token::Term(term.clone()), metadata: m,}));
                    };

                    self.goto(goto);
                }
                StackValue::Token(node) => return Err(token_error(node.value)),
            };
        }

        Ok(Tree {
            root: {
                self.stack
                    .drain(..)
                    .find_map(|v| match v {
                        StackValue::Token(node) => Some(node),
                        _ => None,
                    })
                    .unwrap()
            },
        })
    }
}
