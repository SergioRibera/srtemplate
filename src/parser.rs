use nom::bytes::complete::take_until;
use nom::character::complete::none_of;
use nom::combinator::recognize;
use nom::multi::{many0, many1};
use nom::sequence::preceded;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, sequence::delimited,
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
pub enum TemplateNode {
    Variable(String),
    Function(String, Vec<TemplateNode>),
    Text(String),
}

fn variable_parser(input: &str) -> IResult<&str, TemplateNode> {
    let (input, _) = tag("{{")(input)?;
    let (input, variable_name) = take_until("}}")(input)?;
    let (input, _) = tag("}}")(input)?;

    println!("Input Variable: {input} - Var Name: {variable_name}");

    Ok((
        input,
        TemplateNode::Variable(variable_name.trim().to_string()),
    ))
}

fn internal_function_parser(input: &str) -> IResult<&str, TemplateNode> {
    let (input, _) = multispace0(input)?;
    let (input, function_name) = recognize(many1(none_of("({{}}")))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, arguments) = delimited(
        tag("("),
        many0(preceded(
            multispace0,
            alt((internal_function_parser, argument_parser)),
        )),
        tag(")"),
    )(input)?;

    println!("FunctionName: {function_name} - Args: {arguments:?} - input: {input}");
    let (input, _) = multispace0(input)?;

    Ok((
        input,
        TemplateNode::Function(function_name.to_string(), arguments),
    ))
}

fn function_parser(input: &str) -> IResult<&str, TemplateNode> {
    let (input, _) = tag("{{")(input)?;
    let (input, func) = internal_function_parser(input)?;
    let (input, _) = tag("}}")(input)?;

    Ok((input, func))
}

fn argument_parser(input: &str) -> IResult<&str, TemplateNode> {
    let (input, arg) = recognize(many1(none_of("),{{}}")))(input)?;
    println!("Input Arg: {input} - ArgName: {arg}");
    Ok((input, TemplateNode::Variable(arg.to_string())))
}

fn text_parser(input: &str) -> IResult<&str, TemplateNode> {
    let (input, text) = take_until("{{")(input)?;
    println!("Input Text: {input} - Text: {text}");
    Ok((input, TemplateNode::Text(text.to_string())))
}

pub fn parser(input: &str) -> IResult<&str, Vec<TemplateNode>> {
    println!("Start Parser: {input}");
    many1(alt((function_parser, variable_parser, text_parser)))(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid_syntax() {
        let s = "Hello {{ variable1 }}";
        let res = parser(s);

        println!("{res:?}");

        assert!(res.is_ok());
    }

    #[test]
    fn invalid_syntax() {
        let s = "Hello {{ variable1 }";
        let res = parser(s);

        assert!(res.is_err());
    }

    #[test]
    fn incomplete_syntax() {
        let s = "Hello {{ variable1";
        let res = parser(s);

        assert!(res.is_err());
    }

    #[test]
    fn curl_in_text() {
        let s = "Hello {} } {{ variable1";
        let res = parser(s);

        assert!(res.is_err());
    }

    #[test]
    fn invalid_syntax_simple() {
        let s = "Hello { variable1 }";
        let res = parser(s);

        assert!(res.is_err());
    }

    #[test]
    fn function_syntax() {
        let s = "Hello {{ toLowerCase(variable1) }}";
        let res = parser(s);

        assert!(res.is_ok());
    }

    #[test]
    fn test_variable_parser() {
        let input = "{{ variable }}";
        let result = variable_parser(input);

        assert_eq!(
            result,
            Ok(("", TemplateNode::Variable("variable".to_string())))
        );
    }

    #[test]
    fn test_function_parser() {
        let input = "{{ toLowerCase(trim(variable)) }}";
        let result = function_parser(input);
        assert_eq!(
            result,
            Ok((
                "",
                TemplateNode::Function(
                    "toLowerCase".to_string(),
                    vec![TemplateNode::Function(
                        "trim".to_string(),
                        vec![TemplateNode::Variable("variable".to_string())]
                    )]
                )
            ))
        );
    }

    #[test]
    fn test_function_multiple_param() {
        let input = "{{ toLowerCase(trim(variable), variable1, variable2) }}";
        let result = function_parser(input);
        assert_eq!(
            result,
            Ok((
                "",
                TemplateNode::Function(
                    "toLowerCase".to_string(),
                    vec![
                        TemplateNode::Function(
                            "trim".to_string(),
                            vec![TemplateNode::Variable("variable".to_string()),]
                        ),
                        TemplateNode::Variable("variable1".to_string()),
                        TemplateNode::Variable("variable2".to_string()),
                    ]
                )
            ))
        );
    }

    #[test]
    fn recursive_function_syntax() {
        let s = "Hello {{ toLowerCase(trim(split(variable1, \",\"))) }}";
        let res = parser(s);

        assert_eq!(
            res,
            Ok((
                "",
                vec![
                    TemplateNode::Text("Hello".to_string()),
                    TemplateNode::Function(
                        "toLowerCase".to_string(),
                        vec![TemplateNode::Function(
                            "trim".to_string(),
                            vec![TemplateNode::Function(
                                "split".to_string(),
                                vec![
                                    TemplateNode::Variable("variable1".to_string()),
                                    TemplateNode::Variable(",".to_string())
                                ]
                            )]
                        )]
                    )
                ]
            ))
        );
    }

    #[test]
    fn test_text_parser() {
        let input = "This is some text. {{ variable }} and {{ toLowerCase(trim(variable)) }}";
        let result = text_parser(input);
        assert_eq!(
            result,
            Ok((
                "{{ variable }} and {{ toLowerCase(trim(variable)) }}",
                TemplateNode::Text("This is some text. ".to_string())
            ))
        );
    }

    #[test]
    fn test_parser() {
        let input = "This is some text. {{ variable }} and {{ toLowerCase(trim(variable)) }}";
        let result = parser(input);
        assert_eq!(
            result,
            Ok((
                "",
                vec![
                    TemplateNode::Text("This is some text. ".to_string()),
                    TemplateNode::Variable("variable".to_string()),
                    TemplateNode::Text(" and ".to_string()),
                    TemplateNode::Function(
                        "toLowerCase".to_string(),
                        vec![TemplateNode::Function(
                            "trim".to_string(),
                            vec![TemplateNode::Variable("variable".to_string())]
                        )]
                    )
                ]
            ))
        );
    }
}
