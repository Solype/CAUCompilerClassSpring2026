use crate::{
    error::token_error,
    ruleparser::rules_and_tokens::{Term, Token, TokenMetadata},
    slr::{
        table::{Action, Goto, Productions, SLRTable, StateId},
        tree::{Tree, TreeNode},
    },
};

/**
 * A grammar token together with source-code metadata.
 *
 * The metadata is preserved during parsing so that
 * syntax errors and parse-tree nodes can reference
 * their original location in the source file.
 */
#[derive(Debug, Clone)]
pub struct TokenWithMetadata {
    pub token: Token,
    pub metadata: TokenMetadata,
}
/**
 * Element stored on the parser stack.
 *
 * The parser follows the classical LR stack layout:
 *
 *     State₀ Symbol₁ State₁ Symbol₂ State₂ ...
 *
 * States are used for ACTION/GOTO lookups, while
 * tokens/nonterminals are preserved to build the
 * parse tree during reductions.
 *
 * Example stack:
 *
 *     [State(0),
 *      Token(id),
 *      State(5),
 *      Token(expr),
 *      State(9)]
 *
 * This corresponds to the classical LR parser
 * stack representation:
 *
 *     0 id 5 expr 9
 */
#[derive(Clone)]
enum StackValue {
    State(StateId),
    Token(TreeNode<TokenWithMetadata>),
}
/**
 * SLR parser implementation.
 *
 * The parser uses an SLR parsing table generated
 * from the grammar and constructs a parse tree
 * while validating the input token sequence.
 *
 * Parsing is performed using shift, reduce, goto
 * and accept operations.
 */
pub struct Parser {
    pub slr_table: SLRTable,
    pub productions: Productions,
    stack: Vec<StackValue>,
}

impl Parser {
    /**
     *  Creates a new parser instance.
     * An SLR parsing table is generated from the
     * provided grammar productions.
     *
     * The parser stack is initially empty and is
     * initialized when parsing begins.
     */
    pub fn new(productions: &Productions) -> Self {
        Self {
            slr_table: SLRTable::new(productions),
            productions: productions.clone(),
            stack: vec![],
        }
    }
    /**
     * Executes a parser ACTION.
     *
     * Possible actions:
     *
     * Shift:
     *     - consume the next input token
     *     - create a leaf node
     *     - push the token and destination state
     *
     * Reduce:
     *     - pop symbols corresponding to the
     *       production body
     *     - create a nonterminal node
     *     - push the new node onto the stack
     *
     * Accept:
     *     - consume the END token and finish parsing
     */
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

                // Combine metadata from all reduced children.
                //
                // The resulting nonterminal node spans the
                // complete source range covered by the reduction.
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

    /**
     * Executes a GOTO transition.
     *
     * After a reduction, the parser consults the
     * GOTO table using:
     *
     *     (current_state, reduced_nonterminal)
     *
     * and pushes the resulting state onto the stack.
     */
    fn goto(&mut self, goto: Goto) {
        self.stack.push(StackValue::State(goto));
    }

    /**
     * Parses a token sequence and constructs a parse tree.
     *
     * Algorithm:
     *
     * 1. Initialize the stack with state 0.
     * 2. Repeatedly consult the ACTION table.
     * 3. Execute Shift, Reduce or Accept.
     * 4. After each reduction, consult the GOTO table.
     * 5. Continue until the input is accepted.
     *
     * On success, the root of the parse tree is
     * returned.
     *
     * On failure, a syntax error describing the
     * unexpected token is returned.
     */
    pub fn parse(
        &mut self,
        mut inputs: Vec<(Term, TokenMetadata)>,
        token_file: Option<String>
    ) -> Result<Tree<TokenWithMetadata>, String> {
        self.stack = Vec::from([StackValue::State(0)]);

        inputs.reverse();
        while !inputs.is_empty() {
            let stack_value = self.stack.last().unwrap().clone();
            match stack_value {
                StackValue::State(state_id) => {
                    let (term, m) = inputs.last().unwrap().clone();

                    let Some(action) = self
                        .slr_table
                        .actions
                        .get(&(state_id, term.clone()))
                        .cloned()
                    else {
                        return Err(token_error(TokenWithMetadata {
                            token: Token::Term(term.clone()),
                            metadata: m,
                        }, token_file));
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

                    let Some(goto) = self.slr_table.gotos.get(&(*state, nt.clone())).cloned()
                    else {
                        let (term, m) = inputs.last().unwrap().clone();

                        return Err(token_error(TokenWithMetadata {
                            token: Token::Term(term.clone()),
                            metadata: m,
                        }, token_file));
                    };

                    self.goto(goto);
                }
                StackValue::Token(node) => return Err(token_error(node.value, token_file)),
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

////////////////////////////////////////////////////////////////
/// DISPLAY
////////////////////////////////////////////////////////////////
use core::fmt;

impl fmt::Display for TokenWithMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.token {
            Token::Term(t) => write!(f, "{:?}", t),
            Token::NonTerm(nt) => write!(f, "{:?}", nt),
        }
    }
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
