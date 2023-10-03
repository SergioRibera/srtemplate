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
        let (left, nodes) = parser(text).map_err(|e| SrTemplateError::BadSyntax(e.to_string()))?;
        let res = render_nodes(nodes, &self.variables, &self.functions)?;
        Ok(format!("{left}{res}"))
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
