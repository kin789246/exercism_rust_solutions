use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut res: HashSet<[u32; 3]> = HashSet::new();
    for a in 1..sum+1 {
        for b in (a+1)..(sum-a+1) {
            let c = sum - a - b;
            if c < a || c < b || a*a + b*b != c*c {
                continue;
            }
            res.insert([a, b, c]);
        }
    }
    res
}
