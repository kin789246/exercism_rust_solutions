pub fn square(s: u32) -> u64 {
    //unimplemented!("grains of rice on square {s}");
    if let (1..=64) = s {
        return 2_u64.pow(s-1)
    }
    else {
        panic!("Square must be between 1 and 64");
    }
}

pub fn total() -> u64 {
    //unimplemented!();
    let mut sum = 0;
    for i in 1..=64 {
        sum += square(i);
    }
    sum
}
