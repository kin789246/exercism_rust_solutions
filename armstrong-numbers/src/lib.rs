pub fn is_armstrong_number(num: u32) -> bool {
    let mut num_vec = Vec::new();
    let mut n: u64 = num as u64;
    loop {
        num_vec.push(n%10);
        n /= 10;
        if n == 0 {
            break;
        }
    }
    let digit: u32 = num_vec.len() as u32;
    let sum: u64 = num_vec.into_iter()
        .map(|x| x.pow(digit)).sum();
    if sum == num as u64 {
        return true
    }
    false
}