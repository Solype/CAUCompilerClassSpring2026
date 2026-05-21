use core::fmt;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    fs::Metadata,
    hash::Hash,
};

use indexmap::IndexSet;

type StateId = usize;

enum Action {
    Shift(StateId),
    Reduce(StateId),
    Accept,
}

type Goto = StateId;

struct Token {
    typ: TerminalSymbol,
    metadata: Metadata,
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Debug)]
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

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Debug)]
pub enum NonterminalSymbol {
    Start,
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

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Debug)]
pub enum Symbol {
    Terminal(TerminalSymbol),
    Nonterminal(NonterminalSymbol),
}

type Row<T, K> = HashMap<T, K>;

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Debug)]
pub struct Production {
    nt: NonterminalSymbol,
    inputs: Vec<Symbol>,
}
impl Production {
    pub fn new(nt: NonterminalSymbol, inputs: Vec<Symbol>) -> Self {
        Self { nt, inputs }
    }
}
pub type Productions = IndexSet<Production>;

type FirstSet = HashSet<TerminalSymbol>;
type FollowSet = HashSet<TerminalSymbol>;

type FirstTable = BTreeMap<NonterminalSymbol, FirstSet>;
type FollowTable = BTreeMap<NonterminalSymbol, FollowSet>;

#[derive(Debug)]
pub struct FirstFollowSets {
    pub first: FirstTable,
    pub follow: FollowTable,
}
impl FirstFollowSets {
    fn new(productions: &Productions) -> Self {
        let first_table = Self::_build_first(productions);
        let follow_table = Self::_build_follow(&first_table, productions);
        Self {
            first: first_table,
            follow: follow_table,
        }
    }

    fn _build_first(productions: &Productions) -> FirstTable {
        let mut table = FirstTable::new();
        for Production { nt, .. } in productions {
            table.insert(nt.clone(), FirstSet::new());
        }

        let mut changed = true;

        while changed {
            changed = false;

            for Production { nt, inputs } in productions {
                println!("{:?} {:?}", nt, inputs);
                let Some(symbol) = inputs.first() else {
                    changed |= table.get_mut(&nt).unwrap().insert(TerminalSymbol::Epsilon);

                    continue;
                };

                let firsts = match symbol {
                    Symbol::Terminal(t) => FirstSet::from([t.clone()]),

                    Symbol::Nonterminal(nt) => table.get(nt).cloned().unwrap(),
                };
                for first in firsts {
                    changed |= table.get_mut(&nt).unwrap().insert(first);
                }
            }
        }

        table
    }

