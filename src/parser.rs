#[cfg(feature = "debug")]
use log::trace;

mod error;
mod functions;
mod literals;

#[cfg(test)]
mod test;

pub use error::{SyntaxError, SyntaxErrorKind};

use functions::parse_function_arguments;

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
) -> Result<Vec<TemplateNode<'a>>, crate::Error> {
    #[cfg(feature = "debug")]
    trace!("Start Parser: {input} with delimiters: {start} - {close}");
    let mut res = Vec::with_capacity(20);
    let chars = input.as_bytes();
    let mut position = 0usize;

    while !is_eof(chars, position) {
        if advance_delimiter(chars, start, &mut position) {
            let var = parse_template_expression(input, chars, &mut position)?;

            // check end of sentence
            if !advance_delimiter(chars, close, &mut position) {
                return Err(SyntaxError::found_eof(input, position, close));
            }

            res.push(var);
            continue;
        }

        res.push(raw_text(input, chars, start, &mut position));
    }

    Ok(res)
}

fn parse_template_expression<'a>(
    input: &'a str,
    chars: &[u8],
    position: &mut usize,
) -> Result<TemplateNode<'a>, crate::Error> {
    skip_whitespace(chars, position);
    // expect ident
    let (start, name_end) = identifier(chars, position);
    skip_whitespace(chars, position);

    if !is_eof(chars, *position) && chars[*position] == b'(' {
        advance(chars, position);
        skip_whitespace(chars, position);

        let args = parse_function_arguments(input, chars, position)?;
        skip_whitespace(chars, position);

        if !advance_delimiter(chars, ")", position) {
            return Err(SyntaxErrorKind::UnterminatedArgument.into_error(input, *position));
        }
        skip_whitespace(chars, position);

        Ok(TemplateNode::Function(&input[start..name_end], args))
    } else {
        Ok(TemplateNode::Variable(&input[start..name_end]))
    }
}

fn identifier(chars: &[u8], position: &mut usize) -> (usize, usize) {
    let start = *position;
    while !is_eof(chars, *position)
        && (chars[*position].is_ascii_alphanumeric() || chars[*position] == b'_')
    {
        advance(chars, position);
    }

    (start, *position)
}

fn raw_text<'a>(
    input: &'a str,
    chars: &[u8],
    open_delim: &str,
    position: &mut usize,
) -> TemplateNode<'a> {
    let start = *position;
    while !is_eof(chars, *position) {
        if check_delimiter(chars, open_delim, *position) {
            break;
        }
        advance(chars, position);
    }

    TemplateNode::RawText(&input[start..*position])
}

fn advance(chars: &[u8], position: &mut usize) {
    if !is_eof(chars, *position) {
        *position += 1;
    }
}

fn check_delimiter(chars: &[u8], delim: &str, position: usize) -> bool {
    position + delim.len() <= chars.len()
        && &chars[position..position + delim.len()] == delim.as_bytes()
}

fn advance_delimiter(chars: &[u8], delim: &str, position: &mut usize) -> bool {
    if check_delimiter(chars, delim, *position) {
        if *position + delim.len() <= chars.len() {
            *position += delim.len();
        }
        return true;
    }

    false
}

fn is_eof(chars: &[u8], position: usize) -> bool {
    position >= chars.len()
}

fn skip_whitespace(chars: &[u8], position: &mut usize) {
    while !is_eof(chars, *position) && chars[*position].is_ascii_whitespace() {
        advance(chars, position);
    }
}
