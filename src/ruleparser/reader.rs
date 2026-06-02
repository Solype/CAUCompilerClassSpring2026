use crate::error::file_error;

use super::rules_and_tokens::RawProduction;

/**
 * This function is used to parse a single line inside of the file where the CFG is defined.
 * @param info holds 2 values inside a tuple : the line number (usize) and the line (in form of a string)
 */
fn process_line(infos: (usize, impl Into<String>),) -> Result<Option<RawProduction>, String>
{
    // split the information hold by the parameter in dedicated variable so it's easier to manipulate them
    // buffer.into() to automatically convert the line into a String object
    let (line_number, buffer) = infos;
    let raw = buffer.into();

    // We preprocess the line to remove all the white spaces
    let line: Vec<String> = raw
        .split(&[' ', '\t'])
        .filter(|x| !x.is_empty())
        .map(|x| x.to_string())
        .collect();

    // if the line is empty, we just skip the line, we do not need to throw an error.
    if line.is_empty() {
        return Ok(None);
    }

    // expected term :
    // EXPR -> TERM PLUS TERM

    // if the line is less than three words, and is not empty, it means that the form is either EXPR or EXPR ->
    // those are not valid input.
    if line.len() < 3 {
        let line_size = raw.len();
        let err_message =
            "Each rule must have at least have 3 arguments, put '' in case of empty arguments"
                .to_string();
        return Err(file_error(
            &"Rules".to_string(),
            line_number,
            line_size,
            1,
            &raw,
            &err_message,
        ));
    }

    // check the presence of "->" in second position.
    if line[1] != "->" {
        let col = raw.find(&line[1]).unwrap_or(0) + 1;
        let err_message = format!("Expected symbol : '->', found: '{}'", line[1]);
        return Err(file_error(
            &"Rules".to_string(),
            line_number,
            col,
            line[1].len(),
            &raw,
            &err_message,
        ));
    }

    let inputs = if line.len() == 3 && line[2] == "''" {
        vec![]
    } else {
        line[2..].to_vec()
    };

    Ok(Some(RawProduction::new(line[0].clone(), inputs)))
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
