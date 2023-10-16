pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound == 1 { return vec![] }
    if upper_bound == 2 { return vec![2] }
    let mut primes: Vec<bool> = vec![true; (upper_bound+1) as usize];
    primes[0] = false;
    primes[1] = false;
    let u = (upper_bound as f64).sqrt() as usize; 
    for i in 2..=u {
        if primes[i] == true {
            for j in (i*i..=upper_bound as usize).step_by(i) {
                primes[j] = false;
            }
        }
    }
    let mut ans: Vec<u64> = Vec::new();
    primes
        .iter()
        .enumerate()
        .for_each(|(i,x)| {
            if *x == true {
                ans.push(i as u64);
            }
        });
    ans
}
