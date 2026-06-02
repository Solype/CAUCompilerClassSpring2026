/**
 * This code allows us to directly embed the content of the `SLR.cfg`
 * grammar file into the compiled binary at compile time.
 *
 * If the file does not exist during compilation,
 * Rust will emit a compile-time error.
 */
pub const DEFAULT_RULES: &str = include_str!("../../CFG.cfg");
