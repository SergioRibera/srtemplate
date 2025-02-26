use dashmap::DashMap;
#[cfg(feature = "math")]
use paste::paste;
use std::borrow::Cow;
use std::sync::Arc;

use crate::builtin;
use crate::error::Error;
use crate::parser::parser;
use crate::render::nodes;

#[cfg(feature = "math")]
use crate::gen_math_use;

use self::function::FuncResult;

pub mod function;
pub mod validations;

/// This corresponds to the type for custom functions that may exist.
pub type Function = fn(&[String]) -> FuncResult;

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
#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
pub struct SrTemplate<'a> {
    delimiter_start: Cow<'a, str>,
    delimiter_close: Cow<'a, str>,
    variables: Arc<DashMap<Cow<'a, str>, String>>,
    functions: Arc<DashMap<Cow<'a, str>, Box<Function>>>,
}

impl<'a> SrTemplate<'a> {
    /// Create instance of SrTemplate with custom delimiters
    ///
    /// # Arguments
    ///
    /// * `start`: The start delimiter. This is a string slice or a type that can be converted into a `Cow<str>`.
    /// * `close`: The end delimiter. This is a string slice or a type that can be converted into a `Cow<str>`.
    pub fn with_delimiter<U: Into<Cow<'a, str>>>(start: U, close: U) -> Self {
        Self {
            delimiter_start: start.into(),
            delimiter_close: close.into(),
            ..Default::default()
        }
    }

    /// Adds variable that can later be rendered in the template
    ///
    /// # Arguments
    ///
    /// * `name`: Variable name, this name is the one you will use in the template
    /// * `value`: This is the value on which the template will be replaced in the template
    pub fn add_variable<U: Into<Cow<'a, str>>, T: ToString>(&self, name: U, value: T) {
        let value = value.to_string();
        self.variables
            .entry(name.into())
            .and_modify(|old| *old = value.clone())
            .or_insert_with(|| value.clone());
    }

    /// Adds variables that can later be rendered in the template
    ///
    /// # Arguments
    ///
    /// * `name`: Variable name, this name is the one you will use in the template
    /// * `value`: This is the value on which the template will be replaced in the template
    pub fn add_variables<U: Into<Cow<'a, str>>, V: Iterator<Item = (U, &'a dyn ToString)>>(
        &self,
        values: V,
    ) {
        values.for_each(|(name, value)| {
            self.add_variable(name, value.to_string());
        });
    }

    /// Adds function that can later be rendered in the template
    ///
    /// # Arguments
    ///
    /// * `name`: Function name, this name is the one you will use in the template
    /// * `func`: This is the function that will be evaluated when it is called from the template
    pub fn add_function<T: Into<Cow<'a, str>>>(&self, name: T, func: Function) {
        self.functions
            .entry(name.into())
            .and_modify(|old| *old = Box::new(func.clone()))
            .or_insert_with(|| Box::new(func));
    }

    /// Adds functions that can later be rendered in the template
    ///
    /// # Arguments
    ///
    /// * `name`: Function name, this name is the one you will use in the template
    /// * `func`: This is the function that will be evaluated when it is called from the template
    pub fn add_functions<U: Into<Cow<'a, str>>, V: Iterator<Item = (U, Function)>>(
        &self,
        values: V,
    ) {
        values.for_each(|(name, func)| {
            self.add_function(name, func);
        });
    }

    /// Checks if a variable exists in the template string by its name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the variable to check.
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if the variable exists, `false` otherwise.
    /// ```
    pub fn contains_variable<T: Into<Cow<'a, str>>>(&self, name: T) -> bool {
        self.variables.contains_key(&name.into())
    }

    /// Checks if a function exists in the template string by its name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the function to check.
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if the function exists, `false` otherwise.
    pub fn contains_function<T: Into<Cow<'a, str>>>(&self, name: T) -> bool {
        self.functions.contains_key(&name.into())
    }

    /// Removes a variable from the template string by its name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the variable to remove.
    pub fn remove_variable<T: Into<Cow<'a, str>>>(&self, name: T) {
        self.variables.remove(&name.into());
    }

    /// Removes a function from the template string by its name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the function to remove.
    pub fn remove_function<T: Into<Cow<'a, str>>>(&self, name: T) {
        self.functions.remove(&name.into());
    }

    /// Clears all variables from the template string.
    pub fn clear_variables(&self) {
        self.variables.clear();
    }

    /// Clears all functions from the template string.
    pub fn clear_functions(&self) {
        self.functions.clear();
    }

    /// Sets the delimiters for the template string.
    ///
    /// This function allows you to define the start and end delimiters that will be used to identify
    /// the content within the template string. The delimiters can be any string or character sequence
    /// that does not conflict with the template content.
    ///
    /// # Arguments
    ///
    /// * `start`: The start delimiter. This is a string slice or a type that can be converted into a `Cow<str>`.
    /// * `close`: The end delimiter. This is a string slice or a type that can be converted into a `Cow<str>`.
    pub fn set_delimiter<U: Into<Cow<'a, str>>>(&mut self, start: U, close: U) {
        self.delimiter_start = start.into();
        self.delimiter_close = close.into();
    }

    /// Renders a template by replacing variables and processing functions.
    ///
    /// # Arguments
    ///
    /// * `text` - A template string to be rendered.
    ///
    /// # Returns
    ///
    /// A `Result` where:
    /// - `Ok(String)` contains the rendered template as a string.
    /// - `Err(Error)` contains the details of an error if rendering fails.
    ///
    /// # Example
    ///
    /// ```
    /// use srtemplate::prelude::SrTemplate;
    ///
    /// let ctx = SrTemplate::default();
    /// let template = "Hello, {{ name }}!";
    /// match ctx.render(template) {
    ///     Ok(rendered) => println!("Rendered: {}", rendered),
    ///     Err(err) => eprintln!("Error: {:?}", err),
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The syntax of the template is invalid.
    /// - A variable or function is not found or fails during processing.
    pub fn render<T: AsRef<str>>(&self, text: T) -> Result<String, Error> {
        let input = text.as_ref();
        let open_delim = self.delimiter_start.as_ref();
        let close_delim = self.delimiter_close.as_ref();
        let mut res = String::with_capacity(input.len());
        let tnodes = parser(input, open_delim, close_delim)?;

        for var in tnodes {
            nodes(
                &mut res,
                var,
                self.variables.as_ref(),
                self.functions.as_ref(),
            )?;
        }
        Ok(res)
    }
}

impl Default for SrTemplate<'_> {
    /// Generates an instance with all the builtin functions that are enabled from features
    fn default() -> Self {
        let tmp = Self {
            delimiter_start: "{{".into(),
            delimiter_close: "}}".into(),
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
