use std::collections::HashMap;

use crate::builtin;
use crate::error::SrTemplateError;
use crate::parser::parser;
use crate::render::render_nodes;

pub type TemplateFunction = fn(Vec<String>) -> String;

pub struct SrTemplate<'a> {
    variables: HashMap<&'a str, String>,
    functions: HashMap<&'a str, Box<TemplateFunction>>,
}

impl<'a> SrTemplate<'a> {
    pub fn add_variable(&mut self, name: &'a str, value: String) {
        self.variables.entry(name.as_ref()).or_insert(value);
    }

    pub fn add_function(&mut self, name: &'a str, func: TemplateFunction) {
        self.functions
            .entry(name.as_ref())
            .or_insert(Box::new(func));
    }

    pub fn render(&self, text: &str) -> Result<String, SrTemplateError> {
        let (_, nodes) = parser(text).map_err(|e| SrTemplateError::BadSyntax(e.to_string()))?;
        render_nodes(nodes, &self.variables, &self.functions)
    }
}

impl<'a> Default for SrTemplate<'a> {
    fn default() -> Self {
        let mut tmp = Self {
            variables: Default::default(),
            functions: Default::default(),
        };

        #[cfg(feature = "os")]
        tmp.add_function("env", builtin::env);

        #[cfg(feature = "text")]
        {
            tmp.add_function("toLower", builtin::to_lower);
            tmp.add_function("toUpper", builtin::to_upper);
            tmp.add_function("trim", builtin::trim);
        }

        tmp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_render() {
        let mut ctx = SrTemplate::default();
        ctx.add_variable("var", "Mundo".to_string());
        let template = "Hola {{ var }}";
        let res = ctx.render(template);

        assert!(res.is_ok());

        let res = res.unwrap();
        assert_eq!(res, "Hola Mundo".to_string());
    }

    #[test]
    fn basic_function_render() {
        let mut ctx = SrTemplate::default();
        ctx.add_variable("var", "MuNdO".to_string());
        let template = "Hola {{ toLowerCase(var)}}";
        let res = ctx.render(template);

        assert!(res.is_ok());

        let res = res.unwrap();
        assert_eq!(res, "Hola mundo".to_string());
    }
}
