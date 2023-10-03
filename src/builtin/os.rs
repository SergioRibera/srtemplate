pub fn env(args: Vec<String>) -> String {
    args.iter()
        .map(|a| std::env::var(a).unwrap_or("NotFound".to_string()))
        .collect::<Vec<String>>()
        .join(" ")
}
