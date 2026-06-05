use once_cell::sync::Lazy;
use regex::Regex;

use crate::{error::file_error, ruleparser::rules_and_tokens::TokenMetadata};

#[derive(Debug)]
pub struct RegexTokenRule {
    result: String,
    regex: Regex,
}

#[derive(Debug, Default)]
pub struct RegexTokenizer {
    rule_set: Vec<RegexTokenRule>,
}

// REGEX THAT PARSE OTHER REGEX
static RAW_RULE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^([A-Za-z][A-Za-z0-9_]*)\s*:\s*(.+)$").unwrap());

impl RegexTokenizer {
    pub fn new() -> Self {
        Self::default()
    }

    /**
     * It takes as input the content of the file that link a token to a regex,
     * and transform each line into a rule.
     */
    pub fn parse_file_content(&mut self, content: &String) -> Result<(), String> {
        for (line_number, line) in content.split("\n").enumerate() {
            if let Err(e) = self.add_raw_rule(line.to_string()) {
                return Err(file_error(
                    &"file_name".to_string(),
                    line_number,
                    1,
                    line.len(),
                    &line.to_string(),
                    &e,
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
    pub fn add_cooked_rule(&mut self, rule: RegexTokenRule) {
        self.rule_set.push(rule);
    }

    /**
     * Add a rule based on a line, this function can fail if the line or regex is not well made
     * expected format :
     * TOKEN_NAME:regex
     */
    pub fn add_raw_rule(&mut self, rule: String) -> Result<(), String> {
        // We try to capture the different values within the line. if it matches the regex, we continue, or return an error
        let caps = RAW_RULE_REGEX.captures(&rule).ok_or_else(|| {
            format!(
                "Invalid regex rule '{}'\nExpected format: TOKEN_NAME:regex",
                rule
            )
        })?;

        let result = caps.get(1).unwrap().as_str().to_string();
        let regex_str = caps.get(2).unwrap().as_str().to_string();

        // We try to create a new regex based on the regex given in the line. if it does not success, we return an error
        let regex = Regex::new(&regex_str).map_err(|e| {
            format!(
                "Invalid regex '{}' for token '{}'\n{}",
                regex_str, result, e
            )
        })?;

        // we push the rule inside of the rule set.
        self.rule_set.push(RegexTokenRule { result, regex });

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
    fn check_rule(
        &self,
        rule: &RegexTokenRule,
        remaining: &str,
        line: usize,
        col: &mut usize,
    ) -> Option<(String, TokenMetadata, usize)> {
        // check if the regex has found someting whithin the line.
        if let Some(m) = rule.regex.find(remaining) {
            // if what has been found by the regex is not at the very beginning, we consider it did not find anything
            if m.start() != 0 {
                return None;
            }

            // we extract what has been identified by the regex into a variable
            let text = &remaining[..m.end()];
            let text_str = text.to_string();
            let col_nb = *col;

            *col += text_str.len();

            // we can create a metadata object thanks to all those information.
            let metadata = TokenMetadata {
                span: (line, col_nb),
                str: text_str,
                line: remaining.lines().next().unwrap_or("").to_string(),
            };

            return Some((rule.result.clone(), metadata, text.len()));
        }
        return None;
    }

    /**
     * it transforms the content of a file into multiple futur token as string with their metadata (line, line nb, col nb)
     */
    pub fn tokenize(&self, input: &String) -> Result<Vec<(String, TokenMetadata)>, String> {
        let mut tokens: Vec<(String, TokenMetadata)> = vec![];

        for (line_nb, line) in input.split("\n").enumerate() {
            let mut new_tokens = self.tokenize_line(line, line_nb)?;
            tokens.append(&mut new_tokens);
        }
        Ok(tokens)
    }

    /**
     * Tokenizes a single source line using the current regex rule set.
     *
     * The lexer scans the line from left to right and:
     *   - skips whitespace
     *   - tries every token rule in declaration order
     *   - consumes the first matching rule
     *   - generates token metadata (line number, column number, full line)
     *
     * If no rule matches the current character,
     * a formatted lexer error is returned.
     *
     * Parameters:
     *   - `line`        : source line to tokenize
     *   - `line_number` : 1-based line index in the source file
     *
     * Returns:
     *   - `Ok(Vec<(String, TokenMetadata)>)`
     *       list of recognized tokens
     *
     *   - `Err(String)`
     *       formatted lexer diagnostic
     */
    fn tokenize_line(
        &self,
        line: &str,
        line_number: usize,
    ) -> Result<Vec<(String, TokenMetadata)>, String> {
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

            // We check among all the rules if one match, if that's the cas, we put mathed to true and add the token into the list of tokens
            for rule in &self.rule_set {
                if let Some((token, mut metadata, text_size)) =
                    self.check_rule(rule, remaining, line_number, &mut col)
                {
                    metadata.line = line.to_string();
                    tokens.push((token, metadata));
                    cursor += text_size;
                    matched = true;
                    break;
                }
            }

            // if nothing matched, we throw an error
            if !matched {
                let current = remaining.chars().next().unwrap();

                return Err(file_error(
                    &"Token input".to_string(),
                    line_number,
                    col,
                    current.len_utf8(),
                    &remaining.lines().next().unwrap_or("").to_string(),
                    &format!("Unexpected character '{}'", current),
                ));
            }
        }

        Ok(tokens)
    }

    pub fn set_default_rules(&mut self) -> &mut Self {
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
