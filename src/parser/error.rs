use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Error {
    pub description: String,
    pub at: usize,
    pub context: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{desc}\n {line} | {ctx}\n {dots} | {arrow}^\n     at {line}:{column}",
            desc = self.description,
            line = self.line + 1,
            column = self.column + 1,
            ctx = self.context,
            dots = ".".repeat(self.line.to_string().len()),
            arrow = "-".repeat(self.column)
        ))
    }
}

impl std::error::Error for Error {}

pub fn make(
    chars: &[u8],
    line: usize,
    column: usize,
    start_line: usize,
    description: &str,
    at: usize,
) -> crate::Error {
    let mut len = start_line + column;
    if len + 1 < chars.len() {
        len += 1;
    }
    crate::Error::BadSyntax(Error {
        description: description.to_string(),
        context: String::from_utf8_lossy(&chars[start_line..len]).replace('\n', "\\n"),
        at,
        line,
        column,
    })
}
