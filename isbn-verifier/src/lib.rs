/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut parsed: Vec<u32> = Vec::new();
    for (i, c) in isbn.chars().enumerate() {
        if !c.is_ascii_digit() {
            if i == isbn.len()-1 && c == 'X' {
                parsed.push(10);
            }
            else if c == '-' { continue; }
            else { return false }
        }
        else { parsed.push(c.to_digit(10).unwrap()); }
    }
    //println!("{:?}", parsed);
    if parsed.len() != 10 { return false }
    let mut s = 0;
    let mut t = 0;
    for i in 0..10 {
        t += parsed[i];
        s += t;
    }
    s % 11 == 0
}
