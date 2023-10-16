/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let scores: Vec<u64> = vec![
        1, 3, 3, 2, 1, 4, 2, 4, 1, 8,   // ABCDEFGHIJ
        5, 1, 3, 1, 1, 3, 10, 1, 1, 1,  // KLMNOPQRST
        1, 4, 4, 8, 4, 10,              // UVWXYZ
    ];
    word.to_lowercase()
        .as_bytes()
        .iter()
        .filter(|x| 97<=**x && **x<=123)
        .fold(0, |acc, x| acc + scores[(x-97) as usize])
}
