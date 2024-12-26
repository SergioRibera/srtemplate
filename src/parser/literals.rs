use crate::SrTemplateError;

use super::{advance, is_eof, make_error, TemplateNode};

pub fn string_literal<'a>(
    input: &'a str,
    chars: &[u8],
    position: &mut usize,
    line: &mut usize,
    column: &mut usize,
    start_line: &mut usize,
) -> Result<TemplateNode<'a>, SrTemplateError> {
    advance(chars, position, line, column, start_line);
    let start = *position;
    let mut is_scapped = false;

    while !is_eof(chars, *position) {
        let token = chars[*position];
        if is_scapped {
            is_scapped = false;
        } else if token == b'\\' {
            is_scapped = true;
        } else if token == b'"' {
            advance(chars, position, line, column, start_line);
            return Ok(TemplateNode::String(&input[start..*position - 1]));
        }
        advance(chars, position, line, column, start_line);
    }

    Err(make_error(
        input,
        chars,
        *line,
        *column,
        *start_line,
        "Unterminated string literal",
        start,
    ))
}

pub fn number_literal<'a>(
    input: &'a str,
    chars: &[u8],
    position: &mut usize,
    line: &mut usize,
    column: &mut usize,
    start_line: &mut usize,
) -> Result<TemplateNode<'a>, SrTemplateError> {
    let mut is_float = false;
    let start = *position;

    while !is_eof(chars, *position)
        && (chars[*position].is_ascii_digit() || chars[*position] == b'.')
    {
        if chars[*position] == b'.' && is_float {
            return Err(make_error(
                input,
                chars,
                *line,
                *column,
                *start_line,
                "The float just need one '.'",
                *position,
            ));
        }
        if chars[*position] == b'.' {
            is_float = true;
        }
        advance(chars, position, line, column, start_line);
    }

    if let Some(&token) = chars.get(*position) {
        if !(token.is_ascii_digit() || token == b'.' || token == b',' || token == b')') {
            return Err(make_error(
                input,
                chars,
                *line,
                *column,
                *start_line,
                "Invalid character in Number literal",
                *position,
            ));
        }
    }

    if is_float {
        return Ok(TemplateNode::Float(&input[start..*position]));
    }

    Ok(TemplateNode::Number(&input[start..*position]))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup<'a>(input: &'a str) -> (&'a str, Vec<u8>, usize, usize, usize, usize) {
        let chars = input.as_bytes();
        println!("Input: {input}");
        (input, chars.to_vec(), 0, 1, 0, 0)
    }

    #[test]
    fn test_string_literal_success() {
        let (input, chars, mut position, mut line, mut column, mut start_line) =
            setup("\"hello world\"");
        let result = string_literal(
            input,
            &chars,
            &mut position,
            &mut line,
            &mut column,
            &mut start_line,
        );

        assert!(result.is_ok());
        let node = result.unwrap();
        if let TemplateNode::String(value) = node {
            assert_eq!(value, "hello world");
        } else {
            panic!("Expected a String node");
        }
        assert_eq!(position, 13);
    }

    #[test]
    fn test_string_literal_escaped_quote() {
        let (input, chars, mut position, mut line, mut column, mut start_line) =
            setup(r#""hello \"world\"""#);
        let result = string_literal(
            input,
            &chars,
            &mut position,
            &mut line,
            &mut column,
            &mut start_line,
        );

        assert!(result.is_ok());
        let node = result.unwrap();
        if let TemplateNode::String(value) = node {
            assert_eq!(value, r#"hello \"world\""#);
        } else {
            panic!("Expected a String node");
        }
        assert_eq!(position, 17);
    }

    #[test]
    fn test_string_literal_unterminated() {
        let (input, chars, mut position, mut line, mut column, mut start_line) =
            setup("\"hello world");
        let result = string_literal(
            input,
            &chars,
            &mut position,
            &mut line,
            &mut column,
            &mut start_line,
        );

        assert!(result.is_err());

        if let SrTemplateError::BadSyntax(error) = result.unwrap_err() {
            assert_eq!(&error.description, "Unterminated string literal");
        }
        assert_eq!(position, 12);
    }

    #[test]
    fn test_number_literal_integer() {
        let (input, chars, mut position, mut line, mut column, mut start_line) = setup("12345");
        let result = number_literal(
            input,
            &chars,
            &mut position,
            &mut line,
            &mut column,
            &mut start_line,
        );

        assert!(result.is_ok());
        let node = result.unwrap();
        if let TemplateNode::Number(value) = node {
            assert_eq!(value, "12345");
        } else {
            panic!("Expected a Number node");
        }
        assert_eq!(position, 5);
    }

    #[test]
    fn test_number_literal_float() {
        let (input, chars, mut position, mut line, mut column, mut start_line) = setup("123.45");
        let result = number_literal(
            input,
            &chars,
            &mut position,
            &mut line,
            &mut column,
            &mut start_line,
        );

        assert!(result.is_ok());
        let node = result.unwrap();
        if let TemplateNode::Float(value) = node {
            assert_eq!(value, "123.45");
        } else {
            panic!("Expected a Float node");
        }
        assert_eq!(position, 6);
    }

    #[test]
    fn test_number_literal_multiple_dots() {
        let (input, chars, mut position, mut line, mut column, mut start_line) = setup("123.45.67");
        let result = number_literal(
            input,
            &chars,
            &mut position,
            &mut line,
            &mut column,
            &mut start_line,
        );

        assert!(result.is_err());
        if let SrTemplateError::BadSyntax(error) = result.unwrap_err() {
            assert_eq!(&error.description, "The float just need one '.'");
        }
        assert_eq!(position, 6);
    }

    #[test]
    fn test_number_literal_invalid_character() {
        let (input, chars, mut position, mut line, mut column, mut start_line) = setup("123a45");
        let result = number_literal(
            input,
            &chars,
            &mut position,
            &mut line,
            &mut column,
            &mut start_line,
        );

        assert!(result.is_err());
        if let SrTemplateError::BadSyntax(error) = result.unwrap_err() {
            assert_eq!(&error.description, "Invalid character in Number literal");
        }
        assert_eq!(position, 3);
    }
}
