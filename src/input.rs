use std::{
    env, fs,
    io::{self, Read},
};

use crate::{helper::print_help, ruleparser::default::DEFAULT_RULES};

pub struct Parameters {
    pub rules: String,
    pub input_tokens: String,
    pub regex: Option<String>,
    pub use_regex: bool,
}

fn read_source(path: Option<&str>) -> Result<String, String> {
    let mut buffer = String::new();

    match path {
        Some(file_path) => {
            let mut file = match fs::File::open(file_path) {
                Ok(file) => file,
                Err(e) => return Err(format!("File '{}' : {:?}", file_path, e.kind())),
            };

            file.read_to_string(&mut buffer)
                .unwrap_or_else(|_| panic!("cannot read file: {file_path}"));
        }

        None => {
            io::stdin()
                .read_to_string(&mut buffer)
                .expect("cannot read stdin");
        }
    }

    Ok(buffer)
}

pub fn get_rule_and_input() -> Result<Parameters, String> {
    let args: Vec<String> = env::args().collect();

    let mut rules_file: Option<&str> = None;
    let mut token_file: Option<&str> = None;
    let mut regex_file: Option<&str> = None;
    let mut use_regex: bool = false;

    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-r" => {
                if rules_file.is_some() {
                    return Err("duplicate -r option".to_string());
                }
                if i + 1 >= args.len() {
                    return Err("-r requires a file path".to_string());
                }
                rules_file = Some(args[i + 1].as_str());
                i += 2;
            }

            "--use-regex" => {
                use_regex = true;
                i += 1
            }

            "--regex-path" => {
                if regex_file.is_some() {
                    return Err("duplicate --regex_file option".to_string());
                }
                if i + 1 >= args.len() {
                    return Err("--regex_file requires a file path".to_string());
                }
                regex_file = Some(args[i + 1].as_str());
                i += 2;
            }

            "-h" => {
                print_help();
                return Err(
                    "Sorry, we do not execute the program if you ask for what it does ^^'"
                        .to_string(),
                );
            }

            file => {
                if token_file.is_some() {
                    return Err("too many positional arguments".to_string());
                }

                token_file = Some(file);
                i += 1;
            }
        }
    }
    let rules = match rules_file {
        Some(path) => read_source(Some(path))?,
        None => DEFAULT_RULES.to_string(),
    };

    let input_tokens = match token_file {
        Some(path) => read_source(Some(path))?,
        None => read_source(None)?,
    };

    let regex = match (regex_file, use_regex) {
        (_, false) => None,
        (Some(path), true) => Some(read_source(Some(path))?),
        (None, true) => None,
    };

    Ok(Parameters {
        rules,
        input_tokens,
        regex,
        use_regex,
    })
}
