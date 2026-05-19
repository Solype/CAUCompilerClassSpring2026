use core::panic;
use std::{
    fmt::Debug,
    hash::Hash, vec,
};

use super::{Rule, rules::RuleTree, rules::RuleToApply, rules::RuleTreeBranch};

#[derive(Default, Debug, Clone)]
pub struct ParsedTreeBranch<T>
where
    T: PartialEq + Default + Clone + Hash + Eq + Debug,
{
    pub value: T,
    pub childrens: Vec<ParsedTreeBranch<T>>
}

impl <T> ParsedTreeBranch<T>
where
    T: PartialEq + Default + Clone + Hash + Eq + Debug,
{
    pub fn new(val: T) -> Self
    {
        Self { value: val, childrens: vec![] }
    }
    pub fn display(&self)
    {
        self.display_internal("", true);
    }
    
    fn display_internal(&self, prefix: &str, is_last: bool)
    {
        let connector = if is_last {
            "└──"
        } else {
            "├──"
        };
    
        println!("{}{} {:?}", prefix, connector, self.value);
    
        let new_prefix = if is_last {
            format!("{}    ", prefix)
        } else {
            format!("{}│   ", prefix)
        };
    
        let len = self.childrens.len();
    
        for (i, child) in self.childrens.iter().enumerate() {
            child.display_internal(&new_prefix, i == len - 1);
        }
    }
}



#[derive(Debug)]
pub struct Stack<'a, T>
where
    T: PartialEq + Default + Clone + Hash + Eq + Debug,
{
    stack: Vec<ParsedTreeBranch<T>>,
    current_rule_branch: &'a RuleTreeBranch<T>,
}


impl<'a, T> Stack<'a, T>
where
    T: PartialEq + Default + Clone + Hash + Eq + Debug,
{
    pub(super) fn new(branch: &'a RuleTreeBranch<T>) -> Self
    {
        Self {
            stack: Vec::new(),
            current_rule_branch: branch,
        }
    }
}


#[derive(Default, Debug)]
pub struct Parser<T>
where
    T: PartialEq + Default + Clone + Hash + Eq + Debug,
{
    rules: RuleTree<T>,
}

impl<T> Parser<T>
where
    T: PartialEq + Default + Clone + Debug + Hash + Eq,
{
    pub fn new() -> Self
    {
        Self::default()
    }

    pub fn add_rules(&mut self, rules: Vec<Rule<T>>) -> &mut Self
    {
        for rule in rules {
            self.add_rule(rule);
        }
        return self;
    }

    pub fn add_rule(&mut self, rule: Rule<T>) -> &mut Self
    {
        if rule.len() == 0 {
            self.rules.add_epsilon_possibility(rule.output);
            return self;
        }
        self.rules.add_rule(rule);
        return self;
    }

    pub fn parse_sequence(&mut self, sequence: Vec<T>) -> &mut Self
    {
        let mut stacks: Vec<Stack<T>> = vec![];
        println!("SEQUENCE : {:?}", sequence);

        let mut input: Vec<ParsedTreeBranch<T>> = sequence.iter().rev().map(|x| ParsedTreeBranch::<T>::new(x.clone())).collect();

        while let Some(elem) = input.pop() {
            print!("{:?} ::: [", elem.value);
            for e in input.iter().rev() {
                print!(" {:?},", e.value);
            }
            println!(" ]");

            if stacks.is_empty() {
                println!("stack was empty");
                stacks.push(Stack::<T>::new(self.rules.get_root()));
            }

            println!("\n\n");
            println!("elem : {:?}", elem);
            
            let (action, next_branch) = {
                let current_stack = stacks.last_mut().unwrap();
                println!("current rule tree:");
                self.rules.parse_single_token(
                    elem.value.clone(),
                    current_stack.current_rule_branch,
                )
            };

            match action {

                RuleToApply::Error => {
                    println!("error found");
                    if self.rules.get_root().get_child(&elem.value).is_some() {
                        input.push(elem);
                        stacks.push(Stack::<T>::new(self.rules.get_root()));
                    } else {
                        panic!("Bad input token : {:?}", &elem.value);
                    }

                }

                RuleToApply::Reduce => {
                    println!("reduce found");
                    
                    let children = {
                        let current_stack = stacks.last().unwrap();
                        current_stack.stack.clone()
                    };
                    print!("reducing :");
                    for token in children.iter() {
                        print!("{:?} ", token.value);
                    }
                    println!("\nto : {:?}", next_branch.get_value().unwrap());
                    
                    input.push(ParsedTreeBranch {
                        value: next_branch.get_value().unwrap(),
                        childrens: children,
                    });

                    stacks.pop();
                }

                RuleToApply::Accept => {}

                RuleToApply::Shift => {
                    let current_stack = stacks.last_mut().unwrap();

                    current_stack.current_rule_branch = next_branch;

                    current_stack.stack.push(elem);
                }
            }
        }

        println!("stacks : {:?}", stacks);

        self
    }

    #[allow(dead_code)]
    pub fn display_rules(&self) -> &Self
    {
        self.rules.display();
        return self;
    }
}
