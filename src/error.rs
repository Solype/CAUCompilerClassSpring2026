use owo_colors::OwoColorize;

use crate::slr::parser::TokenWithMetadata;

pub fn token_error(token: TokenWithMetadata, filename: Option<String>) -> String {
    file_error(
        &filename.unwrap_or("STDIN".to_string()),
        token.metadata.span.0,
        token.metadata.span.1,
        token.metadata.str.len(),
        &token.metadata.line,
        &format!("Unexpected token '{}'", token.metadata.str),
    )
}

pub fn file_error(
    file_name: &String,
    line_number: usize,
    col_number: usize,
    size: usize,
    line: &String,
    error_message: &String,
) -> String {
    let underline_col = if col_number == 0 { 0 } else { col_number - 1 };
    let underline = format!("{}{}", " ".repeat(underline_col), "^".repeat(size));
    format!(
        concat!(
            "{}\n",
            " {} {}:{}:{}\n",
            "     |\n",
            " {:>3} | {}\n",
            "     | {}\n",
            "     | {}",
        ),
        "Syntax error".red().bold(),
        "-->".blue().bold(),
        file_name.cyan(),
        line_number,
        col_number,
        line_number.to_string().blue(),
        line,
        underline.red().bold(),
        error_message.red()
    )
}
