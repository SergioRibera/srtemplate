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
/// use srtemplate::SrTemplate;
///
/// let mut ctx = SrTemplate::default();
/// ctx.add_variable("var", &"World");
/// ctx.add_variable("otherVar", &"Other");
/// ctx.add_variable("number", &85u8);
///
/// ctx.render("Hello {{ var }}! This is {{ otherVar }} and this is number: {{number}}").unwrap();
/// ```
#[derive(Clone)]
pub struct SrTemplate<'a> {
    variables: Arc<DashMap<Cow<'a, str>, String>>,
    functions: Arc<DashMap<Cow<'a, str>, Box<TemplateFunction>>>,
}

impl<'a> SrTemplate<'a> {
    /// Adds variables that can later be rendered in the template
    ///
    /// # Arguments
    ///
    /// * `name`: Variable name, this name is the one you will use in the template
    /// * `value`: This is the value on which the template will be replaced in the template
    pub fn add_variable<U: Into<Cow<'a, str>>, T: ToString>(&self, name: U, value: &T) {
        self.variables.insert(name.into(), value.to_string());
    }

    /// Adds functions that can later be rendered in the template
    ///
    /// # Arguments
    ///
    /// * `name`: Function name, this name is the one you will use in the template
    /// * `func`: This is the function that will be evaluated when it is called from the template
    pub fn add_function<T: Into<Cow<'a, str>>>(&self, name: T, func: TemplateFunction) {
        self.functions.insert(name.into(), Box::new(func));
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
    /// ```no_run
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
    pub fn render<T: AsRef<str>>(&self, text: T) -> Result<String, SrTemplateError> {
        let text = text.as_ref();
        let (r, nodes) = parser(text).map_err(|e| SrTemplateError::BadSyntax(e.to_string()))?;
        let res = render_nodes(nodes, &self.variables.clone(), &self.functions.clone())?;
        let res = if text.starts_with(r) {
            format!("{r}{res}")
        } else {
            format!("{res}{r}")
        };
        Ok(res)
    }
}

impl<'a> Default for SrTemplate<'a> {
    /// Generates an instance with all the builtin functions that are enabled from features
    fn default() -> Self {
        let tmp = Self {
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
