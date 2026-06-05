# CAUCompilerClassSpring2026
Final project of the compiler class in Chung Ang Uni in Spring 2026

## Usage

USAGE:
    syntax_analyzer [OPTIONS] <token_file>

POSITIONAL ARGUMENTS:
    <token_file>
        Source file to tokenize and parse.

OPTIONS:
- `-r <file>`            Path to the CFG grammar file.
- `--use-regex`          Enable regex-based tokenizer. If use alone, it uses the built-in regexes
- `--regex-path <file>`  Path to regex token definitions. It only matters if the regex-based tokenizer is enabled
- `-h`, `--help`           Show the usage message.

DESCRIPTION:
- loads a CFG grammar
- tokenizes the input source
- generates SLR parsing tables
- parses the token stream
- builds a parse tree

GRAMMAR FORMAT:

`CODE -> VDECL CODE`

`CODE -> ''`

REGEX TOKEN FORMAT:

`TOKEN_NAME:regex`

EXAMPLES:
```
syntax_analyzer -r grammar.txt source.code
```
```
syntax_analyzer -r grammar.txt \
                --use-regex \
                --regex-path lexer.regex \
                source.code
```
AUTHOR:
    Made with shift/reduce conflicts and emotional damage.

## Compilation
### Requirements

- Rust 1.75+
- Cargo

Check your installation:

```bash
rustc --version
cargo --version
```

### Build

To compile the project in release mode:

```bash
cargo build --release
```

The optimized executable will be generated in:

```txt
target/release/syntax_analyzer
```

Example:

```bash
./target/release/syntax_analyzer -h
```

## CFG explanation
Our non-ambiguous CFG can be found [here](CFG.cfg).
We made 2 changes:
- `EXPR`:\
Original grammar:\
    `EXPR -> EXPR addsub EXPR | EXPR multidiv EXPR`\
    This grammar is ambiguous because an expression such as
    `num addsub num multidiv num` admits multiple parse trees.\
    To enforce standard arithmetic precedence, the grammar was rewritten using HIGHOP, LOWOP and OPERAND nonterminals. Multiplication and division are reduced before addition and subtraction, eliminating the ambiguity.
- `COND`:\
Original grammar:\
    `COND -> COND comp COND`\
    This grammar is ambiguous because an expression such as
    `boolstr comp boolstr comp boolstr` admits multiple parse trees.\
    We chose left associativity: `(boolstr comp boolstr) comp boolstr`\
    This was achieved by introducing RCOND and using left recursion:\
    `COND -> COND comp RCOND` and `COND -> RCOND`

## Parsing Pipeline
### Step 1 - Rule Parsing (BONUS)

The rule parser reads a CFG grammar description and converts it into an internal representation usable by the SLR parser generator.

Features:
- Support for epsilon productions (`''`)
- Automatic terminal/non-terminal discovery
- Production validation
- Human-friendly syntax diagnostics with line and column information
- Colored error messages
- Embedded default grammar support

Example:

```txt
CODE -> VDECL CODE
CODE -> ''
VDECL -> vtype id semi
```

Those rules must be in the format `<Token> -> <Token list>`
with space before and after `->`


---

### Step 2 - Token Scanner (BONUS)

Two tokenization modes are available:

#### Manual Token Scanner

Reads a whitespace-separated token stream and attaches metadata to every token:
- line number
- column number
- original lexeme

Example:

```txt
vtype id lparen rparen
```
Example:

Features:
- Custom regex rule files
- Built-in default lexer
- Longest-prefix matching
- Token metadata generation
- Precise lexer diagnostics with source highlighting

Example rule:

```txt
id:[a-zA-Z_][a-zA-Z0-9_]*
num:[0-9]+
```
Those rules must be in the format :`<Token>:<Regex>` with no space in between

This allow to parse and transform file like :

```c
int hello() {
    return 42;
}
```

Into a token list like :

```txt
vtype id lparen rparen lbrace return num semi rbrace
```


### Step 3 - SLR Table build (BONUS ?)
The canonical LR(0) collection is represented as a DFA.
Each state contains a set of LR(0) items and transitions
correspond to GOTO operations on grammar symbols.\
Build Steps:
1. Compute FIRST and FOLLOW sets.
2. Build canonical LR(0) item collection.
3. Generate SHIFT actions from DFA transitions.
4. Generate GOTO entries from DFA transitions.
5. Generate REDUCE actions using FOLLOW sets.
6. Generate ACCEPT action for the augmented

Any shift/reduce or reduce/reduce conflict causes construction to fail.\
The built SLR Table with our unambiguous CFG can be seen [here](SLR_Table.md)

### Step 4 - Parser run
A parser is created with the SLRTable previously created, and the productions (aka rules).\
The stack is set to empty.

Parsing steps:
1. Initialize the stack with state 0.
2. Repeatedly consult the ACTION table.
3. Execute Shift, Reduce or Accept.\
    During SHIFT operations, leaf nodes are created for terminal symbols.\
    During REDUCE operations, a new nonterminal node is created and the reduced symbols become its children.\
    After ACCEPT, the remaining node becomes the root of the parse tree.
4. After each reduction, consult the GOTO table.
5. Continue until the input is accepted.

On success, the root of the parse tree is returned.\
On failure, a syntax error describing the unexpected token is returned.

## Testing

Every tests can be found in the folder `tests`.

### Parsing with default cfg
go to `tests/parsing_base_cfg`. You can execute the script `./test_parsing.sh [binary_path]` (you need chmod 755).
### Custom cfg file parsing errors
go to `tests/cfg_errors`. You can execute the script `./test_cfg.sh [binary_path]` (you need chmod 755).
### Parsing with custom cfg
go to `tests/other_cfg`. You can try an ambiguous cfg, and the base cfg from js-machine website. Inseide `js_machine` folder:
```bash
../../../syntax_analyzer -r js_machine_cfg.cfg test_input
```
and
```bash
../../../syntax_analyzer -r js_machine_cfg.cfg test_input_regex.txt --use-regex --regex-path lexer.regex
```
### Regex example
go to `tests/regex`. You can try regex:\
 `../../syntax_analyzer test_fdecl_while.c --use-regex`
### Regex error
go to `tests/regex_error`. The files in this folder should raise an error.

## AI disclosure
Generative AI tools were used for:

- Understanding the [js-machine website](https://jsmachines.sourceforge.net/machines/slr.html).
- Understanding the construction of FIRST and FOLLOW sets.
- Understanding canonical LR(0) item collections.
- Understanding SLR parsing tables and parser actions.
- Reformulating and improving technical documentation.
- Understanding of Rust libraries
- Correction of rust lexical mistakes
- Helping write tests scripts
