use super::*;

#[test]
fn not_template() {
    let s = "Hello World!";
    let res = parser(s, "{{", "}}");

    assert!(res.is_ok());
    assert_eq!(res, Ok(("Hello World!", vec![])));
}

#[test]
fn valid_syntax() {
    let s = "Hello {{ variable1 }}";
    let res = parser(s, "{{", "}}");

    assert!(res.is_ok());
}

#[test]
fn invalid_syntax() {
    let s = "Hello {{ variable1 }";
    let res = parser(s, "{{", "}}");

    assert!(res.is_err());
}

#[test]
fn incomplete_syntax() {
    let s = "Hello {{ variable1";
    let res = parser(s, "{{", "}}");

    assert!(res.is_err());
}

#[test]
fn curl_in_text() {
    let s = "Hello {} } {{ variable1";
    let res = parser(s, "{{", "}}");

    assert!(res.is_err());
}

#[test]
fn invalid_syntax_simple() {
    let s = "Hello { variable1 }";
    let res = parser(s, "{{", "}}");

    assert!(res.is_ok());
}

#[test]
fn function_syntax() {
    let s = "Hello {{ toLowerCase(variable1) }}";
    let res = parser(s, "{{", "}}");

    assert!(res.is_ok());
}

#[test]
fn function_outside() {
    let s = "Hello trim(var) {{ toLowerCase(variable1) }}";
    let res = parser(s, "{{", "}}");

    assert!(res.is_ok());
}

#[test]
fn test_variable_parser() {
    let input = "{{ variable }}";
    let result = variable_parser("{{", "}}")(input);

    assert_eq!(
        result,
        Ok(("", TemplateNode::Variable("variable".to_string())))
    );
}

#[test]
fn test_variable_parser_custom_delimiters() {
    let input = "|| variable ||";
    let result = variable_parser("||", "||")(input);

    assert_eq!(
        result,
        Ok(("", TemplateNode::Variable("variable".to_string())))
    );
}

#[test]
fn test_function_parser() {
    let input = "{{ toLowerCase(trim(variable)) }}";
    let result = parser(input, "{{", "}}");
    assert_eq!(
        result,
        Ok((
            "",
            vec![TemplateNode::Function(
                "toLowerCase".to_string(),
                vec![TemplateNode::Function(
                    "trim".to_string(),
                    vec![TemplateNode::Variable("variable".to_string())]
                )]
            )]
        ))
    );
}

#[test]
fn test_function_without_args() {
    let input = "{{ toLowerCase() }}";
    let result = parser(input, "{{", "}}");
    assert_eq!(
        result,
        Ok((
            "",
            vec![TemplateNode::Function("toLowerCase".to_string(), vec![])]
        ))
    );
}

#[test]
fn test_function_multiple_param() {
    let input = "{{ toLowerCase(variable1, trim(variable), variable2) }}";
    let result = parser(input, "{{", "}}");
    assert_eq!(
        result,
        Ok((
            "",
            vec![TemplateNode::Function(
                "toLowerCase".to_string(),
                vec![
                    TemplateNode::Variable("variable1".to_string()),
                    TemplateNode::Function(
                        "trim".to_string(),
                        vec![TemplateNode::Variable("variable".to_string()),]
                    ),
                    TemplateNode::Variable("variable2".to_string()),
                ]
            )]
        ))
    );
}

#[test]
fn raw_text() {
    let s = r#"Hello {{ "ThIs Is a EXAMPLE" }}"#;
    let res = parser(s, "{{", "}}");

    assert_eq!(
        res,
        Ok((
            "",
            vec![
                TemplateNode::Text("Hello ".to_string()),
                TemplateNode::InnerText("ThIs Is a EXAMPLE".to_string())
            ]
        ))
    );
}

#[test]
fn inner_function_raw_text() {
    let s = r#"Hello {{ toLowerCase("ThIs Is a EXAMPLE") }}"#;
    let res = parser(s, "{{", "}}");

    assert_eq!(
        res,
        Ok((
            "",
            vec![
                TemplateNode::Text("Hello ".to_string()),
                TemplateNode::Function(
                    "toLowerCase".to_string(),
                    vec![TemplateNode::InnerText("ThIs Is a EXAMPLE".to_string())]
                ),
            ]
        ))
    );
}

#[test]
fn recursive_function_syntax() {
    let s = r#"Hello {{ toLowerCase(trim(split(variable1, "|"))) }}"#;
    let res = parser(s, "{{", "}}");

    assert_eq!(
        res,
        Ok((
            "",
            vec![
                TemplateNode::Text("Hello ".to_string()),
                TemplateNode::Function(
                    "toLowerCase".to_string(),
                    vec![TemplateNode::Function(
                        "trim".to_string(),
                        vec![TemplateNode::Function(
                            "split".to_string(),
                            vec![
                                TemplateNode::Variable("variable1".to_string()),
                                TemplateNode::InnerText("|".to_string())
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
    let result = text_parser("{{")(input);
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
    let result = parser(input, "{{", "}}");
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
