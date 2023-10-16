pub fn reply(message: &str) -> &str {
    //unimplemented!("have Bob reply to the incoming message: {message}")
    let message = message.trim();
    if message.is_empty() {
        return "Fine. Be that way!"
    }
    let v: Vec<char> = message.chars().filter(|x| x.is_ascii_alphabetic()).collect();
    if message.chars().last().unwrap() == '?' {
        if v.len() != 0 && v.iter().filter(|x| x.is_ascii_lowercase()).count() == 0 {
            return "Calm down, I know what I'm doing!"
        }
        else {
            return "Sure."
        }
    }
    if v.len() != 0 && v.iter().filter(|x| x.is_ascii_lowercase()).count() == 0 {
        return "Whoa, chill out!"
    }
    
    "Whatever."
}
