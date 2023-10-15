#[cfg(feature = "debug")]
use log::trace;
use nom::multi::many1;
use nom::{branch::alt, IResult};

mod function;
mod values;

#[cfg(test)]
mod test;

use function::function_parser;
use values::{text_parser, variable_parser};

#[derive(Debug, PartialEq, Eq)]
pub enum TemplateNode {
    Variable(String),
    Function(String, Vec<TemplateNode>),
    Text(String),
}

pub fn parser(input: &str) -> IResult<&str, Vec<TemplateNode>> {
    #[cfg(feature = "debug")]
    trace!("Start Parser: {input}");
    many1(alt((function_parser, variable_parser, text_parser)))(input)
}
