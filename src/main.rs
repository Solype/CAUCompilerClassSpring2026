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

fn main() -> Result<(), String> {
    let input = get_rule_and_input();

    let mut rules = ruleparser::structs::TokenManager::new();
    let productions = match parse_rules(&input.rules) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("{}", e);
            return Err("Due to previous error, the program will stop".to_string());
        }
    };
    match rules.add_productions(&productions) {
        Err(e) => {
            eprintln!("{}", e);
            return Err("Due to previous error, the program will stop".to_string());
        }
        _ => {}
    }

    let tokens = match rules.scan_tokens(&input.input_tokens) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("{}", e);
            return Err("Due to previous error, the program will stop".to_string());
        }
    };

    let mut parser = slr::parser::Parser::new(&rules.get_production());

    match parser.parse(tokens) {
        Ok(tree) => display_node(&tree.root, 0, &rules),
        Err(e) => eprintln!("{}", e),
    }
    return Ok(())
}
