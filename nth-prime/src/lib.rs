pub fn nth(n: u32) -> u32 {
    //unimplemented!("What is the 0-indexed {n}th prime number?")
	let mut i: u32 = 2;
	let mut nn = n+1;
	while nn > 0
	{
		// each time if a prime number found decrease n
		if is_prime(i) {
            nn -= 1;
        }
		
		i += 1; //increase the integer to go ahead
	}
	// since decrement of k is being done before
    // Increment of i , so i should be decreased by 1
	i-1
}

fn is_prime(k: u32) -> bool {
    match k {
	    // Corner cases
	    x if x <= 1 => false,
	    2 | 3 => true,
	    
        // below 5 there is only two prime numbers 2 and 3
	    x if x % 2 == 0 || x % 3 == 0 => false,
        _ => {
            // Using concept of prime number can be represented in form of (6*k + 1) or(6*k - 1)
            let end = (k as f64).sqrt() as usize;
	        for i in (5..=end).step_by(6) {
                if k % i as u32 == 0 || k % (i+2) as u32 == 0 {
                    return false
                }
            }
	        true
        }
    }
}