use std::{
    collections::{HashMap, HashSet},
    fs::Metadata,
    hash::Hash,
};

type State = usize;

enum Action {
    State(State),
    Return(State),
    Accept,
}

type Goto = State;

struct Token {
    typ: TerminalSymbol,
    metadata: Metadata,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum TerminalSymbol {
    Epsilon,
    Vtype,
    Literal,
    Boolstr,
    Num,
    Character,
    Id,
    If,
    Else,
    While,
    Return,
    Class,
    Addsub,
    Multdiv,
    Assign,
    Comp,
    Semi,
    Comma,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    End,
    Undefined,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum NonterminalSymbol {
    Code,
    VDecl,
    Assign,
    Rhs,
    HighOp,
    LowOp,
    Operand,
    FDecl,
    Arg,
    MoreArgs,
    Block,
    Stmt,
    Cond,
    RCond,
    Else,
    Return,
    CDecl,
    ODecl,
}

#[derive(Debug)]
pub enum Symbol {
    Terminal(TerminalSymbol),
    Nonterminal(NonterminalSymbol),
}
type Symbols = Vec<Symbol>;

type Row<T, K> = HashMap<T, K>;

pub type Productions = HashMap<NonterminalSymbol, Vec<Symbols>>;
pub struct Rules {
    pub start: NonterminalSymbol,
    pub productions: Productions,
}

type FirstSet = HashSet<TerminalSymbol>;
type FollowSet = HashSet<TerminalSymbol>;

type FirstTable = HashMap<NonterminalSymbol, FirstSet>;
type FollowTable = HashMap<NonterminalSymbol, FollowSet>;

#[derive(Debug)]
pub struct FirstFollowSets {
    pub first: FirstTable,
    pub follow: FollowTable,
}
impl FirstFollowSets {
    fn new(rules: &Rules) -> Self {
        let first_table = Self::_build_first(rules);
        let follow_table = Self::_build_follow(&first_table, rules);
        Self {
            first: first_table,
            follow: follow_table,
        }
    }

    fn _build_first(rules: &Rules) -> FirstTable {
        let mut table = HashMap::new();
        for nt in rules.productions.keys() {
            table.insert(nt.clone(), FirstSet::new());
        }

        let mut changed = true;

        while changed {
            changed = false;

            for (output, inputs) in &rules.productions {
                for input in inputs {
                    let Some(symbol) = input.first() else {
                        changed |= table
                            .get_mut(&output)
                            .unwrap()
                            .insert(TerminalSymbol::Epsilon);

                        continue;
                    };

                    let firsts = match symbol {
                        Symbol::Terminal(t) => FirstSet::from([t.clone()]),

                        Symbol::Nonterminal(nt) => table.get(nt).cloned().unwrap(),
                    };
                    for first in firsts {
                        changed |= table.get_mut(&output).unwrap().insert(first);
                    }
                }
            }
        }

        table
    }

    fn _build_follow(first_table: &FirstTable, rules: &Rules) -> FollowTable {
        let mut table = HashMap::new();
        for nt in rules.productions.keys() {
            table.insert(
                nt.clone(),
                if nt == &rules.start {
                    FollowSet::from([TerminalSymbol::End])
                } else {
                    FollowSet::from([TerminalSymbol::Undefined])
                },
            );
        }

        let mut changed = true;

        while changed {
            changed = false;

            for (output, inputs) in &rules.productions {
                if *output == NonterminalSymbol::CDecl {
                    println!("{:?}", table.get(output).unwrap())
                }
                for input in inputs {
                    for (i, symbol) in input.iter().enumerate() {
                        let Symbol::Nonterminal(followed) = symbol else {
                            continue;
                        };

                        let follows = match input.get(i + 1) {
                            Some(Symbol::Terminal(t)) => FollowSet::from([t.clone()]),

                            Some(Symbol::Nonterminal(nt)) => {
                                let mut nt_first = first_table.get(nt).cloned().unwrap();
                                if nt_first.contains(&TerminalSymbol::Epsilon) {
                                    nt_first.extend(table.get(&nt).cloned().unwrap());
                                }
                                nt_first
                            }

                            None => table.get(&output).cloned().unwrap(),
                        };
                        for follow in follows.iter().filter(|t| {
                            ![TerminalSymbol::Epsilon, TerminalSymbol::Undefined].contains(t)
                        }) {
                            table
                                .get_mut(followed)
                                .unwrap()
                                .remove(&TerminalSymbol::Undefined);
                            changed |= table.get_mut(followed).unwrap().insert(follow.clone());
                        }
                    }
                }
            }
        }

        table
    }
}
pub struct LRTable {
    pub actions_table: HashMap<State, Row<TerminalSymbol, Action>>,
    pub goto_table: HashMap<State, Row<NonterminalSymbol, Goto>>,
}

impl LRTable {
    pub fn new(rules: &Rules) -> Self {
        let first_follow = FirstFollowSets::new(rules);

        println!("first");
        for set in first_follow.first {
            println!("{:?}", set);
        }
        println!("follow");
        for set in first_follow.follow {
            println!("{:?}", set);
        }

        Self {
            actions_table: HashMap::new(),
            goto_table: HashMap::new(),
        }
    }
}
