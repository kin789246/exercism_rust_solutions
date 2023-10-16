use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let parsed: String = candidate.split(&['-',' '][..]).collect();
    //println!("{:?}", parsed.chars());
    let n = parsed.len();
    let mut d: HashSet<char> = HashSet::new();
    parsed
        .to_lowercase()
        .chars()
        .for_each(|c| { d.insert(c); });

    n == d.len()
}
