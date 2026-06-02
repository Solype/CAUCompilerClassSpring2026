mod error;
mod helper;
mod input;
mod ruleparser;
mod slr;

use std::collections::HashSet;

use crate::{
    input::{Parameters, get_rule_and_input},
    ruleparser::{
        reader::parse_rules,
        regex_tokenizer::RegexTokenizer,
        rules_and_tokens::{NonTerm, Sym, Term, Token, TokenMetadata},
        structs::{END, TokenManager, UNDEFINED},
    },
    slr::{
        parser::TokenWithMetadata,
        table::{Action, SLRTable},
        tree::TreeNode,
    },
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

    let connector = if is_last { "└──" } else { "├──" };

    if let Token::Term(_) = node.value.token
        && use_regex
    {
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

/// Can be used to display SLR Table in markdown format
#[allow(dead_code)]
fn display_slr_table(slr_table: &SLRTable, token_manager: &TokenManager) {
    let (nonterm_set, term_set): (HashSet<(&String, &usize)>, HashSet<(&String, &usize)>) =
        token_manager
            .token
            .iter()
            .partition(|(_, id)| token_manager.non_terminal_token.contains(id));

    let mut nonterms: Vec<_> = nonterm_set.into_iter().collect();
    let mut terms: Vec<_> = term_set.into_iter().collect();

    nonterms.sort_by_key(|(_, id)| *id);
    terms.sort_by_key(|(_, id)| *id);

    let mut terms: Vec<_> = terms
        .into_iter()
        .filter(|(_, id)| **id > UNDEFINED.0.0)
        .collect();

    // Push END ($) to the end
    let end_str = "$".to_string();
    terms.push((&end_str, &END.0.0));

    print!("| State |I| ACTION |");

    for _ in 1..terms.len() {
        print!(" |");
    }

    print!("I | GOTO |");

    for _ in 2..nonterms.len() {
        print!(" |");
    }

    println!();

    // Separator row
    print!("|---|---|");
    for _ in &terms {
        print!("---|");
    }
    for _ in &nonterms {
        print!("---|");
    }
    println!();

    // Token row
    print!("| | I |");
    for (name, _) in &terms {
        print!(" {} |", name);
    }
    print!(" I |");

    for (name, _) in &nonterms {
        print!(" {} |", name);
    }

    println!();

    // Rows
    for state_id in 0..slr_table.lr_items.states.len() {
        print!("| {} | I |", state_id);

        // ACTION columns
        for (_, id) in &terms {
            let term = Term(Sym(**id));

            let cell = match slr_table.actions.get(&(state_id, term)) {
                Some(Action::Shift(next)) => format!("s{}", next),
                Some(Action::Reduce(rule)) => format!("r{}", rule),
                Some(Action::Accept) => "acc".to_string(),
                None => String::new(),
            };

            print!(" {} |", cell);
        }
        print!(" I |");

        // GOTO columns
        for (_, id) in &nonterms {
            let nt = NonTerm(Sym(**id));

            let cell = slr_table
                .gotos
                .get(&(state_id, nt))
                .map(|goto| goto.to_string())
                .unwrap_or_default();

            print!(" {} |", cell);
        }

        println!();
    }
}

fn cook_tokens(
    input: &Parameters,
    rules: &TokenManager,
) -> Result<Vec<(String, TokenMetadata)>, String> {
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


/**
 * Executes the complete compiler/parsing pipeline.
 *
 * Pipeline stages:
 *   - load CLI parameters and source files
 *   - build grammar productions
 *   - tokenize the input source
 *   - convert tokens into parser terminals
 *   - generate the SLR parser
 *   - parse the token stream
 *   - display the resulting parse tree
 *
 * Returns the first encountered compilation or parsing error.
 */
fn run() -> Result<(), String> {
    let input = get_rule_and_input()?;

    let mut rules = ruleparser::structs::TokenManager::new();
    rules.add_productions(&parse_rules(&input.rules)?)?;

    let cooked_tokens = cook_tokens(&input, &rules)?;
    let tokens = rules.wrap_cooked_token(&cooked_tokens)?;

    let mut parser = slr::parser::Parser::new(&rules.get_production());
    let tree = parser.parse(tokens)?;

    display_node(&tree.root, String::new(), true, input.use_regex, &rules);
    // display_slr_table(&parser.slr_table, &rules);

    Ok(())
}

/**
 * Program entry point.
 *
 * Runs the compiler pipeline and prints formatted diagnostics
 * if an error occurs before exiting the program.
 */
#[allow(unreachable_code)]
fn main() -> Result<(), String> {
    if let Err(e) = run() {
        eprintln!("{}", e);
        return Err("Due to previous error, the program will stop".to_string());
    }
    Ok(())
}
