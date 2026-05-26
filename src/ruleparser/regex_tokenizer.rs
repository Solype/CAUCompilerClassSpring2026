use regex::Regex;
use once_cell::sync::Lazy;

use crate::{error::file_error, ruleparser::rules_and_tokens::TokenMetadata};

#[derive(Debug)]
pub struct RegexTokenRule {
    result: String,
    regex : Regex,
}

#[derive(Debug, Default)]
pub struct RegexTokenizer {
    rule_set: Vec<RegexTokenRule>
}


static RAW_RULE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^([A-Z_][A-Z0-9_]*)\s*:\s*(.+)$")
        .unwrap()
});

impl RegexTokenizer {

    pub fn new() -> Self
    {
        Self::default()
    }

    /**
     * It takes as input the content of the file that link a token to a regex,
     * and transform each line into a rule.
     */
    pub fn parse_file_content(&mut self, content: &String) -> Result<(), String>
    {
        for (line_number, line) in content.split("\n").enumerate() {
            if let Err(e) = self.add_raw_rule(line.to_string()) {
                return Err(file_error(
                    &"file_name".to_string(),
                    line_number,
                    1,
                    line.len(),
                    &line.to_string(),
                    &e
                ));
            }
        }
        Ok(())
    }

    /**
     * add a rule already made. because a regex can fail to be created (see library documentation),
     * we use the key word "cooked" for a regex that has been successfully created, and "raw" for a
     * potential regex, that might fail when created
     */
    pub fn add_cooked_rule(&mut self, rule: RegexTokenRule)
    {
        self.rule_set.push(rule);
    }

    /**
     * Add a rule based on a line, this function can fail if the line or regex is not well made
     * expected format :
     * TOKEN_NAME:regex
     */
    pub fn add_raw_rule(&mut self, rule: String) -> Result<(), String>
    {
        // We try to capture the different values within the line. if it matches the regex, we continue, or return an error
        let caps = RAW_RULE_REGEX.captures(&rule)
            .ok_or_else(|| {
                format!("Invalid regex rule '{}'\nExpected format: TOKEN_NAME:regex", rule)
            })?;

        let result = caps.get(1).unwrap().as_str().to_string();
        let regex_str = caps.get(2).unwrap().as_str().to_string();

        // We try to create a nuew regex based on the regex given in the line. if it does not success, we return an error
        let regex = Regex::new(&regex_str).map_err(|e| {
                format!("Invalid regex '{}' for token '{}'\n{}", regex_str, result, e )
            })?;

        self.rule_set.push(RegexTokenRule {
            result,
            regex,
        });

        Ok(())
    }

    /**
     * We check if the given RegexTokenRule is correct.
     * see the structure RegexTokenRule
     * Its inputs are the remainings to parse, the line number and the column number.
     * If it does not mathc the regular expression, it just return None, for nothing found
     * If it finds something, it return :
     * - Name of the token
     * - the metadata of the token
     * - the size of the what it analized, so next call can skip what has already been identified
     */
    fn check_rule(&self, rule: &RegexTokenRule, remaining: &str, line: usize, col: &mut usize) -> Option<(String, TokenMetadata, usize)>
    {
        if let Some(m) = rule.regex.find(remaining) {
            if m.start() != 0 {
                return None;
            }

            let text = &remaining[..m.end()];

            let metadata = TokenMetadata {
                span: (line, *col),
                str: text.to_string(),
                line: remaining.lines().next().unwrap_or("").to_string(),
            };

            for _ in text.chars() {
                *col += 1;
            }

            return Some((rule.result.clone(), metadata, text.len()));

        }
        return None;
    }

    /**
     * it transforms the content of a line 
     */
    pub fn tokenize(
        &self,
        input: &String,
    ) -> Result<Vec<(String, TokenMetadata)>, String> {
        let mut tokens: Vec<(String, TokenMetadata)> = vec![];
        
        for (line_nb, line) in input.split("\n").enumerate() {
            let mut new_tokens = self.tokenize_line(line, line_nb)?;
            tokens.append(&mut new_tokens);
        }
        Ok(tokens)
    }

