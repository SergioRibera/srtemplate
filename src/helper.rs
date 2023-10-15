#[doc(hidden)]
pub mod arg;
pub mod serialize;

#[cfg(test)]
mod test {
    use super::serialize::FromArgs;

    #[test]
    fn simple_parse_args() {
        let args = vec!["abc".to_string(), "54".to_string(), "4.5".to_string()];

        let a = <(String, u8)>::from_args(&args);

        assert!(a.is_ok());
    }

    #[test]
    fn bad_parse_args() {
        let args = vec!["abc".to_string(), "2 75".to_string(), "4.5".to_string()];

        let a = <(String, f32)>::from_args(&args);

        assert!(a.is_err());
    }

    #[test]
    fn incomplete_parse_args() {
        let args = vec!["abc".to_string()];

        let a = <(String, i32)>::from_args(&args);

        println!("{a:?}");

        assert!(a.is_err());
    }

    #[test]
    fn complete_parse_args() {
        let args = vec!["abc".to_string(), "54".to_string(), "4.5".to_string()];

        let a = <(String, i32, f64)>::from_args(&args);

        assert!(a.is_ok());

        let (a, b, c) = a.unwrap();

        assert_eq!(a, "abc".to_string());
        assert_eq!(b, 54i32);
        assert_eq!(c, 4.5f64);
    }
}
