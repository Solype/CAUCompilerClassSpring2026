use owo_colors::OwoColorize;

use crate::slr::parser::TokenWithMetadata;

pub fn token_error(
    token: TokenWithMetadata
) -> String {
    file_error(
        &"Token file".to_string(),
        token.metadata.span.1,
        token.metadata.span.0,
        token.metadata.str.len(),
        &token.metadata.str,
        &"Unexpected token".to_string()
    )
}

pub fn file_error(
    file_name: &String,
    line_number: usize,
    col_number: usize,
    size: usize,
    line: &String,
    error_message: &String
) -> String {
    let underline = format!(
        "{}{}",
        " ".repeat(col_number - 1),
        "^".repeat(size)
    );

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
