use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut res: HashMap<String, u32> = HashMap::new();
    words.to_lowercase()
        .split(|c:char| c.is_whitespace() || (c.is_ascii_punctuation() && c != '\''))
        .for_each(|w| {
            let pattern = ['\"', '\''];
            let mut ww = w;
            if ww.starts_with(&pattern[..]) {                                
                ww = ww.strip_prefix(&pattern[..]).unwrap();
            }
            if ww.ends_with(&pattern[..]) {
                ww = ww.strip_suffix(&pattern[..]).unwrap();
            }
            if ww.is_empty() {
                return;
            }
            if let Some(x) = res.get_mut(ww) {
                *x += 1;
            }
            else {
                res.insert(ww.to_string(), 1);
            }
        });
    res
}
