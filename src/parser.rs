use std::{collections::HashMap, fmt::Debug};

#[derive(Debug)]
pub struct Rule<T>
where
    T: PartialEq + Default,
{
    input: Vec<T>,
    output: T,
}

impl <T> Rule<T>
where
    T: PartialEq + Default + Clone + Debug
{
    pub fn new(output: T, input: Vec<T>) -> Self
    {
        Self {
            input: input,
            output: output,
        }
    }

    #[inline]
    pub fn len(&self) -> usize
    {
        self.input.len()
    }

    pub fn is_matching(&self, expr: &Vec<T>) -> bool
    {
        if self.len() > expr.len() {
            return false;
        }
        let difference = expr.len() - self.input.len();
        for (a, b) in self.input.iter().zip(expr.iter().nth(difference)) {
                if a != b {
                    return false;
                }
        }
        return true;
    }

    pub fn expression(&self) -> T
    {
        self.output.clone()
    }
}


#[derive(Default, Debug)]
pub struct Parser<T>
where
    T: PartialEq + Default,
{
    stack: Vec<T>,
    rules: HashMap<usize, Vec<Rule<T>>>,
    biggest_rule: usize
}

impl<T> Parser<T>
where
    T: PartialEq + Default + Clone + Debug,
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

    pub fn add_rule(&mut self, rule : Rule<T>) -> &mut Self
    {
        let size = rule.len();
        self.rules.entry(size).or_insert_with(Vec::new).push(rule);

        if size > self.biggest_rule {
            self.biggest_rule = size
        }
        return self;
    }

    fn activate_rule(stack: &mut Vec<T>, rule: &Rule<T>, size : usize)
    {
        for _ in 0..size {
            stack.pop();
        }
        stack.push(rule.expression());
    }

    fn match_rule_of_size(&mut self, size : usize) -> &mut Self
    {
        if let Some(rules) = self.rules.get(&size) {
            for rule in rules {
                if rule.is_matching(&self.stack) {
                    Self::activate_rule(&mut self.stack, rule, size);
                    self.match_rule();
                    return self
                }
            }
        }
        return self;
    }

    fn match_rule(&mut self) -> &mut Self{
        for size in (1..=self.biggest_rule).rev() {
            if self.stack.len() < size {
                return self;
            }
            self.match_rule_of_size(size);
        }
        return self;
    }

    pub fn push_expr(&mut self, expr : T) -> &mut Self
    {
        self.stack.push(expr);
        self.match_rule();
        return self;
    }

    #[inline]
    pub fn is_valid(&self) -> bool
    {
        self.stack.len() == 1
    }

    pub fn display_stack(&self) -> &Self
    {
        println!("{:?}", self.stack);
        self
    }
}
