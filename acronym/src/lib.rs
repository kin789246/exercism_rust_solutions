pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c:char| c.is_whitespace() || c == '-' || c == '_')
        .collect::<Vec<_>>()
        .iter()
        .flat_map(|w| w.chars().take(1).chain(
            w.chars().skip_while(|c| c.is_uppercase())
                .filter(|c| c.is_uppercase())))
        .collect::<String>()
        .to_uppercase()
}