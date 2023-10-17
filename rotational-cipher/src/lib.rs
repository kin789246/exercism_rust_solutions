pub fn rotate(input: &str, key: i8) -> String {
    input.as_bytes().iter()
        .map(|c| {
            if c.is_ascii_lowercase() {
                return ((*c - b'a' + (key + 26) as u8) % 26 + b'a') as char
            }
            else if c.is_ascii_uppercase() {
                return ((*c - b'A' + (key + 26) as u8) % 26 + b'A') as char
            }
            else {
                return *c as char
            }
        })
        .collect::<String>()
}
