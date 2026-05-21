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

    Some(RawProduction::new(
        line[0].clone(),
        line[2..].to_vec(),
    ))
}

pub fn parse_rules(buffer: &String) -> Vec<RawProduction> {
    buffer
        .split('\n')
        .filter_map(process_line)
        .collect()
}
