use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut r: BTreeMap<char, i32> = BTreeMap::new();
    for (k, v) in h.iter() {
        for c in v.iter() {
            let lc: char = *c.to_lowercase().collect::<Vec<char>>().first().unwrap();
            r.insert(lc, *k);
        }
    }
    r
}
