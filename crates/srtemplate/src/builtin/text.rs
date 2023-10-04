pub fn to_lower(args: Vec<String>) -> String {
    args.iter()
        .map(|a| a.to_lowercase())
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn to_upper(args: Vec<String>) -> String {
    args.iter()
        .map(|a| a.to_uppercase())
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn trim(args: Vec<String>) -> String {
    args.iter()
        .map(|a| a.trim())
        .collect::<Vec<_>>()
        .join(" ")
}
