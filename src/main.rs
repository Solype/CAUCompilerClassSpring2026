mod input;
mod slr;
mod ruleparser;


use crate::{input::get_rule_and_input, ruleparser::{reader::parse_rules}};

fn main() {
    let mut rules = ruleparser::structs::TokenManager::new();
    let input = get_rule_and_input();
    rules.add_productions(&parse_rules(&input.rules));
    let parser = slr::parser::LRTable::new(&rules.get_production());
    println!("input tokens: {}", input.input_tokens);
    println!("{}", parser);
    println!("{}", rules);
    // println!("{:?}", rules);
}
