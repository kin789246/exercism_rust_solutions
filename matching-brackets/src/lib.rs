pub fn brackets_are_balanced(string: &str) -> bool {
    //unimplemented!("Check if the string \"{string}\" contains balanced brackets");
    let mut balance: Vec<char> = Vec::new();
    for c in string.chars() {
        match c {
            '[' | '{' | '(' => balance.push(c),
            ']' => if balance.pop() != Some('[') { return false },
            '}' => if balance.pop() != Some('{') { return false },
            ')' => if balance.pop() != Some('(') { return false },
            _ => continue
        }
    }
    return balance.is_empty()
}
