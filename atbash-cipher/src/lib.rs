/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .to_lowercase()
        .as_bytes()
        .iter()
        .map( |x| match x {
            c if c.is_ascii_alphabetic() => {
                //println!("{} = {}", *c, *c as char);
                25 - (*c - b'a') + b'a'
            },
            c if c.is_ascii_digit() => {
                *c
            },
            _ => 0
        })
        .filter(|x| *x != 0)
        .collect::<Vec<u8>>()
        .chunks(5)
        .map(|arr| {
            arr.iter().map(|x| *x as char).collect::<String>()
        })
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .to_lowercase()
        .as_bytes()
        .iter()
        .map( |x| match x {
            c if c.is_ascii_alphabetic() => {
                (25- (*c - b'a') + b'a') as char
            },
            c if c.is_ascii_digit() => {
                *c as char
            },
            _ => 0 as char
        })
        .filter(|x| *x != '\0')
        .collect::<String>()
}
