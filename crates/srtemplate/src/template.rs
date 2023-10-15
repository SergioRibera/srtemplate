use dashmap::DashMap;
#[cfg(feature = "math")]
use paste::paste;
use std::borrow::Cow;
use std::sync::Arc;

use crate::builtin;
use crate::error::SrTemplateError;
use crate::parser::parser;
use crate::render::render_nodes;

#[cfg(feature = "math")]
use crate::gen_math_use;

use self::function::FuncResult;

pub mod function;
pub mod validations;

pub type TemplateFunction = fn(&[String]) -> FuncResult;

#[derive(Clone)]
pub struct SrTemplate<'a> {
    variables: Arc<DashMap<&'a str, Box<Cow<'a, str>>>>,
    functions: Arc<DashMap<&'a str, Box<TemplateFunction>>>,
}

impl<'a> SrTemplate<'a> {
    pub fn add_variable<T: ToString>(&mut self, name: &'a str, value: &T) {
        self.variables
            .insert(name, Box::new(value.to_string().into()));
    }

    pub fn add_function(&mut self, name: &'a str, func: TemplateFunction) {
        self.functions.insert(name, Box::new(func));
    }

    pub fn render(&self, text: &str) -> Result<String, SrTemplateError> {
        let (left, nodes) = parser(text).map_err(|e| SrTemplateError::BadSyntax(e.to_string()))?;
        let res = render_nodes(nodes, &self.variables.clone(), &self.functions.clone())?;
        Ok(format!("{left}{res}"))
    }
}

impl<'a> Default for SrTemplate<'a> {
    fn default() -> Self {
        let mut tmp = Self {
            variables: Arc::default(),
            functions: Arc::default(),
        };

        #[cfg(feature = "os")]
        tmp.add_function("env", builtin::env);

        #[cfg(feature = "text")]
        {
            tmp.add_function("toLower", builtin::to_lower);
            tmp.add_function("toUpper", builtin::to_upper);
            tmp.add_function("trim", builtin::trim);
        }

        #[cfg(feature = "math")]
        {
            gen_math_use!(tmp);
        }

        tmp
    }
}
