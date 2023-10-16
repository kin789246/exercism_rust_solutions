pub fn factors(mut n: u64) -> Vec<u64> {
    //unimplemented!("This should calculate the prime factors of {n}")
    let mut result: Vec<u64> = Vec::new();
    let mut i = 2..;
    while n > 1 {
        let x = i.next().unwrap();
        while n % x == 0 {
            result.push(x);
            n /= x;
        }
    }
    result
}