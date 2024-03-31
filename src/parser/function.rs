#[cfg(feature = "debug")]
use log::debug;
use nom::bytes::complete::take_until;
use nom::character::complete::none_of;
use nom::combinator::recognize;
use nom::multi::{many1, separated_list0};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, sequence::delimited,
    IResult,
};

use super::TemplateNode;

fn internal_function_parser<'a>(
    start: &'a str,
    close: &'a str,
) -> impl FnMut(&'a str) -> IResult<&str, TemplateNode> {
    move |input| {
        let (input, _) = multispace0(input)?;
        let (input, function_name) =
            recognize(many1(none_of(format!("\",({start}{close}").as_str())))(input)?;
        let (input, _) = multispace0(input)?;
        let (input, arguments) = delimited(
            tag("("),
            separated_list0(
                tag(","),
                alt((
                    internal_function_parser(start, close),
                    raw_string,
                    argument_parser(start, close),
                )),
            ),
            tag(")"),
        )(input)?;

        #[cfg(feature = "debug")]
        debug!("FunctionName: {function_name} - Args: {arguments:?} - input: {input}");
        let (input, _) = multispace0(input)?;

        Ok((
            input,
            TemplateNode::Function(function_name.to_string(), arguments),
        ))
    }
}

pub(super) fn function_parser<'a>(
    start: &'a str,
    close: &'a str,
) -> impl FnMut(&'a str) -> IResult<&str, TemplateNode> {
    move |input| {
        let (input, _) = tag(start)(input)?;
        let (input, func) = internal_function_parser(start, close)(input)?;
        let (input, _) = tag(close)(input)?;

        Ok((input, func))
    }
}

pub(super) fn raw_string(input: &str) -> IResult<&str, TemplateNode> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("\"")(input)?;
    let (input, content) = take_until("\"")(input)?;
    let (input, _) = tag("\"")(input)?;
    let (input, _) = multispace0(input)?;

    Ok((input, TemplateNode::InnerText(content.to_string())))
}

fn argument_parser<'a>(
    start: &'a str,
    close: &'a str,
) -> impl FnMut(&'a str) -> IResult<&str, TemplateNode> {
    move |input| {
        let (input, _) = multispace0(input)?;
        let (input, arg) =
            recognize(many1(none_of(format!("\"),{start}{close}").as_str())))(input)?;
        let arg = arg.trim();
        #[cfg(feature = "debug")]
        debug!("Input Arg: {input} - ArgName: {arg}");
        Ok((input, TemplateNode::Variable(arg.to_string())))
    }
}
