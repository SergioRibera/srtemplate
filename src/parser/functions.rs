use crate::SrTemplateError;

use super::*;

pub fn parse_function_arguments<'a>(
    input: &'a str,
    chars: &[u8],
    position: &mut usize,
    line: &mut usize,
    column: &mut usize,
    start_line: &mut usize,
) -> Result<Vec<TemplateNode<'a>>, SrTemplateError> {
    let mut args = Vec::new();

    while !is_eof(chars, *position) && chars[*position] != b')' {
        skip_whitespace(chars, position, line, column, start_line);

        if chars[*position] == b')' {
            break;
        }

        match chars[*position] {
            b'"' => {
                args.push(string_literal(
                    input, chars, position, line, column, start_line,
                )?);
            }
            n if n.is_ascii_digit() => args.push(number_literal(
                input, chars, position, line, column, start_line,
            )?),
            _ => {
                args.push(parse_template_expression(
                    input, chars, position, line, column, start_line,
                )?);
            }
        }

        skip_whitespace(chars, position, line, column, start_line);
        if !advance_delimiter(chars, ",", column, position) {
            break;
        }
    }
    Ok(args)
}