    fn _build_follow(first_table: &FirstTable, productions: &Productions) -> FollowTable {
        let mut table = FollowTable::new();
        for Production { nt, .. } in productions {
            table.insert(
                nt.clone(),
                if nt == &productions.first().unwrap().nt {
                    FollowSet::from([TerminalSymbol::End])
                } else {
                    FollowSet::from([TerminalSymbol::Undefined])
                },
            );
        }

        let mut changed = true;

        while changed {
            changed = false;

            for Production { nt, inputs } in productions {
                for (i, symbol) in inputs.iter().enumerate() {
                    let Symbol::Nonterminal(followed) = symbol else {
                        continue;
                    };

                    let follows = match inputs.get(i + 1) {
                        Some(Symbol::Terminal(t)) => FollowSet::from([t.clone()]),

                        Some(Symbol::Nonterminal(nt)) => {
                            let mut nt_first = first_table.get(nt).cloned().unwrap();
                            if nt_first.contains(&TerminalSymbol::Epsilon) {
                                nt_first.extend(table.get(&nt).cloned().unwrap());
                            }
                            nt_first
                        }

                        None => table.get(&nt).cloned().unwrap(),
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

        table
    }
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Debug)]
pub struct LRItem {
    production: Production,
    dot: usize,
}
impl fmt::Display for LRItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ->", self.production.nt)?;

        for (i, symbol) in self.production.inputs.iter().enumerate() {
            if i == self.dot {
                write!(f, " .")?;
            }

            write!(f, " {:?}", symbol)?;
        }

        if self.dot == self.production.inputs.len() {
            write!(f, " .")?;
        }

        Ok(())
    }
}
pub type State = BTreeSet<LRItem>;
pub struct LRItems {
    states: IndexSet<State>,
}
impl LRItems {
    fn _closure_items(
        nt: &NonterminalSymbol,
        dot: usize,
        productions: &Productions,
    ) -> Vec<LRItem> {
        productions
            .iter()
            .filter_map(|production| {
                if nt == &production.nt {
                    Some(LRItem {
                        production: production.clone(),
                        dot,
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    fn _closure(mut state: State, productions: &Productions) -> State {
        let mut changed = true;
        let mut to_add = vec![];
        while changed {
            changed = false;
            for item in &state {
                if let Some(Symbol::Nonterminal(nt)) = item.production.inputs.get(item.dot) {
                    to_add.extend(Self::_closure_items(nt, 0, productions));
                }
            }

            while !to_add.is_empty() {
                let new_item = to_add.pop().unwrap();
                changed |= state.insert(new_item);
            }
        }

        state
    }

    pub fn _goto(from_state: &State, symbol: &Symbol) -> State {
        let mut new_state = State::new();
        for item in from_state {
            if item.production.inputs.get(item.dot) == Some(symbol) {
                let mut new_item = item.clone();
                new_item.dot = item.dot + 1;
                new_state.insert(new_item.clone());
            }
        }

        new_state
    }

    pub fn new(productions: &Productions, follows: &FollowTable) -> Self {
        let start_production = Production::new(
            NonterminalSymbol::Start,
            vec![Symbol::Nonterminal(productions.first().unwrap().nt.clone())],
        );

        let mut start_state = State::from([LRItem {
            production: start_production,
            dot: 0,
        }]);

        start_state = Self::_closure(start_state, &productions);

        let mut lr_items = LRItems {
            states: IndexSet::from([start_state.clone()]),
        };

        let mut new_states = vec![start_state.clone()];
        println!("state 0:");
        for i in start_state.clone() {
            println!("{}", i);
        }

        while !new_states.is_empty() {
            let from_state = new_states.pop().unwrap();

            let mut next_symbols = HashSet::new();
            next_symbols.extend(
                from_state
                    .iter()
                    .map(|item| item.production.inputs.get(item.dot))
                    .flatten(),
            );
            for next_symbol in next_symbols {
                println!(
                    "goto state {:?} symbol: {:?}\n",
                    lr_items.states.get_index_of(&from_state),
                    next_symbol
                );
                let mut new_state = Self::_goto(&from_state, next_symbol);
                new_state = Self::_closure(new_state, productions);
                if lr_items.states.insert(new_state.clone()) {
                    new_states.push(new_state.clone());
                }
                println!("-->> state {:?}", lr_items.states.get_index_of(&new_state),);
                for i in new_state.clone() {
                    println!("{}", i);
                }
            }
        }
        for state in &lr_items.states {
            println!("{:?}", state);
        }
        println!("count: {:?}", lr_items.states.iter().count());

        lr_items
    }
}
pub struct LRTable {
    pub actions_table: HashMap<State, Row<TerminalSymbol, Action>>,
    pub goto_table: HashMap<State, Row<NonterminalSymbol, Goto>>,
}

impl LRTable {
    pub fn new(productions: &Productions) -> Self {
        let first_follow = FirstFollowSets::new(productions);

        println!("first");
        for set in &first_follow.first {
            println!("{:?}", set);
        }
        println!("follow");
        for set in &first_follow.follow {
            println!("{:?}", set);
        }

        let lr_items = LRItems::new(productions, &first_follow.follow);
        Self {
            actions_table: HashMap::new(),
            goto_table: HashMap::new(),
        }
    }
}
