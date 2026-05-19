use std::{
    collections::{HashMap, HashSet}, fmt::Debug, hash::Hash,
};

use super::parser::ParsedTreeBranch;

#[derive(Debug)]
pub struct Rule<T>
where
    T: PartialEq + Default,
{
    pub(super) input: Vec<T>,
    pub(super) output: T,
}

impl<T> Rule<T>
where
    T: PartialEq + Default + Clone + Debug,
{
    pub fn new(output: T, input: Vec<T>) -> Self
    {
        Self { input: input, output: output, }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.input.len()
    }
}

#[derive(Default, Debug, PartialEq)]
pub(super) struct RuleTreeBranch<T>
where
    T: PartialEq + Default + Clone + Hash + Eq,
{
    output: Option<T>,
    children: HashMap<T, RuleTreeBranch<T>>,
    deepness: usize,
}

impl<T> RuleTreeBranch<T>
where
    T: PartialEq + Default + Clone + Hash + Eq + Debug,
{
    pub fn get_and_add_child_mut(&mut self, matching_value: &T) -> &mut Self
    {
        self.children
            .entry(matching_value.clone())
            .or_insert(RuleTreeBranch {
                output: None,
                children: HashMap::new(),
                deepness: self.deepness + 1,
            })
    }

    #[inline]
    pub fn get_value(&self) -> Option<T>
    {
        return self.output.clone();
    }

    #[inline]
    pub fn is_last(&self) -> bool
    {
        self.children.is_empty()
    }

    #[inline]
    pub fn get_child(&self, matching_value: &T) -> Option<&Self> {
        self.children.get(matching_value)
    }

    #[allow(dead_code)]
    pub fn display(&self)
    {
        self.display_internal("", true);
    }

    #[allow(dead_code)]
    fn display_internal(&self, prefix: &str, is_last: bool)
    {
        let new_prefix = if is_last { format!("{}    ", prefix) } else { format!("{}│   ", prefix) };

        let len = self.children.len();

        for (i, (key, child)) in self.children.iter().enumerate() {
            let child_last = i == (len - 1);
            let child_connector = if child_last {"└──"} else {"├──"};

            match &child.output {
                Some(val) => {
                    println!("{}{} {:?} -> {:?}", new_prefix, child_connector, key, val);
                }
                None => {
                    println!("{}{} {:?}",new_prefix,child_connector,key);
                }
            }
            child.display_internal(&new_prefix, child_last);
        }
    }
}

pub enum RuleToApply {
    Shift,
    Reduce,
    Accept,
    Error,
}

#[derive(Default, Debug)]
pub(super) struct RuleTree<T>
where
    T: PartialEq + Default + Clone + Hash + Eq + Debug,
{
    can_be_epsilon: HashSet<T>,
    root: Option<RuleTreeBranch<T>>,
    final_expr: T
}

impl<T> RuleTree<T>
where
    T: PartialEq + Default + Clone + Hash + Eq + Debug,
{
    #[inline]
    pub fn get_root(&self) -> &RuleTreeBranch<T>
    {
        match &self.root {
            None => panic!("Cannot give root cause no root exists for rules, no rule exists"),
            Some(val) => val
        }
    }

    #[inline]
    pub fn set_final(&mut self, expr: T) -> &mut Self
    {
        self.final_expr = expr; self
    }

    pub fn add_epsilon_possibility(&mut self, token: T) -> &mut Self
    {
        if self.root.is_some() {
            eprintln!(
                "\x1b[93m[WARNING]\x1b[0m epsilon possibilities should be defined before inserting rules into the parser tree."
            );
        }
        self.can_be_epsilon.insert(token);
        return self;
    }

    pub fn add_rule(&mut self, rule: Rule<T>) -> &mut Self
    {
        print!("adding the rule : {:?} <- ", rule.output);
        for elem in &rule.input {
            print!("{:?} ", elem);
        }
        println!(";");

        let mut branch = match self.root.as_mut() {
            Some(root) => root,
            None => {
                self.root = Some(RuleTreeBranch::default());
                self.root.as_mut().unwrap()
            }
        };

        for token in rule.input {
            branch = branch.get_and_add_child_mut(&token);
        }
        if let Some(val) = &branch.output {
            if *val != rule.output {
                eprintln!(
                    "\x1b[93m[WARNING]\x1b[0m Rule ignored because another rule already matches this sequence."
                );
            }
            return self;
        }
        branch.output = Some(rule.output);
        return self;
    }

    #[allow(dead_code)]
    pub fn display(&self) -> &Self
    {
        println!("Tree:");
        if let Some(tree) = &self.root {
            tree.display();
        }
        return self;
    }


    pub fn parse_single_token<'a>(
        &'a self,
        token: T,
        mut branch: &'a RuleTreeBranch<T>,
    ) -> (RuleToApply, &'a RuleTreeBranch<T>)
    {
        loop {
            if let Some(new_branch) = branch.get_child(&token) {
                if new_branch.is_last() {
                    return (RuleToApply::Reduce, new_branch);
                } else {
                    return (RuleToApply::Shift, new_branch);
                }
            }
            let mut epsilon_branch = None;

            for epsilon in &self.can_be_epsilon {
                if let Some(child) = branch.get_child(epsilon) {
                    epsilon_branch = Some(child);
                    break;
                }
            }
            if let Some(next_branch) = epsilon_branch {
                branch = next_branch;
                continue;
            }
            if branch.output.is_some() {
                return (RuleToApply::Reduce, branch);
            } else {
                branch.display();
                return (RuleToApply::Error, branch);
            }
        }
    }
}
