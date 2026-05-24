use owo_colors::OwoColorize;

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
