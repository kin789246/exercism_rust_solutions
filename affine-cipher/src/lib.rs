/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

// is_coprime function
fn gcd(a: i32, b: i32) -> i32 {
    if b > 0 {
        return gcd(b, a%b);
    }
    a
}

fn is_coprime(a: i32, b: i32) -> bool {
    gcd(a, b) == 1
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !is_coprime(a, 26) { 
        return Err(AffineCipherError::NotCoprime(a)); 
    }
    let mut ciphers: Vec<u8> = Vec::new();
    plaintext
        .to_ascii_lowercase()
        .as_bytes()
        .iter()
        .for_each( |x| {
            if x.is_ascii_digit() {
                ciphers.push(*x);
            }
            else if !x.is_ascii_lowercase() {
                return;
            }
            else {
                let e = (a * (x - b'a') as i32 + b) % 26;
                ciphers.push(e as u8 + b'a');
            }
        });

    Ok(ciphers.chunks(5)
       .map(|arr| {
           arr.iter().map(|x| *x as char).collect::<String>()
       })
       .collect::<Vec<String>>()
       .join(" ")
    ) 
}


fn get_MMI(a: i32) -> i32 {
    let mut i = 1;
    loop {
        if a*i % 26 == 1 {
            break;
        }
        i += 1;
    }
    i
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !is_coprime(a, 26) { 
        return Err(AffineCipherError::NotCoprime(a)); 
    }
    let mmi = get_MMI(a);
    Ok(ciphertext.as_bytes()
        .iter()
        .map( |x| {
            if x.is_ascii_lowercase() {
                // println!("{a}, {b}, {mmi}, {}", *x-b'a');
                // println!("dy = {:?}", ((mmi * ((*x - b'a') as i32 - b) % 26).abs() as u8 + b'a') as char);
                ((((mmi * ((*x - b'a') as i32 - b) % 26) + 26) % 26) as u8 + b'a') as char
            }
            else {
                *x as char
            }
        })
        .filter(|x| !x.is_whitespace())
        .collect::<String>()
    )
}
