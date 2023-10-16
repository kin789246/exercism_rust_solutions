fn main() {
    //println!("{:?}", affine_cipher::gcd(99, 63));
    let _= affine_cipher::encode("abc", 5, 7);
    println!("{:?}", affine_cipher::decode("tytgnfjr", 3, 7));
}
