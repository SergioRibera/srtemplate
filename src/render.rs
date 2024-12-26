use std::borrow::Cow;

use dashmap::DashMap;

use crate::error::Error;
use crate::parser::TemplateNode;
use crate::template::Function;
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
pub fn nodes(
    res: &mut String,
    tnode: TemplateNode,
    vars: &DashMap<Cow<'_, str>, String>,
    funcs: &DashMap<Cow<'_, str>, Box<Function>>,
) -> Result<(), Error> {
    match tnode {
        TemplateNode::RawText(text)
        | TemplateNode::String(text)
        | TemplateNode::Float(text)
        | TemplateNode::Number(text) => res.push_str(text),
        TemplateNode::Variable(variable) => {
            let variable = vars
                .get(variable)
                .ok_or(Error::VariableNotFound(variable.to_owned()))?;

            res.push_str(&variable);
        }
        TemplateNode::Function(function, arguments) => {
            let evaluated_arguments: Result<Vec<String>, Error> = arguments
                .into_iter()
                .map(|arg| node(arg, vars, funcs))
                .collect();

            let evaluated_arguments = evaluated_arguments?;
            #[cfg(feature = "debug")]
            debug!("Evaluated Args: {evaluated_arguments:?}");

            let result_of_function = funcs
                .get(function)
                .ok_or(Error::FunctionNotImplemented(function.to_owned()))?(
                &evaluated_arguments
            )?;

            #[cfg(feature = "debug")]
            debug!("Result of function: {result_of_function:?}");

            res.push_str(&result_of_function);
        }
    }

    Ok(())
}

pub fn node(
    tnode: TemplateNode,
    vars: &DashMap<Cow<'_, str>, String>,
    funcs: &DashMap<Cow<'_, str>, Box<Function>>,
) -> Result<String, Error> {
    match tnode {
        TemplateNode::RawText(text)
        | TemplateNode::String(text)
        | TemplateNode::Float(text)
        | TemplateNode::Number(text) => Ok(text.to_owned()),
        TemplateNode::Variable(variable) => {
            let variable = vars
                .get(variable)
                .ok_or(Error::VariableNotFound(variable.to_owned()))?;

            Ok(variable.to_owned())
        }
        TemplateNode::Function(function, arguments) => {
            let evaluated_arguments: Result<Vec<String>, Error> = arguments
                .into_iter()
                .map(|arg| node(arg, vars, funcs))
                .collect();

            let evaluated_arguments = evaluated_arguments?;
            #[cfg(feature = "debug")]
            debug!("Evaluated Args: {evaluated_arguments:?}");

            let result_of_function = funcs
                .get(function)
                .ok_or(Error::FunctionNotImplemented(function.to_owned()))?(
                &evaluated_arguments
            )?;

            #[cfg(feature = "debug")]
            debug!("Result of function: {result_of_function:?}");

            Ok(result_of_function)
        }
    }
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
        let tnodes = parser(template, "{{", "}}").unwrap();
        let mut res = String::new();

        for tnode in tnodes.into_iter() {
            let out = nodes(&mut res, tnode, &vars, &DashMap::new());
            assert!(out.is_ok());
        }

        assert_eq!(&res, "Hello World");
    }

    #[test]
    fn basic_function_render() {
        let vars = DashMap::from_iter([(Cow::Borrowed("var"), "WoRlD".to_string())]);
        let funcs = DashMap::from_iter([(
            Cow::Borrowed("toLowerCase"),
            Box::new(builtin::text::to_lower as Function),
        )]);
        let template = "Hello {{ toLowerCase(var) }}";
        let tnodes = parser(template, "{{", "}}").unwrap();
        let mut res = String::new();

        for tnode in tnodes.into_iter() {
            let out = nodes(&mut res, tnode, &vars, &funcs);
            assert!(out.is_ok());
        }

        assert_eq!(&res, "Hello world");
    }

    #[test]
    fn recursive_function_render() {
        let vars = DashMap::from_iter([(Cow::Borrowed("var"), "WoRlD".to_string())]);
        let funcs = DashMap::from_iter([
            (
                Cow::Borrowed("toLowerCase"),
                Box::new(builtin::text::to_lower as Function),
            ),
            (
                Cow::Borrowed("trim"),
                Box::new(builtin::text::trim as Function),
            ),
        ]);
        let template = "Hello {{ toLowerCase(trim(var)) }}";
        let tnodes = parser(template, "{{", "}}").unwrap();
        let mut res = String::new();

        for node in tnodes.into_iter() {
            let out = nodes(&mut res, node, &vars, &funcs);
            assert!(out.is_ok());
        }

        assert_eq!(&res, "Hello world");
    }

    #[test]
    fn raw_string_render() {
        let vars = DashMap::from_iter([(Cow::Borrowed("var"), "    WoRlD".to_string())]);
        let funcs = DashMap::from_iter([
            (
                Cow::Borrowed("toLowerCase"),
                Box::new(builtin::text::to_lower as Function),
            ),
            (
                Cow::Borrowed("trim"),
                Box::new(builtin::text::trim as Function),
            ),
        ]);
        let template = r#"Hello
{{ toLowerCase(trim(var, "  !   ")) }}"#;
        let tnodes = parser(template, "{{", "}}").unwrap();
        let mut res = String::new();

        for tnode in tnodes.into_iter() {
            let out = nodes(&mut res, tnode, &vars, &funcs);
            assert!(out.is_ok());
        }

        assert_eq!(&res, "Hello\nworld !");
    }
}
