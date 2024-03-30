#[cfg(feature = "debug")]
use log::debug;
use nom::character::complete::none_of;
use nom::combinator::recognize;
use nom::multi::{many1, separated_list0};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, sequence::delimited,
    IResult,
};

use super::TemplateNode;

fn internal_function_parser(input: &str) -> IResult<&str, TemplateNode> {
    let (input, _) = multispace0(input)?;
    let (input, function_name) = recognize(many1(none_of(", ({{}}")))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, arguments) = delimited(
        tag("("),
        separated_list0(tag(","), alt((internal_function_parser, argument_parser))),
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

pub(super) fn function_parser(input: &str) -> IResult<&str, TemplateNode> {
    let (input, _) = tag("{{")(input)?;
    let (input, func) = internal_function_parser(input)?;
    let (input, _) = tag("}}")(input)?;

    Ok((input, func))
}

fn argument_parser(input: &str) -> IResult<&str, TemplateNode> {
    let (input, arg) = recognize(many1(none_of("),{{}}")))(input)?;
    let arg = arg.trim();
    #[cfg(feature = "debug")]
    debug!("Input Arg: {input} - ArgName: {arg}");
    Ok((input, TemplateNode::Variable(arg.to_string())))
}
