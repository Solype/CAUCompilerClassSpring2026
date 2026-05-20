use core::fmt;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    fs::Metadata,
    hash::Hash,
    iter::Map,
    os::linux::raw::stat,
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
type Symbols = Vec<Symbol>;

type Row<T, K> = HashMap<T, K>;

pub type Production = Vec<Symbols>;
pub type Productions = BTreeMap<NonterminalSymbol, Production>;
pub struct Rules {
    pub start: NonterminalSymbol,
    pub productions: Productions,
}

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
    fn new(rules: &Rules) -> Self {
        let first_table = Self::_build_first(rules);
        let follow_table = Self::_build_follow(&first_table, rules);
        Self {
            first: first_table,
            follow: follow_table,
        }
    }

    fn _build_first(rules: &Rules) -> FirstTable {
        let mut table = FirstTable::new();
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
        let mut table = FollowTable::new();
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

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Debug)]
pub struct LRItem {
    expr: NonterminalSymbol,
    rule: Symbols,
    dot: usize,
}
impl fmt::Display for LRItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ->", self.expr)?;

        for (i, symbol) in self.rule.iter().enumerate() {
            if i == self.dot {
                write!(f, " .")?;
            }

            write!(f, " {:?}", symbol)?;
        }

        if self.dot == self.rule.len() {
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
    fn _closure_items(nt: &NonterminalSymbol, dot: usize, rules: &Rules) -> Vec<LRItem> {
        rules
            .productions
            .get(&nt)
            .unwrap()
            .iter()
            .map(|symbols| LRItem {
                expr: nt.clone(),
                rule: symbols.clone(),
                dot: dot,
            })
            .collect()
    }
    fn _closure(nt: &NonterminalSymbol, dot: usize, rules: &Rules, mut state: &mut State) {
        let closure_items = Self::_closure_items(nt, dot, rules);
        let mut to_expands = vec![];
        for item in &closure_items {
            if state.insert(item.clone())
                && let Some(s) = item.rule.get(dot)
            {
                to_expands.push(s.clone());
            }
        }

        while !to_expands.is_empty() {
            let Symbol::Nonterminal(nt) = to_expands.pop().unwrap() else {
                continue;
            };
            Self::_closure(&nt, 0, &rules, &mut state);
        }
    }

    fn _closure2(mut state: State, rules: &Rules) -> State {
        let mut changed = true;
        let mut to_add = vec![];
        while changed {
            changed = false;
            for item in &state {
                if let Some(Symbol::Nonterminal(nt)) = item.rule.get(item.dot) {
                    to_add.extend(Self::_closure_items(nt, 0, rules));
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
            if item.rule.get(item.dot) == Some(symbol) {
                let mut new_item = item.clone();
                new_item.dot = item.dot + 1;
                new_state.insert(new_item.clone());
            }
        }

        new_state
    }

    pub fn new(rules: &Rules, follows: &FollowTable) -> Self {
        let mut start_state = State::from([LRItem {
            expr: NonterminalSymbol::Start,
            rule: vec![Symbol::Nonterminal(rules.start.clone())],
            dot: 0,
        }]);

        start_state = Self::_closure2(start_state, &rules);

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
                    .map(|item| item.rule.get(item.dot))
                    .flatten(),
            );
            for next_symbol in next_symbols {
                println!(
                    "goto state {:?} symbol: {:?}\n",
                    lr_items.states.get_index_of(&from_state),
                    next_symbol
                );
                let mut new_state = Self::_goto(&from_state, next_symbol);
                new_state = Self::_closure2(new_state, rules);
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
    pub fn new(rules: &Rules) -> Self {
        let first_follow = FirstFollowSets::new(rules);

        println!("first");
        for set in &first_follow.first {
            println!("{:?}", set);
        }
        println!("follow");
        for set in &first_follow.follow {
            println!("{:?}", set);
        }

        let lr_items = LRItems::new(rules, &first_follow.follow);
        Self {
            actions_table: HashMap::new(),
            goto_table: HashMap::new(),
        }
    }
}
