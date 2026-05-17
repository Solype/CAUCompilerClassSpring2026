use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

#[derive(Debug)]
pub struct Rule<T>
where
    T: PartialEq + Default,
{
    input: Vec<T>,
    output: T,
}

impl<T> Rule<T>
where
    T: PartialEq + Default + Clone + Debug,
{
    pub fn new(output: T, input: Vec<T>) -> Self {
        Self {
            input: input,
            output: output,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.input.len()
    }
}

#[derive(Default, Debug)]
struct RuleTreeBranch<T>
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
    pub fn get_and_add_child_mut(&mut self, matching_value: &T) -> &mut Self {
        self.children
            .entry(matching_value.clone())
            .or_insert(RuleTreeBranch {
                output: None,
                children: HashMap::new(),
                deepness: self.deepness + 1,
            })
    }

    #[inline]
    pub fn get_child(&self, matching_value: &T) -> Option<&Self> {
        let t = self.children.get(matching_value);
        // println!(
        //     "v: {:?}",
        //     // matching_value, self.children,
        //     t
        // );

        t
    }

    pub fn display(&self) {
        println!("{}{:?}", "\t".repeat(self.deepness), self.output);
        for (key, val) in self.children.iter() {
            println!("{}{:?}", "\t".repeat(self.deepness + 1), key);
            val.display();
        }
    }
}

#[derive(Debug, PartialEq)]
struct ReturnSequence<T> {
    pub value: T,
    pub size: usize,
}

#[derive(Default, Debug)]
struct RuleTree<T>
where
    T: PartialEq + Default + Clone + Hash + Eq + Debug,
{
    can_be_epsilon: HashSet<T>,
    root: Option<RuleTreeBranch<T>>,
}

impl<T> RuleTree<T>
where
    T: PartialEq + Default + Clone + Hash + Eq + Debug,
{
    pub fn add_epsilon_possibility(&mut self, token: T) -> &mut Self {
        if self.root.is_some() {
            eprintln!(
                "\x1b[93m[WARNING]\x1b[0m epsilon possibilities should be defined before inserting rules into the parser tree."
            );
        }
        self.can_be_epsilon.insert(token);
        return self;
    }

    pub fn add_rule(&mut self, rule: Rule<T>) -> &mut Self {
        for (i, token) in rule.input.clone().iter().enumerate() {
            if self.can_be_epsilon.get(&token).is_some() {
                let mut new_input = rule.input.clone();
                new_input.remove(i);
                self.add_rule(Rule {
                    output: rule.output.clone(),
                    input: new_input,
                });
            }
        }

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
        if branch.output.is_some() {
            eprintln!(
                "\x1b[93m[WARNING]\x1b[0m Rule ignored because another rule already matches this sequence."
            );
            return self;
        }
        branch.output = Some(rule.output);
        return self;
    }

    pub fn display(&self) -> &Self {
        println!("Tree:");
        if let Some(tree) = &self.root {
            tree.display();
        }
        return self;
    }

    pub fn parse_sequence<'a, I>(
        &self,
        sequence: &mut std::iter::Peekable<I>,
    ) -> Option<ReturnSequence<T>>
    where
        I: Iterator<Item = &'a T>,
        T: 'a,
    {
        if self.root.is_none() {
            return None;
        }

        let mut ret_sequence: Option<ReturnSequence<T>> = None;
        let mut branch = self.root.as_ref().unwrap();
        let mut ndx: usize = 0;

        while let Some(elem) = sequence.peek() {
            println!("current token {:?}", elem);
            if let Some(child) = branch.get_child(elem) {
                branch = child;
                if let Some(val) = branch.output.clone() {
                    ret_sequence = Some(ReturnSequence {
                        value: val,
                        size: ndx + 1,
                    });
                }
                sequence.next();
                ndx += 1;
            } else {
                return ret_sequence;
            }
        }
        ret_sequence
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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_rules(&mut self, rules: Vec<Rule<T>>) -> &mut Self {
        for rule in rules {
            self.add_rule(rule);
        }
        return self;
    }

    pub fn add_rule(&mut self, rule: Rule<T>) -> &mut Self {
        if rule.len() == 0 {
            self.rules.add_epsilon_possibility(rule.output);
            return self;
        }
        self.rules.add_rule(rule);
        return self;
    }

    pub fn parse_sequence(&self, sequence: &mut Vec<T>) -> &Self {
        let mut ndx: usize = 0;
        while ndx < sequence.len() {
            if let Some(expr) = self
                .rules
                .parse_sequence(&mut sequence.iter().skip(ndx).peekable())
            {
                // println!("Expression: {:?}", expr);
                sequence.splice(ndx..ndx + expr.size, vec![expr.value]);
                ndx = 0;
                // println!("new sequence: {:?}", sequence);
            } else {
                ndx += 1;
            }
        }
        return self;
    }

    pub fn display_rules(&self) -> &Self {
        self.rules.display();
        return self;
    }
}
