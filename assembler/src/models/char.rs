#[allow(clippy::module_name_repetitions)]
#[must_use]
pub fn escape_char(char: char) -> char {
    match char {
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        '\\' => '\\',
        _ => panic!("Invalid escape code"),
    }
}
