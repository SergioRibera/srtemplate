use function::gen_function;
use proc_macro::TokenStream;
use venial::{parse_item, Error, Item};

mod function;
mod variable;

#[proc_macro_attribute]
pub fn function(_: TokenStream, body: TokenStream) -> TokenStream {
    let func = match parse_item(body.into()) {
        Ok(Item::Function(func)) => Ok(func),
        Err(e) => Err(e),
        Ok(_) => Err(Error::new("Just support functions")),
    };

    func.and_then(gen_function)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

#[proc_macro_derive(Variable, attributes(template))]
pub fn derive_variable(input: TokenStream) -> TokenStream {
    variable::derive(input.into())
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum TextCase {
    Lower,
    Upper,
    Sentence,
    Snake,
    Camel,
    Pascal,
    Kebab,
    ScreamingSnake,
}

impl TextCase {
    pub fn convert(&self, s: &str) -> String {
        match self {
            TextCase::Lower => s.to_lowercase(),
            TextCase::Upper => s.to_uppercase(),
            _ => {
                let words = Self::split_into_words(s);
                self.process_words(words)
            }
        }
    }

    fn split_into_words(s: &str) -> Vec<String> {
        let mut words = Vec::new();
        let mut current_word = String::new();
        let mut prev_char = None::<char>;

        for c in s.chars() {
            if c.is_ascii_punctuation() || c.is_whitespace() {
                if !current_word.is_empty() {
                    words.push(current_word);
                    current_word = String::new();
                }
            } else if c.is_uppercase() {
                if let Some(prev) = prev_char {
                    if prev.is_lowercase() || prev.is_ascii_digit() {
                        if !current_word.is_empty() {
                            words.push(current_word);
                            current_word = String::new();
                        }
                    }
                }
                current_word.push(c);
            } else if c.is_lowercase() {
                if let Some(prev) = prev_char {
                    if prev.is_uppercase() && current_word.len() > 1 {
                        let last_char = current_word.pop().unwrap();
                        if !current_word.is_empty() {
                            words.push(current_word);
                        }
                        current_word = String::from(last_char);
                    }
                }
                current_word.push(c);
            } else {
                current_word.push(c);
            }
            prev_char = Some(c);
        }

        if !current_word.is_empty() {
            words.push(current_word);
        }

        words
    }

    fn process_words(&self, words: Vec<String>) -> String {
        match self {
            TextCase::Snake => words
                .iter()
                .map(|word| word.to_lowercase())
                .collect::<Vec<_>>()
                .join("_"),

            TextCase::Camel => {
                if words.is_empty() {
                    return String::new();
                }
                let mut processed = vec![words[0].to_lowercase()];
                processed.extend(words[1..].iter().map(|w| Self::capitalize(w)));
                processed.join("")
            }

            TextCase::Pascal => words
                .iter()
                .map(|word| Self::capitalize(word))
                .collect::<Vec<_>>()
                .join(""),

            TextCase::Kebab => words
                .iter()
                .map(|word| word.to_lowercase())
                .collect::<Vec<_>>()
                .join("-"),

            TextCase::ScreamingSnake => words
                .iter()
                .map(|word| word.to_uppercase())
                .collect::<Vec<_>>()
                .join("_"),

            _ => String::new(),
        }
    }

    fn capitalize(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => {
                let first_upper = first.to_uppercase().collect::<String>();
                let rest_lower: String = chars.collect::<String>().to_lowercase();
                format!("{}{}", first_upper, rest_lower)
            }
        }
    }
}

impl std::str::FromStr for TextCase {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "lower" | "lowercase" => Ok(TextCase::Lower),
            "upper" | "uppercase" => Ok(TextCase::Upper),
            "sentence" | "sentencecase" => Ok(TextCase::Sentence),
            "snake" | "snakecase" => Ok(TextCase::Snake),
            "camel" | "camelcase" => Ok(TextCase::Camel),
            "pascal" | "pascalcase" => Ok(TextCase::Pascal),
            "kebab" | "kebabcase" => Ok(TextCase::Kebab),
            "screamingsnake" | "screaming_snake" | "screaming-snake" | "screaming snake" => {
                Ok(TextCase::ScreamingSnake)
            }
            _ => Err(format!("Invalid text case: {}", s)),
        }
    }
}

impl<T: AsRef<str>> From<T> for TextCase {
    fn from(s: T) -> Self {
        s.as_ref()
            .parse()
            .unwrap_or_else(|_| panic!("Failed to convert '{}' into TextCase", s.as_ref()))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!("lowercase".parse::<TextCase>(), Ok(TextCase::Lower));
        assert_eq!("UPPERCASE".parse::<TextCase>(), Ok(TextCase::Upper));
        assert_eq!("snake_case".parse::<TextCase>(), Ok(TextCase::Snake));
        assert_eq!("camelCase".parse::<TextCase>(), Ok(TextCase::Camel));
        assert_eq!("PascalCase".parse::<TextCase>(), Ok(TextCase::Pascal));
        assert_eq!("kebab-case".parse::<TextCase>(), Ok(TextCase::Kebab));
        assert_eq!(
            "SCREAMING_SNAKE".parse::<TextCase>(),
            Ok(TextCase::ScreamingSnake)
        );
        assert!("invalid_case".parse::<TextCase>().is_err());
    }

    #[test]
    fn test_into() {
        let lower: TextCase = "lowercase".into();
        assert!(matches!(lower, TextCase::Lower));

        let snake: TextCase = "snakecase".into();
        assert!(matches!(snake, TextCase::Snake));

        let screaming_snake: TextCase = "screaming_snake".into();
        assert!(matches!(screaming_snake, TextCase::ScreamingSnake));
    }

    #[test]
    fn test_case_conversions() {
        let input = "hello_world-example 123HTTPRequest";

        assert_eq!(
            TextCase::Lower.convert(input),
            "hello_world-example 123httpprequest"
        );
        assert_eq!(
            TextCase::Upper.convert(input),
            "HELLO_WORLD-EXAMPLE 123HTTPREQUEST"
        );
        assert_eq!(
            TextCase::Snake.convert(input),
            "hello_world_example_123_http_request"
        );
        assert_eq!(
            TextCase::Camel.convert(input),
            "helloWorldExample123HttpRequest"
        );
        assert_eq!(
            TextCase::Pascal.convert(input),
            "HelloWorldExample123HttpRequest"
        );
        assert_eq!(
            TextCase::Kebab.convert(input),
            "hello-world-example-123-http-request"
        );
        assert_eq!(
            TextCase::ScreamingSnake.convert(input),
            "HELLO_WORLD_EXAMPLE_123_HTTP_REQUEST"
        );
    }
}
