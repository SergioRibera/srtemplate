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

/// This corresponds to the type for custom functions that may exist.
pub type TemplateFunction = fn(&[String]) -> FuncResult;

/// This structure is the basis of everything, it is responsible for managing variables and functions.
///
/// # Examples
/// ```no_run
/// let mut ctx = SrTemplate::default();
/// ctx.add_variable("var", &"World");
/// ctx.add_variable("otherVar", &"Other");
/// ctx.add_variable("number", &85u8);
///
/// ctx.render("Hello {{ var }}! This is {{ otherVar }} and this is number: {{number}}").unwrap()
/// ```
#[derive(Clone)]
pub struct SrTemplate<'a> {
    variables: Arc<DashMap<&'a str, Box<Cow<'a, str>>>>,
    functions: Arc<DashMap<&'a str, Box<TemplateFunction>>>,
}

impl<'a> SrTemplate<'a> {
    /// Adds variables that can later be rendered in the template
    ///
    /// # Arguments
    ///
    /// * `name`: Variable name, this name is the one you will use in the template
    /// * `value`: This is the value on which the template will be replaced in the template
    pub fn add_variable<T: ToString>(&mut self, name: &'a str, value: &T) {
        self.variables
            .insert(name, Box::new(value.to_string().into()));
    }

    /// Adds functions that can later be rendered in the template
    ///
    /// # Arguments
    ///
    /// * `name`: Function name, this name is the one you will use in the template
    /// * `func`: This is the function that will be evaluated when it is called from the template
    pub fn add_function(&mut self, name: &'a str, func: TemplateFunction) {
        self.functions.insert(name, Box::new(func));
    }

    /// Renders text as a template, replacing variables and processing functions.
    ///
    /// # Arguments
    ///
    /// * `text`: The text in template format to be processed.
    ///
    /// # Returns
    ///
    /// A `Result` where `Ok` contains the rendered template as a `String`, and `Err` holds a [`SrTemplateError`] if an error occurs.
    ///
    /// # Example
    ///
    /// ```rust
    /// use srtemplate::prelude::SrTemplate;
    /// use srtemplate::prelude::SrTemplateError;
    ///
    /// let ctx = SrTemplate::default();
    /// let template = "Hello, {{ name }}!";
    /// match ctx.render(template) {
    ///     Ok(rendered) => println!("Rendered: {}", rendered),
    ///     Err(err) => eprintln!("Error: {:?}", err),
    /// }
    /// ```
    pub fn render(&self, text: &str) -> Result<String, SrTemplateError> {
        let (left, nodes) = parser(text).map_err(|e| SrTemplateError::BadSyntax(e.to_string()))?;
        let res = render_nodes(nodes, &self.variables.clone(), &self.functions.clone())?;
        Ok(format!("{left}{res}"))
    }
}

impl<'a> Default for SrTemplate<'a> {
    /// Generates an instance with all the builtin functions that are enabled from features
    fn default() -> Self {
        let mut tmp = Self {
            variables: Arc::default(),
            functions: Arc::default(),
        };

        #[cfg(feature = "os")]
        tmp.add_function("env", builtin::os::env);

        #[cfg(feature = "text")]
        {
            tmp.add_function("toLower", builtin::text::to_lower);
            tmp.add_function("toUpper", builtin::text::to_upper);
            tmp.add_function("trim", builtin::text::trim);
        }

        #[cfg(feature = "math")]
        {
            gen_math_use!(tmp);
        }

        tmp
    }
}
