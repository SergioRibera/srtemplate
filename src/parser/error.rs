use std::fmt;

use thiserror::Error;

use crate::Error;

#[derive(Clone, Debug, PartialEq, Error)]
pub enum SyntaxErrorToken {
    #[error("end of input")]
    Eof,

    #[error("\"{0}\"")]
    Char(char),

    #[error("\"{0}\"")]
    String(String),
}

#[derive(Clone, Debug, PartialEq, Error)]
pub enum SyntaxErrorKind {
    #[error("Expected {0}, but found {1}")]
    Expected(SyntaxErrorToken, SyntaxErrorToken),

    #[error("Invalid character in Number literal")]
    InvalidNumber,

    #[error("Unterminated function arguments")]
    UnterminatedArgument,

    #[error("Unterminated string literal")]
    UnterminatedString,

    #[error("Expected one '.' in a float")]
    FloatDotted,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SyntaxError {
    pub kind: SyntaxErrorKind,
    pub at: usize,
    pub context: String,
    pub line: usize,
    pub column: usize,
    pub help: String,
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line_padding = " ".repeat(self.line.to_string().len());
        let arrow_padding = " ".repeat(self.column);

        f.write_fmt(format_args!(
            "\x1b[1;91mSyntaxError:\x1b[0m {}\n",
            self.kind
        ))?;
        f.write_fmt(format_args!(
            " \x1b[1;34m-->\x1b[0m {}:{}\n",
            self.line, self.column
        ))?;
        f.write_fmt(format_args!(
            "\x1b[1;34m {} | \x1b[0m{}\n",
            self.line, self.context
        ))?;
        f.write_fmt(format_args!(
            " {line_padding}   {arrow_padding}\x1b[1;91m^ {}\x1b[0m\n",
            self.help
        ))
    }
}

impl std::error::Error for SyntaxError {}

impl SyntaxErrorKind {
    pub fn into_error(self, input: &str, at: usize) -> Error {
        let (line, column, context) = get_line_from_offset(input, at);

        crate::Error::BadSyntax(SyntaxError {
            kind: self,
            at,
            context,
            line,
            column,
            help: String::new(),
        })
    }
}

impl SyntaxError {
    pub fn found_eof(input: &str, at: usize, expected: impl Into<String>) -> Error {
        let (line, column, context) = get_line_from_offset(input, at);

        let expected = expected.into();
        crate::Error::BadSyntax(SyntaxError {
            help: format!("help: add \"{expected}\""),
            kind: SyntaxErrorKind::Expected(
                SyntaxErrorToken::String(expected),
                SyntaxErrorToken::Eof,
            ),
            at,
            context,
            line,
            column,
        })
    }
}

fn get_line_from_offset(input: &str, mut offset: usize) -> (usize, usize, String) {
    loop {
        let last_newline = offset
            - input[..offset]
                .chars()
                .rev()
                .position(|c| c == '\n')
                .unwrap_or(offset);

        let line = input[..=last_newline.min(input.len() - 1)]
            .chars()
            .filter(|c| *c == '\n')
            .count()
            + 1;

        let Some(column) = (offset - last_newline).checked_sub(1) else {
            offset = last_newline.saturating_sub(1);
            continue;
        };

        let next_newline = input[offset..]
            .chars()
            .skip(1)
            .position(|c| c == '\n')
            .unwrap_or_else(|| input.len() - offset)
            + offset;

        let context = input[last_newline..next_newline].to_owned();

        break (line, column, context);
    }
}

// pub fn make(
//     kind: SyntaxErrorKind,
//     at: usize,
// ) -> crate::Error {
//     let mut len = start_line + column;
//     if len + 1 < chars.len() {
//         len += 1;
//     }
//     crate::Error::BadSyntax(SyntaxError {
//         context: String::from_utf8_lossy(&chars[start_line..len]).replace('\n', "\\n"),
//         kind,
//         at,
//         line,
//         column,
//     })
// }
