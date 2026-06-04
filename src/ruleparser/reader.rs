use crate::error::file_error;

use super::rules_and_tokens::RawProduction;

/**
 * This function is used to parse a single line inside of the file where the CFG is defined.
 * @param info holds 2 values inside a tuple : the line number (usize) and the line (in form of a string)
 */
use regex::Regex;

fn process_line(
    infos: (usize, impl Into<String>),
) -> Result<Option<RawProduction>, String>
{
    let (line_number, buffer) = infos;
    let raw = buffer.into();
    println!("line: {}", raw);

    let line = raw.trim();

    if line.is_empty() {
        return Ok(None);
    }

    // NONTERM -> SYMBOL SYMBOL SYMBOL
    // NONTERM -> ''
    let re = Regex::new(
        r"^([A-Z][A-Z0-9_]*)\s*->\s*(('')|([A-Z][A-Z0-9_]*|[a-z][a-z0-9_]*)(\s+([A-Z][A-Z0-9_]*|[a-z][a-z0-9_]*))*)$"
    ).unwrap();

    let captures = match re.captures(line) {
        Some(c) => c,
        None => {
            return Err(file_error(
                &"Rules".to_string(),
                line_number,
                1,
                line.len(),
                &raw,
                &"Invalid production format".to_string(),
            ));
        }
    };


    let lhs = captures.get(1).unwrap().as_str().to_string();

    let rhs = captures.get(2).unwrap().as_str();

    let inputs = if rhs == "''" {
        vec![]
    } else {
        rhs.split_whitespace()
            .map(String::from)
            .collect()
    };

    Ok(Some(RawProduction::new(lhs, inputs)))
}

pub fn parse_rules(buffer: &String) -> Result<Vec<RawProduction>, String> {
    // we create a vector by following those steps:
    // - we transform buffer into an array of line, with \n being a new line
    // - we transform this list of line into an array of enumerated line : ["line 1", "line 2"] -> [(0, "line 1"), (1, "line 2")]
    // - on each line, we apply the function "process_line", and replace the content of the array with the result of "process_line"
    // - then, on the result of each process line that are in the new array,
    //      we get all the value of the non error lines, but throw an error if there is an error among the process line result
    let rules: Vec<Option<RawProduction>> = buffer
        .split('\n')
        .enumerate()
        .map(process_line)
        .collect::<Result<Vec<_>, _>>()?;

    // we return the list given once all the empty line are removed, which is the use of "flatten"
    Ok(rules.into_iter().flatten().collect())
}
