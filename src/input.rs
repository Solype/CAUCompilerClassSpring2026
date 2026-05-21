use std::{
    env,
    fs,
    io::{self, Read},
};

use crate::ruleparser::default::DEFAULT_RULES;

pub struct Parameters {
    pub rules: String,
    pub input_tokens: String,
}

fn read_source(path: Option<&str>) -> String {
    let mut buffer = String::new();

    match path {
        Some(file_path) => {
            let mut file = fs::File::open(file_path)
                .unwrap_or_else(|_| panic!("cannot open file: {file_path}"));

            file.read_to_string(&mut buffer)
                .unwrap_or_else(|_| panic!("cannot read file: {file_path}"));
        }

        None => {
            io::stdin()
                .read_to_string(&mut buffer)
                .expect("cannot read stdin");
        }
    }

    buffer
}

pub fn get_rule_and_input() -> Parameters {
    let args: Vec<String> = env::args().collect();

    let mut rules_file: Option<&str> = None;
    let mut token_file: Option<&str> = None;

    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-r" => {
                if rules_file.is_some() {
                    panic!("duplicate -r option");
                }

                if i + 1 >= args.len() {
                    panic!("-r requires a file path");
                }

                rules_file = Some(args[i + 1].as_str());
                i += 2;
            }

            file => {
                if token_file.is_some() {
                    panic!("too many positional arguments");
                }

                token_file = Some(file);
                i += 1;
            }
        }
    }

    let rules = match rules_file {
        Some(path) => read_source(Some(path)),
        None => DEFAULT_RULES.to_string(),
    };

    let input_tokens = match token_file {
        Some(path) => read_source(Some(path)),
        None => read_source(None),
    };

    Parameters {
        rules,
        input_tokens,
    }
}
