use owo_colors::OwoColorize;

pub fn print_help() {
    println!(
        r#"
{title}

{usage}
    compiler [OPTIONS] <token_file>

{positional}
    <token_file>
        Source file to tokenize and parse.

{options}
    {r_opt:<20} Path to the CFG grammar file.
    {regex_opt:<20} Enable regex-based tokenizer. If use alone, it uses the built-in regexes
    {regex_path:<20} Path to regex token definitions. It only matters if the regex-based tokenizer is enabled
    {help_opt:<20} Show this help message.

{description}
    • loads a CFG grammar
    • tokenizes the input source
    • generates SLR parsing tables
    • parses the token stream
    • builds a parse tree

{grammar}
    CODE -> VDECL CODE
    CODE -> ''

{regex}
    TOKEN_NAME:regex

{examples}
    compiler -r grammar.txt source.code

    compiler -r grammar.txt \
             --use-regex \
             --regex-path lexer.regex \
             source.code

{author}
    Made with shift/reduce conflicts and emotional damage.
"#,
        title       = "Mini Compiler / SLR Parser".bright_green().bold(),

        usage       = "USAGE:".bright_blue().bold(),

        positional  = "POSITIONAL ARGUMENTS:".bright_blue().bold(),

        options     = "OPTIONS:".bright_blue().bold(),

        description = "DESCRIPTION:".bright_blue().bold(),

        grammar     = "GRAMMAR FORMAT:".bright_blue().bold(),

        regex       = "REGEX TOKEN FORMAT:".bright_blue().bold(),

        examples    = "EXAMPLES:".bright_blue().bold(),

        author      = "AUTHOR:".bright_blue().bold(),

        r_opt       = "-r <file>".yellow(),

        regex_opt   = "--use-regex".yellow(),

        regex_path  = "--regex-path <file>".yellow(),

        help_opt    = "-h, --help".yellow(),
    );
}
