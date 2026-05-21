use super::structs::RawProduction;

fn process_line(buffer: impl Into<String>) -> Option<RawProduction> {
    let line: Vec<String> = buffer
        .into()
        .split(&[' ', '\t'])
        .filter(|x| !x.is_empty())
        .map(|x| x.to_string())
        .collect();

    if line.len() < 3 {
        return None;
    }

    let inputs = if line.len() == 3 && line[2] == "''" {
        vec![]
    } else {
        line[2..].to_vec()
    };

    Some(RawProduction::new(
        line[0].clone(),
        inputs,
    ))
}

pub fn parse_rules(buffer: &String) -> Vec<RawProduction> {
    buffer
        .split('\n')
        .filter_map(process_line)
        .collect()
}
