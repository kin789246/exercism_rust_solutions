pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec!["".to_string(); digits.len()+1]
    }
    let mut v: Vec<String> = Vec::new();
    for i in 0.. {
        if i+len <= digits.len() {
            v.push((&digits[i..(i+len)]).to_string());
        }
        else {
            break;
        }
    }
    v
}
