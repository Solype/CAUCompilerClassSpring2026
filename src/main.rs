mod input;
mod ruleparser;
mod slr;
mod error;

use crate::{
    input::get_rule_and_input,
    ruleparser::{reader::parse_rules, structs::TokenManager},
    slr::{parser::TokenWithMetadata, tree::TreeNode},
};

fn display_node(node: &TreeNode<TokenWithMetadata>, depth: usize, token_manager: &TokenManager) {
    println!("{}", token_manager.get_token_name(node.value.token.id()));
    for child in &node.children {
        print!("{}   \\--- ", "\t".repeat(depth),);
        display_node(child, depth + 1, token_manager);
    }
}

fn run() -> Result<(), String> {
    let input = get_rule_and_input()?;

    let mut rules = ruleparser::structs::TokenManager::new();

    let productions = parse_rules(&input.rules)?;

    rules.add_productions(&productions)?;

    let tokens = rules.scan_tokens(&input.input_tokens)?;

    let mut parser = slr::parser::Parser::new(&rules.get_production());

    let tree = parser.parse(tokens)?;

    display_node(&tree.root, 0, &rules);

    Ok(())
}

fn main() -> Result<(), String> {
    if let Err(e) = run() {
        eprintln!("{}", e);
        return Err("Due to previous error, the program will stop".to_string());
    }
    Ok(())
}
