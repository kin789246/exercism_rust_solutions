use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut all_letters: HashSet<char> = HashSet::new();
    for c in 'A'..='Z' { all_letters.insert(c); }
    sentence.to_uppercase().chars().for_each(|c| {
        all_letters.remove(&c);
    });
    all_letters.len() == 0
}
