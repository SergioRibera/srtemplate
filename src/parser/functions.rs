use crate::Error;

use super::literals::{number_literal, string_literal};
use super::{advance_delimiter, is_eof, parse_template_expression, skip_whitespace, TemplateNode};

pub fn parse_function_arguments<'a>(
    input: &'a str,
    chars: &[u8],
    position: &mut usize,
) -> Result<Vec<TemplateNode<'a>>, Error> {
    let mut args = Vec::new();

    while !is_eof(chars, *position) && chars[*position] != b')' {
        skip_whitespace(chars, position);

        if is_eof(chars, *position) || chars[*position] == b')' {
            break;
        }

        match chars[*position] {
            b'"' => {
                args.push(string_literal(input, chars, position)?);
            }
            n if n.is_ascii_digit() => args.push(number_literal(input, chars, position)?),
            _ => {
                args.push(parse_template_expression(input, chars, position)?);
            }
        }

        skip_whitespace(chars, position);
        if !advance_delimiter(chars, ",", position) {
            break;
        }
    }
    Ok(args)
}
