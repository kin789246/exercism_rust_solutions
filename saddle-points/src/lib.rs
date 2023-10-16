pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    // switch column and row
    let n = input[0].len();
    let mut columns: Vec<Vec<u64>> = vec![vec![]; n];
    for v in input.iter() {
        for (c, x) in v.iter().enumerate() {
            columns[c].push(*x);
        }
    }
    let mut ans: Vec<(usize, usize)> = Vec::new();
    for (r, v) in input.iter().enumerate() {
        if let Some(max) = (*v).iter().max() {
            for (c, x) in v.iter().enumerate() {
                if *x == *max { 
                    if *max == *columns[c].iter().min().unwrap() {
                        ans.push((r, c));
                    }
                }
            }
        }
    }
    ans
}
