/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        //unimplemented!("if the value {value} is a palindrome return Some, otherwise return None");
        let mut vec: Vec<u64> = Vec::new();
        let mut v = value;
        while v > 0 {
            vec.push(v%10);
            v /= 10;
        }
        let mut r = vec.clone();
        r.reverse();
        if vec == r { return Some(Palindrome(value)) }
        return None
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut min_p = Palindrome(u64::MAX);
    let mut max_p = Palindrome(min);
    for i in min..=max {
        for j in i..=max {
            if let Some(p) = Palindrome::new(i*j) {
                if min_p.into_inner() > p.into_inner() { min_p = p; }
                if max_p.into_inner() < p.into_inner() { max_p = p; }
            }
        }
    }
    if min_p == Palindrome(u64::MAX) && max_p == Palindrome(min) { return None }
    Some((min_p, max_p))
}
