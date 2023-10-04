use dashmap::DashMap;

use crate::error::SrTemplateError;
use crate::parser::TemplateNode;
use crate::template::TemplateFunction;
#[cfg(feature = "debug")]
use log::debug;

pub fn render_nodes(
    nodes: Vec<TemplateNode>,
    vars: &DashMap<&str, String>,
    funcs: &DashMap<&str, Box<TemplateFunction>>,
) -> Result<String, SrTemplateError> {
    let mut res = String::new();

    for node in nodes {
        match node {
            TemplateNode::Text(text) => res.push_str(&text),
            TemplateNode::Variable(variable) => {
                let variable = vars
                    .get(variable.as_str())
                    .ok_or(SrTemplateError::VariableNotFound(variable))?;

                res.push_str(&variable);
            }
            TemplateNode::Function(function, arguments) => {
                let evaluated_arguments: Result<Vec<String>, SrTemplateError> = arguments
                    .into_iter()
                    .map(|arg| render_nodes(vec![arg], vars, funcs))
                    .collect();
                let func = funcs
                    .get(function.as_str())
                    .ok_or(SrTemplateError::FunctionNotImplemented(function))?;

                let evaluated_arguments = evaluated_arguments?;
                #[cfg(feature = "debug")]
                debug!("Evaluated Args: {evaluated_arguments:?}");

                let result_of_function = func(evaluated_arguments);
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
        let vars = DashMap::from_iter([("var", "World".to_string())]);
        let template = "Hello {{ var }}";
        let (_, nodes) = parser(template).unwrap();
        let res = render_nodes(nodes, &vars, &DashMap::new());

        assert!(res.is_ok());

        let res = res.unwrap();
        assert_eq!(&res, "Hello World");
    }

    #[test]
    fn basic_function_render() {
        let vars = DashMap::from_iter([("var", "WoRld".to_string())]);
        let funcs = DashMap::from_iter([(
            "toLowerCase",
            Box::new(builtin::to_lower as TemplateFunction),
        )]);
        let template = "Hello {{ toLowerCase(var) }}";
        let (_, nodes) = parser(template).unwrap();
        let res = render_nodes(nodes, &vars, &funcs);

        assert!(res.is_ok());

        let res = res.unwrap();
        assert_eq!(res, "Hello world".to_string());
    }

    #[test]
    fn recursive_function_render() {
        let vars = DashMap::from_iter([("var", "    WoRlD ".to_string())]);
        let funcs = DashMap::from_iter([
            (
                "toLowerCase",
                Box::new(builtin::to_lower as TemplateFunction),
            ),
            ("trim", Box::new(builtin::trim as TemplateFunction)),
        ]);
        let template = "Hello {{ toLowerCase(trim(var)) }}";
        let (_, nodes) = parser(template).unwrap();
        let res = render_nodes(nodes, &vars, &funcs);

        assert!(res.is_ok());

        let res = res.unwrap();
        assert_eq!(res, "Hello world".to_string());
    }

    #[test]
    fn raw_string_render() {
        let vars = DashMap::from_iter([("var", "    WoRlD ".to_string())]);
        let funcs = DashMap::from_iter([
            (
                "toLowerCase",
                Box::new(builtin::to_lower as TemplateFunction),
            ),
            ("trim", Box::new(builtin::trim as TemplateFunction)),
        ]);
        let template = r#"Hello
{{ toLowerCase(trim(var)) }}"#;
        let (_, nodes) = parser(template).unwrap();
        let res = render_nodes(nodes, &vars, &funcs);

        assert!(res.is_ok());

        let res = res.unwrap();
        assert_eq!(res, "Hello\nworld".to_string());
    }
}
