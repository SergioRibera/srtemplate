use nom::character::complete::none_of;
use nom::combinator::recognize;
use nom::multi::{many1, many0};
use nom::sequence::preceded;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace0},
    sequence::delimited,
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
    let (input, _) = multispace0(input)?;
    let (input, variable_name) = alphanumeric1(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("}}")(input)?;

    Ok((input, TemplateNode::Variable(variable_name.to_string())))
}

fn function_parser(input: &str) -> IResult<&str, TemplateNode> {
    let (input, _) = tag("{{")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, function_name) = alphanumeric1(input)?;
    let (input, _) = multispace0(input)?;
    let (input, arguments) = delimited(
        tag("("),
        many0(preceded(
            multispace0,
            alt((variable_parser, function_parser)),
        )),
        tag(")"),
    )(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("}}")(input)?;

    Ok((
        input,
        TemplateNode::Function(function_name.to_string(), arguments),
    ))
}

fn text_parser(input: &str) -> IResult<&str, TemplateNode> {
    let (input, text) = recognize(many1(none_of("{{")))(input)?;
    Ok((input, TemplateNode::Text(text.to_string())))
}

pub fn parser(input: &str) -> IResult<&str, Vec<TemplateNode>> {
    many1(alt((function_parser, variable_parser, text_parser)))(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid_syntax() {
        let s = "Hello {{ variable1 }}";
        let res = parser(s);

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

        assert!(res.is_ok());
    }

    #[test]
    /// This check the sintax with just {}, this cannot be interpretter
    fn valid_syntax_simple() {
        let s = "Hello { variable1 }";
        let res = parser(s);

        assert!(res.is_ok());
    }

    #[test]
    fn function_syntax() {
        let s = "Hello {{ toLoweCase(variable1) }}";
        let res = parser(s);

        assert!(res.is_ok());
    }

    #[test]
    fn recursive_function_syntax() {
        let s = "Hello {{ toLoweCase(trim(split(variable1, \",\"))) }}";
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
