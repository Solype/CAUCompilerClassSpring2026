mod input;
mod ruleparser;
mod slr;
mod error;
mod helper;

use crate::{
    input::{Parameters, get_rule_and_input},
    ruleparser::{reader::parse_rules, regex_tokenizer::RegexTokenizer, rules_and_tokens::{Token, TokenMetadata}, structs::TokenManager},
    slr::{parser::TokenWithMetadata, tree::TreeNode},
};

use owo_colors::OwoColorize;

pub fn display_node(
    node: &TreeNode<TokenWithMetadata>,
    prefix: String,
    is_last: bool,
    use_regex: bool,
    token_manager: &TokenManager,
) {
    let token_name = token_manager.get_token_name(node.value.token.id());

    let connector = if is_last {
        "└──"
    } else {
        "├──"
    };

    if let Token::Term(_) = node.value.token && use_regex {
        println!(
            "{}{} {} token({})",
            prefix,
            connector.bright_blue(),
            node.value.metadata.str.green(),
            token_name.white().bold()
        );
    } else {
        println!(
            "{}{} {}",
            prefix,
            connector.bright_blue(),
            token_name.white().bold()
        );
    }

    let child_prefix = if is_last {
        format!("{}    ", prefix)
    } else {
        format!("{}│   ", prefix)
    };

    let len = node.children.len();

    for (i, child) in node.children.iter().enumerate() {
        display_node(
            child,
            child_prefix.clone(),
            i == len - 1,
            use_regex,
            token_manager,
        );
    }
}

fn cook_tokens(input: &Parameters, rules: &TokenManager) -> Result<Vec<(String, TokenMetadata)>, String>
{
    let mut reg = RegexTokenizer::new();

    let cooked_tokens = if input.use_regex {
        if let Some(regex_file_content) = &input.regex {
            reg.parse_file_content(&regex_file_content)?
        } else {
            reg.set_default_rules();
        }
        reg.tokenize(&input.input_tokens)?
    } else {
        rules.scan_tokens(&input.input_tokens)?
    };

    return Ok(cooked_tokens);
}


fn run() -> Result<(), String> {
    let input = get_rule_and_input()?;

    let mut rules = ruleparser::structs::TokenManager::new();
    rules.add_productions(&parse_rules(&input.rules)?)?;

    let cooked_tokens = cook_tokens(&input, &rules)?;
    let tokens = rules.wrap_cooked_token(&cooked_tokens)?;

    let mut parser = slr::parser::Parser::new(&rules.get_production());
    let tree = parser.parse(tokens)?;

    display_node(&tree.root, String::new(), true, input.use_regex, &rules);

    Ok(())
}

#[allow(unreachable_code)]
fn main() -> Result<(), String> {

    if let Err(e) = run() {
        eprintln!("{}", e);
        return Err("Due to previous error, the program will stop".to_string());
    }
    Ok(())
}
