use rand::prelude::*;

pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.chars().any(|c| !c.is_lowercase()) {
        return None;
    }
    if key.len() == 0 {
        return None;
    }

    let mut res: String = String::new();
    s.as_bytes().iter().enumerate().for_each(|(i, c)| {
        //println!("{} {:?}", *c as char, key.as_bytes().iter().nth(i));
        if let Some(nth_key) = key.as_bytes().iter().nth(i) {
            res.push(
                ((*c -b'a' + *nth_key - b'a') %26 + b'a') as char
            );
        }
        else {
            res.push(*c as char);
        }
    });
    Some(res)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if key.chars().any(|c| !c.is_lowercase()) {
        return None;
    }
    if key.len() == 0 {
        return None;
    }

    let mut res: String = String::new();
    s.as_bytes().iter().enumerate().for_each(|(i, c)| {
        if let Some(nth_key) = key.as_bytes().iter().nth(i) {
            res.push(
                ((*c -b'a' + 26 - (*nth_key - b'a')) %26 + b'a') as char
            );
        }
        else {
            res.push(*c as char);
        }
    });
    Some(res)
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = thread_rng();
    let mut key: String = String::new();
    for _i in 0..100 {
        key.push(char::from_u32(rng.gen_range(0..26) + b'a' as u32).unwrap());
    }
    (key.clone(), encode(&key, s).unwrap())
}
