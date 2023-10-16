pub fn number(user_number: &str) -> Option<String> {
    let no = user_number.chars()
        .filter(|c| c.is_numeric())
        .collect::<String>();

    if no.len() == 10 {
        if no.chars().nth(0).unwrap() < '2' || no.chars().nth(3).unwrap() < '2' {
            return None;
        }
        return Some(no);
    }
    else if no.len() == 11 {
        if no.chars().nth(0).unwrap() != '1' {
            return None;
        }
        if no.chars().nth(1).unwrap() < '2' || no.chars().nth(4).unwrap() < '2' {
            return None;
        }
        return Some(no.get(1..).unwrap().to_string());
    }
    else {
        return None;
    }
}
