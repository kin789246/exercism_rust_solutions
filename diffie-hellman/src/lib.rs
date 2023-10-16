use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    //unimplemented!("Pick a private key greater than 1 and less than {p}")
    let mut rng = rand::thread_rng();
    rng.gen_range(2..p)
}

fn modular_pow(mut base: u128, mut exponent: u128, modulus: u128) -> u64 {
    let mut result:u128 = 1;
    base %= modulus;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        exponent = exponent >> 1;
        base = ((base%modulus) * (base%modulus)) % modulus;
    }
    let result:u64 = result.try_into().expect("overflow");
    result
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    //unimplemented!("Calculate public key using prime numbers {p} and {g}, and private key {a}")
    //A = g**a mod p
    if p == 1 {
        return 0
    }
    modular_pow(g as u128, a as u128, p as u128)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    // unimplemented!(
    //     "Calculate secret key using prime number {p}, public key {b_pub}, and private key {a}"
    // )
    //s = b_pub**a mod p
    if p == 1 {
        return 0
    }
    modular_pow(b_pub as u128, a as u128, p as u128)
}
