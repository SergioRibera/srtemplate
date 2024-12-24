#[cfg(feature = "debug")]
use log::trace;

mod error;
mod lexer;

#[cfg(test)]
mod test;

pub use error::Error;
pub use lexer::SintaxNode;

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
) -> Result<Vec<TemplateNode<'a>>, Error> {
    #[cfg(feature = "debug")]
    trace!("Start Parser: {input} with delimiters: {start} - {close}");
    let mut parser = Parser::new(input, start, close);
    parser
        .parse()
        .map(|nodes| nodes.into_iter().map(|node| (input, node).into()).collect())
}

pub struct Parser<'a> {
    input: &'a str,
    chars: &'a [u8],
    position: usize,
    start_line: usize,
    line: usize,
    column: usize,
    open_delim: &'a str,
    close_delim: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str, open_delim: &'a str, close_delim: &'a str) -> Self {
        Self {
            input,
            chars: input.as_bytes(),
            position: 0,
            start_line: 0,
            line: 0,
            column: 0,
            open_delim,
            close_delim,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<SintaxNode>, Error> {
        let mut buffer = Vec::new();

        while !self.is_eof() {
            buffer.push(self.next_token()?);
            // if self.advance_delimiter(self.open_delim) {
            //     let var = self.parse_template_expression()?;

            //     // check end of sentence
            //     if !self.advance_delimiter(self.close_delim) {
            //         return Err(self.make_error(
            //             &format!("Expected {:?}, but found end of input", self.close_delim),
            //             self.position,
            //         ));
            //     }

            //     buffer.push(var);
            //     continue;
            // }

            // buffer.push(self.raw_text(self.position)?);
        }

        Ok(buffer)
    }

    fn advance(&mut self) {
        if self.position < self.chars.len() {
            if self.chars[self.position] == b'\n' {
                self.line += 1;
                self.column = 0;
                self.start_line = self.position;
            } else {
                self.column += 1;
            }
            self.position += 1;
        }
    }

    fn check_delimiter(&self, delim: &str) -> bool {
        self.position + delim.len() <= self.chars.len()
            && &self.chars[self.position..self.position + delim.len()] == delim.as_bytes()
    }

    fn advance_delimiter(&mut self, delim: &str) -> bool {
        if self.check_delimiter(delim) {
            if self.position + delim.len() <= self.chars.len() {
                self.position += delim.len();
                self.column += delim.len();
            }
            return true;
        }

        false
    }

    fn is_eof(&self) -> bool {
        self.position >= self.chars.len()
    }

    fn skip_whitespace(&mut self) {
        while !self.is_eof() && self.chars[self.position].is_ascii_whitespace() {
            self.advance();
        }
    }

    fn next_token(&mut self) -> Result<SintaxNode, Error> {
        let start = self.position;

        if self.advance_delimiter(self.open_delim) {
            let var = self.parse_template_expression()?;

            // check end of sentence
            if !self.advance_delimiter(self.close_delim) {
                return Err(self.make_error(
                    &format!("Expected {:?}, but found end of input", self.close_delim),
                    self.position,
                ));
            }

            return Ok(var);
        }

        self.raw_text(start)
    }

    fn string_literal(&mut self) -> Result<SintaxNode, Error> {
        self.advance();
        let start = self.position;
        let mut is_scapped = false;

        while !self.is_eof() {
            if self.chars[self.position] == b'\\' {
                is_scapped = !is_scapped;
            } else if is_scapped {
                is_scapped = false;
            }
            if self.chars[self.position] == b'"' && !is_scapped {
                self.advance();
                return Ok(SintaxNode::Str {
                    start,
                    end: self.position - 1,
                });
            }
            self.advance();
        }

        Err(self.make_error("Unterminated string literal", start))
    }

    fn identifier(&mut self) -> Result<(usize, usize), Error> {
        let start = self.position;
        while !self.is_eof() && self.chars[self.position].is_ascii_alphanumeric() {
            self.advance();
        }

        Ok((start, self.position))
    }

    fn number_literal(&mut self) -> Result<SintaxNode, Error> {
        let mut is_float = false;
        let start = self.position;

        while !self.is_eof() && self.chars[self.position].is_ascii_digit()
            || self.chars[self.position] == b'.'
        {
            if self.chars[self.position] == b'.' && is_float {
                return Err(self.make_error("The float just need one '.'", self.position));
            }
            if self.chars[self.position] == b'.' {
                is_float = true;
            }
            self.advance();
        }

        if self.chars[self.position].is_ascii_alphabetic() {
            return Err(self.make_error("Number literal not support letters", self.position));
        }

        if is_float {
            return Ok(SintaxNode::Float {
                start,
                end: self.position,
            });
        }

        Ok(SintaxNode::Number {
            start,
            end: self.position,
        })
    }

    fn raw_text(&mut self, start: usize) -> Result<SintaxNode, Error> {
        while !self.is_eof() {
            if self.check_delimiter(self.open_delim) {
                break;
            }
            self.advance();
        }

        Ok(SintaxNode::RawText {
            start,
            end: self.position,
        })
    }

    fn parse_template_expression(&mut self) -> Result<SintaxNode, Error> {
        self.skip_whitespace();
        // expect ident
        let (start, name_end) = self.identifier()?;
        self.skip_whitespace();

        if !self.is_eof() && self.chars[self.position] == b'(' {
            self.advance();
            self.skip_whitespace();

            let args = self.parse_function_arguments()?;
            self.skip_whitespace();

            if !self.advance_delimiter(")") {
                return Err(self.make_error("Unterminated function arguments", start));
            }
            self.skip_whitespace();

            Ok(SintaxNode::Function {
                name_start: start,
                name_end,
                args,
            })
        } else {
            Ok(SintaxNode::Variable {
                start,
                end: name_end,
            })
        }
    }

    fn parse_function_arguments(&mut self) -> Result<Vec<SintaxNode>, Error> {
        let mut args = Vec::new();

        while !self.is_eof() && self.chars[self.position] != b')' {
            self.skip_whitespace();

            if self.chars[self.position] == b')' {
                break;
            }

            match self.chars[self.position] {
                b'"' => {
                    args.push(self.string_literal()?);
                }
                n if n.is_ascii_digit() => args.push(self.number_literal()?),
                _ => {
                    args.push(self.parse_template_expression()?);
                }
            }

            self.skip_whitespace();
            if !self.advance_delimiter(",") {
                break;
            }
        }
        Ok(args)
    }

    fn make_error(&self, description: &str, at: usize) -> Error {
        let mut len = self.start_line + self.column;
        if len + 1 <= self.chars.len() {
            len += 1;
        }
        Error {
            description: description.to_string(),
            context: self.input[self.start_line..len]
                .to_string()
                .replace('\n', "\\n"),
            at,
            line: self.line,
            column: self.column,
        }
    }
}
