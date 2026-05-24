pub fn file_error(
    file_name: &String,
    line_number: usize,
    col_number: usize,
    size: usize,
    line: &String,
    error_message: &String
) -> String {
    let underline = format!("{}{}", " ".repeat(col_number - 1), "^".repeat(size));
    format!(
        concat!(
            "\nSynthax error\n",
            " --> {}:{}:{}\n",
            "     |\n",
            " {:>3} | {}\n",
            "     | {}\n",
            "     | {}",
        ),
        file_name,
        line_number,
        col_number,
        line_number,
        line,
        underline,
        error_message
    )
}