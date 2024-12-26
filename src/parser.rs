#[cfg(feature = "debug")]
use log::trace;

mod error;
mod functions;
mod literals;

#[cfg(test)]
mod test;

pub use error::{make_error, Error};

use crate::SrTemplateError;
use functions::*;
use literals::*;

/// Variants of the types of nodes that exist in the syntax
#[derive(Debug, PartialEq, Eq)]
pub enum TemplateNode<'a> {
    /// Variables to be rendered
    Variable(&'a str),
    /// Functions to be rendered
    Function(&'a str, Vec<TemplateNode<'a>>),
    /// Plain text, pass as variable
    String(&'a str),
    /// Number, pass as variable
    Number(&'a str),
    /// Decimal, pass as variable
    Float(&'a str),
    /// Plain text, this will be ignored in the rendering
    RawText(&'a str),
}

/// Parse a string input into a vector of `TemplateNode`s.
///
/// This function takes a string input and parses it into a vector of `TemplateNode`s, representing different elements of the template. It uses the `nom` parser combinator library to handle the parsing.
///
/// # Returns
///
/// An `IResult` containing the remaining unparsed input (if any) and a vector of `TemplateNode`s, representing the parsed elements of the template.
pub fn parser<'a>(
    input: &'a str,
    start: &'a str,
    close: &'a str,
) -> Result<Vec<TemplateNode<'a>>, SrTemplateError> {
    #[cfg(feature = "debug")]
    trace!("Start Parser: {input} with delimiters: {start} - {close}");
    let mut res = Vec::with_capacity(20);
    let chars = input.as_bytes();
    let mut position = 0usize;
    let mut column = 0usize;
    let mut line = 0usize;
    let mut start_line = 0usize;

    while !is_eof(chars, position) {
        if advance_delimiter(chars, start, &mut column, &mut position) {
            let var = parse_template_expression(
                input,
                chars,
                &mut position,
                &mut line,
                &mut column,
                &mut start_line,
            )?;

            // check end of sentence
            if !advance_delimiter(chars, close, &mut column, &mut position) {
                return Err(make_error(
                    input,
                    chars,
                    line,
                    column,
                    start_line,
                    &format!("Expected {close:?}, but found end of input"),
                    position,
                ));
            }

            res.push(var);
            continue;
        }

        res.push(raw_text(
            input,
            chars,
            start,
            position,
            &mut position,
            &mut line,
            &mut column,
            &mut start_line,
        ));
    }

    Ok(res)
}

pub(self) fn parse_template_expression<'a>(
    input: &'a str,
    chars: &[u8],
    position: &mut usize,
    line: &mut usize,
    column: &mut usize,
    start_line: &mut usize,
) -> Result<TemplateNode<'a>, SrTemplateError> {
    skip_whitespace(chars, position, line, column, start_line);
    // expect ident
    let (start, name_end) = identifier(chars, position, line, column, start_line);
    skip_whitespace(chars, position, line, column, start_line);

    if !is_eof(chars, *position) && chars[*position] == b'(' {
        advance(chars, position, line, column, start_line);
        skip_whitespace(chars, position, line, column, start_line);

        let args = parse_function_arguments(input, chars, position, line, column, start_line)?;
        skip_whitespace(chars, position, line, column, start_line);

        if !advance_delimiter(chars, ")", column, position) {
            return Err(make_error(
                input,
                chars,
                *line,
                *column,
                *start_line,
                "Unterminated function arguments",
                start,
            ));
        }
        skip_whitespace(chars, position, line, column, start_line);

        Ok(TemplateNode::Function(&input[start..name_end], args))
    } else {
        Ok(TemplateNode::Variable(&input[start..name_end]))
    }
}

pub(self) fn identifier(
    chars: &[u8],
    position: &mut usize,
    line: &mut usize,
    column: &mut usize,
    start_line: &mut usize,
) -> (usize, usize) {
    let start = *position;
    while !is_eof(chars, *position)
        && (chars[*position].is_ascii_alphanumeric() || chars[*position] == b'_')
    {
        advance(chars, position, line, column, start_line);
    }

    (start, *position)
}

pub(self) fn raw_text<'a>(
    input: &'a str,
    chars: &[u8],
    open_delim: &str,
    start: usize,
    position: &mut usize,
    line: &mut usize,
    column: &mut usize,
    start_line: &mut usize,
) -> TemplateNode<'a> {
    while !is_eof(chars, *position) {
        if check_delimiter(chars, open_delim, *position) {
            break;
        }
        advance(chars, position, line, column, start_line);
    }

    TemplateNode::RawText(&input[start..*position])
}

pub(self) fn advance(
    chars: &[u8],
    position: &mut usize,
    line: &mut usize,
    column: &mut usize,
    start_line: &mut usize,
) {
    if *position < chars.len() {
        if chars[*position] == b'\n' {
            *line += 1;
            *column = 0;
            *start_line = *position;
        } else {
            *column += 1;
        }
        *position += 1;
    }
}

pub(self) fn check_delimiter(chars: &[u8], delim: &str, position: usize) -> bool {
    position + delim.len() <= chars.len()
        && &chars[position..position + delim.len()] == delim.as_bytes()
}

pub(self) fn advance_delimiter(
    chars: &[u8],
    delim: &str,
    column: &mut usize,
    position: &mut usize,
) -> bool {
    if check_delimiter(chars, delim, *position) {
        if *position + delim.len() <= chars.len() {
            *position += delim.len();
            *column += delim.len();
        }
        return true;
    }

    false
}

pub(self) fn is_eof(chars: &[u8], position: usize) -> bool {
    position >= chars.len()
}

pub(self) fn skip_whitespace(
    chars: &[u8],
    position: &mut usize,
    line: &mut usize,
    column: &mut usize,
    start_line: &mut usize,
) {
    while !is_eof(chars, *position) && chars[*position].is_ascii_whitespace() {
        advance(chars, position, line, column, start_line);
    }
}
