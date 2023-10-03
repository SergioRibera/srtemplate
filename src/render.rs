use std::collections::HashMap;

use crate::error::SrTemplateError;
use crate::parser::TemplateNode;
use crate::template::TemplateFunction;

pub fn render_nodes(
    nodes: Vec<TemplateNode>,
    vars: &HashMap<&str, String>,
    funcs: &HashMap<&str, Box<TemplateFunction>>,
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
                    .ok_or(SrTemplateError::FunctionNotSupported(function))?;

                let evaluated_arguments = evaluated_arguments?;
                println!("Evaluated Args: {evaluated_arguments:?}");

                let result_of_function = func(evaluated_arguments);
                println!("Result of function: {result_of_function:?}");

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

    use super::*;

    #[test]
    fn basic_render() {
        let vars = HashMap::from_iter([("var", "Mundo".to_string())]);
        let template = "Hola {{ var }}";
        let (_, nodes) = parser(template).unwrap();
        let res = render_nodes(nodes, &vars, &HashMap::new());

        assert!(res.is_ok());

        let res = res.unwrap();
        assert_eq!(&res, "Hola Mundo");
    }

    #[test]
    fn basic_function_render() {
        let vars = HashMap::from_iter([("var", "MuNdO".to_string())]);
        let funcs = HashMap::from_iter([
            (
                "toLowerCase",
                Box::new(builtin::to_lower as TemplateFunction),
            ),
            ("trim", Box::new(builtin::trim as TemplateFunction)),
        ]);
        let template = "Hola {{ toLowerCase(trim(var)) }}";
        let (_, nodes) = parser(template).unwrap();
        let res = render_nodes(nodes, &vars, &funcs);

        assert!(res.is_ok());

        let res = res.unwrap();
        assert_eq!(res, "Hola mundo".to_string());
    }
}
