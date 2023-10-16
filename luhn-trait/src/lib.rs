pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T> Luhn for T where T: ToString {
    fn valid_luhn(&self) -> bool {
        let code = self.to_string();
        println!("{code}");
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
}
