#[cfg(feature = "debug")]
use log::debug;
use nom::bytes::complete::take_until;
use nom::{bytes::complete::tag, IResult};

use crate::parser::function::raw_string;

use super::TemplateNode;

pub(super) fn variable_parser<'a>(
    start: &'a str,
    close: &'a str,
) -> impl FnMut(&'a str) -> IResult<&str, TemplateNode> {
    move |input| {
        let (input, _) = tag(start)(input)?;
        let (input, variable_name) = take_until(close)(input)?;
        let (input, _) = tag(close)(input)?;

        #[cfg(feature = "debug")]
        debug!("Input Variable: {input} - Var Name: {variable_name}");

        Ok((
            input,
            TemplateNode::Variable(variable_name.trim().to_string()),
        ))
    }
}

pub(super) fn raw_string_parser<'a>(
    start: &'a str,
    close: &'a str,
) -> impl FnMut(&'a str) -> IResult<&str, TemplateNode> {
    move |input| {
        let (input, _) = tag(start)(input)?;
        let (input, content) = raw_string(input)?;
        let (input, _) = tag(close)(input)?;

        #[cfg(feature = "debug")]
        debug!("Input Raw String: {input} - Content: {content:?}");

        Ok((input, content))
    }
}

pub(super) fn text_parser<'a>(
    start: &'a str,
) -> impl FnMut(&'a str) -> IResult<&str, TemplateNode> {
    move |input| {
        let (input, text) = take_until(start)(input)?;
        #[cfg(feature = "debug")]
        debug!("Input Text: {input} - Text: {text}");
        Ok((input, TemplateNode::Text(text.to_string())))
    }
}
