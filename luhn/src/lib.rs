/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if !code.chars().all(|c| c.is_digit(10) || c.is_whitespace()) {
        return false
    }
    let mut luhn = code
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    if luhn.len() == 1 && luhn[0] == 0 {
        return false
    }
    let n = luhn.len();
    for (i, d) in luhn.iter_mut().enumerate() {
        if n % 2 == 1 {
            if i%2 == 1 {
                *d *= 2;
                if *d > 9 { *d -= 9; }
            }
        }
        else {
            if i%2 == 0 {
                *d *= 2;
                if *d > 9 { *d -= 9; }
            }            
        }
    }
    if luhn.iter().sum::<u32>() % 10 != 0 {
        return false
    }
    true
}
