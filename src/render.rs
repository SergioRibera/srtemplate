use std::borrow::Cow;

use dashmap::DashMap;

use crate::error::SrTemplateError;
use crate::parser::TemplateNode;
use crate::template::TemplateFunction;
#[cfg(feature = "debug")]
use log::debug;

/// Renders a vector of `TemplateNode`s, replacing variables and processing functions.
///
/// This function processes a list of `TemplateNode`s and returns a `Result` containing the rendered template as a `String` or a [`SrTemplateError`] in case of an error.
///
/// # Arguments
///
/// * `nodes`: A vector of `TemplateNode`s to be processed.
/// * `vars`: A reference to a `DashMap` containing variable names as keys and `Cow<'_, str>` as values.
/// * `funcs`: A reference to a `DashMap` containing function names as keys and `TemplateFunction` closures as values.
///
/// # Returns
///
/// A `Result` where `Ok` contains the rendered template as a `String`, and `Err` holds a [`SrTemplateError`] if an error occurs.
pub fn render_nodes(
    nodes: Vec<TemplateNode>,
    vars: &DashMap<Cow<'_, str>, String>,
    funcs: &DashMap<Cow<'_, str>, Box<TemplateFunction>>,
) -> Result<String, SrTemplateError> {
    let mut res = String::new();

    for node in nodes {
        match node {
            TemplateNode::RawText(text) | TemplateNode::String(text) => res.push_str(&text),
            TemplateNode::Variable(variable) => {
                let variable = vars
                    .get(variable)
                    .ok_or(SrTemplateError::VariableNotFound(variable.to_owned()))?;

                res.push_str(&variable);
            }
            TemplateNode::Function(function, arguments) => {
                let evaluated_arguments: Result<Vec<String>, SrTemplateError> = arguments
                    .into_iter()
                    .map(|arg| render_nodes(vec![arg], vars, funcs))
                    .collect();

                let evaluated_arguments = evaluated_arguments?;
                #[cfg(feature = "debug")]
                debug!("Evaluated Args: {evaluated_arguments:?}");

                let result_of_function = funcs
                    .get(function)
                    .ok_or(SrTemplateError::FunctionNotImplemented(function.to_owned()))?(
                    &evaluated_arguments,
                )?;

                #[cfg(feature = "debug")]
                debug!("Result of function: {result_of_function:?}");

                res.push_str(&result_of_function);
            }
        }
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::builtin;
    use crate::parser::parser;

    use dashmap::DashMap;

    use super::*;

    #[test]
    fn basic_render() {
        let vars = DashMap::from_iter([(Cow::Borrowed("var"), "World".to_string())]);
        let template = "Hello {{ var }}";
        let nodes = parser(template, "{{", "}}").unwrap();
        let res = render_nodes(nodes, &vars, &DashMap::new());

        assert!(res.is_ok());

        let res = res.unwrap();
        assert_eq!(&res, "Hello World");
    }

    #[test]
    fn basic_function_render() {
        let vars = DashMap::from_iter([(Cow::Borrowed("var"), "WoRlD".to_string())]);
        let funcs = DashMap::from_iter([(
            Cow::Borrowed("toLowerCase"),
            Box::new(builtin::text::to_lower as TemplateFunction),
        )]);
        let template = "Hello {{ toLowerCase(var) }}";
        let nodes = parser(template, "{{", "}}").unwrap();
        let res = render_nodes(nodes, &vars, &funcs);

        assert!(res.is_ok());

        let res = res.unwrap();
        assert_eq!(res, "Hello world".to_string());
    }

    #[test]
    fn recursive_function_render() {
        let vars = DashMap::from_iter([(Cow::Borrowed("var"), "WoRlD".to_string())]);
        let funcs = DashMap::from_iter([
            (
                Cow::Borrowed("toLowerCase"),
                Box::new(builtin::text::to_lower as TemplateFunction),
            ),
            (
                Cow::Borrowed("trim"),
                Box::new(builtin::text::trim as TemplateFunction),
            ),
        ]);
        let template = "Hello {{ toLowerCase(trim(var)) }}";
        let nodes = parser(template, "{{", "}}").unwrap();
        let res = render_nodes(nodes, &vars, &funcs);

        assert!(res.is_ok());

        let res = res.unwrap();
        assert_eq!(res, "Hello world".to_string());
    }

    #[test]
    fn raw_string_render() {
        let vars = DashMap::from_iter([(Cow::Borrowed("var"), "    WoRlD".to_string())]);
        let funcs = DashMap::from_iter([
            (
                Cow::Borrowed("toLowerCase"),
                Box::new(builtin::text::to_lower as TemplateFunction),
            ),
            (
                Cow::Borrowed("trim"),
                Box::new(builtin::text::trim as TemplateFunction),
            ),
        ]);
        let template = r#"Hello
{{ toLowerCase(trim(var, "  !   ")) }}"#;
        let nodes = parser(template, "{{", "}}").unwrap();
        let res = render_nodes(nodes, &vars, &funcs);

        println!("Err: {res:?}");
        assert!(res.is_ok());

        let res = res.unwrap();
        assert_eq!(res, "Hello\nworld !".to_string());
    }
}
