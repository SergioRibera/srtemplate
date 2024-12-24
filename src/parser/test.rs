use super::*;

#[test]
fn not_template() {
    let s = "Hello World!";
    let res = parser(s, "{{", "}}");

    assert!(res.is_ok());
    assert_eq!(res, Ok(vec![TemplateNode::RawText("Hello World!")]));
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

    assert_eq!(
        res,
        Ok(vec![
            TemplateNode::RawText("Hello trim(var) "),
            TemplateNode::Function("toLowerCase", vec![TemplateNode::Variable("variable1")])
        ])
    );
}

#[test]
fn test_function_parser() {
    let input = "{{ toLowerCase(trim(variable)) }}";
    let result = parser(input, "{{", "}}");
    assert_eq!(
        result,
        Ok(vec![TemplateNode::Function(
            "toLowerCase",
            vec![TemplateNode::Function(
                "trim",
                vec![TemplateNode::Variable("variable")]
            )]
        )])
    );
}

#[test]
fn test_function_without_args() {
    let input = "{{ toLowerCase() }}";
    let result = parser(input, "{{", "}}");
    assert_eq!(
        result,
        Ok(vec![TemplateNode::Function("toLowerCase", vec![])])
    );
}

#[test]
fn test_function_multiple_param() {
    let input = "{{ toLowerCase(variable1, trim(variable), variable2) }}";
    let result = parser(input, "{{", "}}");
    assert_eq!(
        result,
        Ok(vec![TemplateNode::Function(
            "toLowerCase",
            vec![
                TemplateNode::Variable("variable1"),
                TemplateNode::Function("trim", vec![TemplateNode::Variable("variable"),]),
                TemplateNode::Variable("variable2"),
            ]
        )])
    );
}

#[test]
fn raw_text() {
    let s = r#"Hello {{ "ThIs Is a EXAMPLE" }}"#;
    let res = parser(s, "{{", "}}");

    assert!(res.is_err());
}

#[test]
fn inner_function_raw_text() {
    let s = r#"Hello {{ toLowerCase("ThIs Is a EXAMPLE") }}"#;
    let res = parser(s, "{{", "}}");

    assert_eq!(
        res,
        Ok(vec![
            TemplateNode::RawText("Hello "),
            TemplateNode::Function(
                "toLowerCase",
                vec![TemplateNode::String("ThIs Is a EXAMPLE")]
            ),
        ])
    );
}

#[test]
fn numbers() {
    let s = r#"Hello {{ test(14, 0.25, 00000, 00000.0) }}"#;
    let res = parser(s, "{{", "}}");

    assert_eq!(
        res,
        Ok(vec![
            TemplateNode::RawText("Hello "),
            TemplateNode::Function(
                "test",
                vec![
                    TemplateNode::Number("14"),
                    TemplateNode::Float("0.25"),
                    TemplateNode::Number("00000"),
                    TemplateNode::Float("00000.0"),
                ]
            ),
        ])
    );
}

#[test]
fn invalid_numbers() {
    let s = r#"Hello {{ test(14.0.8) }}"#;
    let res = parser(s, "{{", "}}");

    assert!(res.is_err());

    let s = r#"Hello {{ test(14test) }}"#;
    let res = parser(s, "{{", "}}");

    assert!(res.is_err());
}

#[test]
fn recursive_function_syntax() {
    let s = r#"Hello {{ toLowerCase(trim(split(variable1, "|"))) }}"#;
    let res = parser(s, "{{", "}}");

    assert_eq!(
        res,
        Ok(vec![
            TemplateNode::RawText("Hello "),
            TemplateNode::Function(
                "toLowerCase",
                vec![TemplateNode::Function(
                    "trim",
                    vec![TemplateNode::Function(
                        "split",
                        vec![
                            TemplateNode::Variable("variable1"),
                            TemplateNode::String("|")
                        ]
                    )]
                )]
            )
        ])
    );
}

#[test]
fn test_parser() {
    let input = "This is some text. {{ variable }} and {{ toLowerCase(trim(variable)) }}";
    let result = parser(input, "{{", "}}");
    assert_eq!(
        result,
        Ok(vec![
            TemplateNode::RawText("This is some text. "),
            TemplateNode::Variable("variable"),
            TemplateNode::RawText(" and "),
            TemplateNode::Function(
                "toLowerCase",
                vec![TemplateNode::Function(
                    "trim",
                    vec![TemplateNode::Variable("variable")]
                )]
            )
        ])
    );
}
