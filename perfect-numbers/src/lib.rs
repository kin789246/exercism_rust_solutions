#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 { return None }
    let s: u64 = (1..num).filter(|n| num%n == 0).sum();
    if num == s { return Some(Classification::Perfect) }
    if num > s { return Some(Classification::Deficient) }
    if num < s { return Some(Classification::Abundant) }    
    else { None }
}
