pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    //unimplemented!("Sum the multiples of all of {factors:?} which are less than {limit}")
    use std::collections::HashSet;
    let mut uniques: HashSet<u32> = HashSet::new();

    for x in factors {
        if *x == 0 {
            continue;
        }
        let mut n = limit / x;
        if limit % x == 0 {
            n -= 1;
        }
        for i in 1..=n {
            uniques.insert(i*x);
        }
    }
    uniques.iter().sum()
}