    fn tokenize_line(&self, line: &str, line_number: usize) -> Result<Vec<(String, TokenMetadata)>, String> {
        let mut tokens = vec![];

        let mut cursor = 0;
        let mut col = 1;

        while cursor < line.len() {

            let remaining = &line[cursor..];

            // Ignore espaces
            if let Some(c) = remaining.chars().next() {
                if c.is_whitespace() {
                    col += 1;
                    cursor += c.len_utf8();
                    continue;
                }
            }

            let mut matched = false;

            for rule in &self.rule_set {
                if let Some((token, mut metadata, text_size)) =
                    self.check_rule(rule, remaining, line_number, &mut col) {

                        metadata.line = line.to_string();
                        tokens.push((token, metadata));
                        cursor += text_size;
                        matched = true;
                        break;
                }
            }

            if !matched {

                let current = remaining.chars().next().unwrap();

                return Err(file_error(
                    &"Token input".to_string(),
                    line_number,
                    col,
                    current.len_utf8(),
                    &remaining.lines().next().unwrap_or("").to_string(),
                    &format!(
                        "Unexpected character '{}'",
                        current
                    ),
                ));
            }
        }

        Ok(tokens)
    }





    pub fn set_default_rules(&mut self) -> &mut Self{
        // -----------------------------
        // KEYWORDS
        // -----------------------------
        self.add_cooked_rule(RegexTokenRule {
            result: "vtype".to_string(),
            regex: Regex::new(r"^(int|char|float|bool|void)\b").unwrap(),
        });

        self.add_cooked_rule(RegexTokenRule {
            result: "if".to_string(),
            regex: Regex::new(r"^if\b").unwrap(),
        });

        self.add_cooked_rule(RegexTokenRule {
            result: "else".to_string(),
            regex: Regex::new(r"^else\b").unwrap(),
        });

        self.add_cooked_rule(RegexTokenRule {
            result: "while".to_string(),
            regex: Regex::new(r"^while\b").unwrap(),
        });

        self.add_cooked_rule(RegexTokenRule {
            result: "return".to_string(),
            regex: Regex::new(r"^return\b").unwrap(),
        });

        self.add_cooked_rule(RegexTokenRule {
            result: "class".to_string(),
            regex: Regex::new(r"^class\b").unwrap(),
        });

        // -----------------------------
        // LITERALS
        // -----------------------------
        self.add_cooked_rule(RegexTokenRule {
            result: "boolstr".to_string(),
            regex: Regex::new(r"^(true|false)\b").unwrap(),
        });

        self.add_cooked_rule(RegexTokenRule {
            result: "character".to_string(),
            regex: Regex::new(r"^'([^'\\]|\\.)'").unwrap(),
        });

        self.add_cooked_rule(RegexTokenRule {
            result: "literal".to_string(),
            regex: Regex::new(r#"^"([^"\\]|\\.)*""#).unwrap(),
        });

        self.add_cooked_rule(RegexTokenRule {
            result: "num".to_string(),
            regex: Regex::new(r"^[0-9]+").unwrap(),
        });

        // -----------------------------
        // IDENTIFIER
        // -----------------------------
        self.add_cooked_rule(RegexTokenRule {
            result: "id".to_string(),
            regex: Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap(),
        });

        // -----------------------------
        // COMPARISON
        // -----------------------------
        self.add_cooked_rule(RegexTokenRule {
            result: "comp".to_string(),
            regex: Regex::new(r"^(==|!=|<=|>=|<|>)").unwrap(),
        });

        // -----------------------------
        // OPERATORS
        // -----------------------------
        self.add_cooked_rule(RegexTokenRule {
            result: "assign".to_string(),
            regex: Regex::new(r"^=").unwrap(),
        });

        self.add_cooked_rule(RegexTokenRule {
            result: "addsub".to_string(),
            regex: Regex::new(r"^(\+|-)").unwrap(),
        });

        self.add_cooked_rule(RegexTokenRule {
            result: "multidiv".to_string(),
            regex: Regex::new(r"^(\*|/)").unwrap(),
        });

        // -----------------------------
        // DELIMITERS
        // -----------------------------
        self.add_cooked_rule(RegexTokenRule {
            result: "lparen".to_string(),
            regex: Regex::new(r"^\(").unwrap(),
        });

        self.add_cooked_rule(RegexTokenRule {
            result: "rparen".to_string(),
            regex: Regex::new(r"^\)").unwrap(),
        });

        self.add_cooked_rule(RegexTokenRule {
            result: "lbrace".to_string(),
            regex: Regex::new(r"^\{").unwrap(),
        });

        self.add_cooked_rule(RegexTokenRule {
            result: "rbrace".to_string(),
            regex: Regex::new(r"^\}").unwrap(),
        });

        self.add_cooked_rule(RegexTokenRule {
            result: "semi".to_string(),
            regex: Regex::new(r"^;").unwrap(),
        });

        self.add_cooked_rule(RegexTokenRule {
            result: "comma".to_string(),
            regex: Regex::new(r"^,").unwrap(),
        });
        self
    }

}