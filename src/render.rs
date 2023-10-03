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
