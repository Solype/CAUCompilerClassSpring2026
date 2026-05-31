use crate::error::file_error;

use super::rules_and_tokens::RawProduction;

fn process_line(infos: (usize, impl Into<String>)) -> Result<Option<RawProduction>, String> {
    let (line_number, buffer) = infos;
    let raw = buffer.into();

    let line: Vec<String> = raw
        .split(&[' ', '\t'])
        .filter(|x| !x.is_empty())
        .map(|x| x.to_string())
        .collect();

    // Exemple attendu :
    // EXPR -> TERM PLUS TERM
    if line.is_empty() {
        return Ok(None);
    }

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

    // Vérifie la présence de ->
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
    let rules: Vec<Option<RawProduction>> = buffer
        .split('\n')
        .enumerate()
        .map(process_line)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(rules.into_iter().flatten().collect())
}
