/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() { return None }
    let hd = s1.chars().zip(s2.chars()).filter(|(x, y)| *x != *y).count();
    Some(hd)
}
