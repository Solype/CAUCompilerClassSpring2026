# CAUCompilerClassSpring2026
Final project of the compiler class in Chung Ang Uni in Spring 2026

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
The rule parsing 
### Step 2 - Token Scanner (BONUS)

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

## AI disclosure
Generative AI tools were used for:

- Understanding the [js-machine website](https://jsmachines.sourceforge.net/machines/slr.html).
- Understanding the construction of FIRST and FOLLOW sets.
- Understanding canonical LR(0) item collections.
- Understanding SLR parsing tables and parser actions.
- Reformulating and improving technical documentation.
- Understanding of Rust libraries
- Correction of rust lexical mistakes
